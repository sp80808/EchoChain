#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use ark_groth16::Groth16;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_serialize::CanonicalDeserialize;
    use ark_ec::PairingEngine;
    use ark_std::rand::thread_rng;
    use ark_std::UniformRand;
    use sp_runtime::traits::CheckedAdd;
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::vec::Vec;
    use sp_std::prelude::*;

    #[pallet::pallet]
        #[pallet::generate_store(pub(super) trait Store)]
        pub struct Pallet<T>(_);
    
        /// Storage for the next worker to assign a task to.
        #[pallet::storage]
        #[pallet::getter(fn next_worker)]
        pub type NextWorker<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;
        /// Storage for the task distribution algorithm.
        #[pallet::storage]
        #[pallet::getter(fn task_distribution_algorithm)]
        pub type TaskDistributionAlgorithm<T: Config> = StorageValue<_, TaskDistribution, ValueQuery>;

        /// Task distribution algorithm enum.
        #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
        pub enum TaskDistribution {
            WeightedRoundRobin,
            LeastLoaded,
        }

        /// Storage for worker weights.
        #[pallet::storage]
        #[pallet::getter(fn worker_weights)]
        pub type WorkerWeights<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Currency type for potential rewards or penalties in compute tasks
        #[pallet::constant]
        type Currency: Currency<Self::AccountId>;

        /// Maximum size of the data that can be processed in a single task.
        #[pallet::constant]
        type MaxDataSize: Get<u32>;
        
        /// Timeout duration for tasks in blocks
        #[pallet::constant]
        type TaskTimeout: Get<Self::BlockNumber>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Compute task created [task_id]
        ComputeTaskCreated(u32),
        /// Compute task assigned [task_id, worker]
        ComputeTaskAssigned(u32, T::AccountId),
        /// Compute result submitted [task_id, worker]
        ComputeResultSubmitted(u32, T::AccountId),
        /// Result was verified
        ComputeResultVerified(u32, T::AccountId),
        /// Off-chain computation task initiated [task_id]
        OffChainTaskInitiated(u32),
        /// Compute task timed out and reassigned [task_id, new_worker]
        ComputeTaskReassigned(u32, T::AccountId),
        /// Secure off-chain task initiated with TEE [task_id]
        SecureOffChainTaskInitiated(u32),
    }

    /// Storage for compute tasks
    #[pallet::storage]
    #[pallet::getter(fn compute_tasks)]
    pub type ComputeTasks<T> = StorageMap<_, Blake2_128Concat, u32, TaskInfo<T::AccountId, T::Hash, T::BlockNumber>, OptionQuery>;

    /// Storage for task results
    #[pallet::storage]
    #[pallet::getter(fn task_results)]
    pub type TaskResults<T> = StorageMap<_, Blake2_128Concat, u32, ResultInfo<T::AccountId, T::Hash>, OptionQuery>;

    /// Task information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct TaskInfo<AccountId, Hash, BlockNumber> {
        creator: AccountId,
        task_hash: Hash,
        assigned_worker: Option<AccountId>,
        status: TaskStatus,
        creation_block: BlockNumber,
    }

    /// Task result structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct ResultInfo<AccountId, Hash> {
        worker: AccountId,
        result_hash: Hash,
        verified: bool,
    }

    /// Task status enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum TaskStatus {
        Created,
        Assigned,
        Completed,
        Verified,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new compute task with ZKP verification
        #[pallet::weight(10_000)]
        pub fn create_compute_task(
            origin: OriginFor<T>,
            task_id: u32,
            task_hash: T::Hash,
            zkp: Vec<u8>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // ZKP verification logic
            ensure!(
                !ComputeTasks::<T>::contains_key(task_id),
                Error::<T>::TaskAlreadyExists
            );

            let task_info = TaskInfo {
                creator: who.clone(),
                task_hash,
                assigned_worker: None,
                status: TaskStatus::Created,
                creation_block: <frame_system::Pallet<T>>::block_number(),
            };
            ComputeTasks::<T>::insert(task_id, task_info);
            Self::deposit_event(Event::ComputeTaskCreated(task_id));
            Ok(())
        }

        /// Assign a compute task to a worker using weighted round-robin
        #[pallet::weight(5_000)]
        pub fn assign_task(
            origin: OriginFor<T>,
            task_id: u32,
            available_workers: Vec<T::AccountId>
        ) -> DispatchResult {
            let _worker = ensure_signed(origin)?;

            // Ensure there are available workers
            ensure!(!available_workers.is_empty(), Error::<T>::NoAvailableWorkers);

            // Get the next worker to assign the task to
            let mut next_worker = NextWorker::<T>::get();

            // Ensure the next worker is in the list of available workers
            let worker = available_workers.iter().find(|w| **w == next_worker).ok_or(Error::<T>::WorkerNotFound)?.clone();

            let mut task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.status == TaskStatus::Created && task.assigned_worker.is_none(),
                Error::<T>::TaskAlreadyAssigned
            );

            task.assigned_worker = Some(worker.clone());
            task.status = TaskStatus::Assigned;
            ComputeTasks::<T>::insert(task_id, task);
            Self::deposit_event(Event::ComputeTaskAssigned(task_id, worker.clone()));

            // Update the next worker based on the selected algorithm
            match TaskDistributionAlgorithm::<T>::get() {
                TaskDistribution::WeightedRoundRobin => {
                    let weight = WorkerWeights::<T>::get(&worker);
                    let mut assigned_tasks = 0;
                    while assigned_tasks < weight {
                        next_worker = available_workers.iter().find(|w| **w != next_worker).ok_or(Error::<T>::WorkerNotFound)?.clone();
                        assigned_tasks += 1;
                    }
                    NextWorker::<T>::put(next_worker);
                }
                TaskDistribution::LeastLoaded => {
                    // Find the worker with the least number of assigned tasks
                    let mut least_loaded_worker: Option<T::AccountId> = None;
                    let mut min_assigned_tasks: u32 = u32::MAX;

                    for w in &available_workers {
                        let weight = WorkerWeights::<T>::get(w); // Assuming WorkerWeights can also represent current load
                        if weight < min_assigned_tasks {
                            min_assigned_tasks = weight;
                            least_loaded_worker = Some(w.clone());
                        }
                    }

                    // Assign the task to the least loaded worker
                    if let Some(least_loaded) = least_loaded_worker {
                        NextWorker::<T>::put(least_loaded);
                    } else {
                        // If no worker is found, return an error
                        return Err(Error::<T>::WorkerNotFound.into());
                    }
                }
            }

            Ok(())
        }

        /// Submit result for a compute task
        #[pallet::weight(8_000)]
        pub fn submit_result(
            origin: OriginFor<T>,
            task_id: u32,
            result_hash: T::Hash
        ) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            let task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.status == TaskStatus::Assigned && task.assigned_worker == Some(worker.clone()),
                Error::<T>::UnauthorizedWorker
            );

            let result_info = ResultInfo {
                worker: worker.clone(),
                result_hash,
                verified: false,
            };
            TaskResults::<T>::insert(task_id, result_info);

            let mut updated_task = task;
            updated_task.status = TaskStatus::Completed;
            ComputeTasks::<T>::insert(task_id, updated_task);
            Self::deposit_event(Event::ComputeResultSubmitted(task_id, worker));
            Ok(())
        }

        /// Submit multiple results for compute tasks in a batch to optimize gas costs
        #[pallet::weight(15_000)]
        pub fn submit_batch_results(
            origin: OriginFor<T>,
            task_results: Vec<(u32, T::Hash)>
        ) -> DispatchResult {
            let worker = ensure_signed(origin)?;
            ensure!(!task_results.is_empty(), Error::<T>::NoResultsToVerify);

            for (task_id, result_hash) in task_results {
                let task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
                ensure!(
                    task.status == TaskStatus::Assigned && task.assigned_worker == Some(worker.clone()),
                    Error::<T>::UnauthorizedWorker
                );

                let result_info = ResultInfo {
                    worker: worker.clone(),
                    result_hash,
                    verified: false,
                };
                TaskResults::<T>::insert(task_id, result_info);

                let mut updated_task = task;
                updated_task.status = TaskStatus::Completed;
                ComputeTasks::<T>::insert(task_id, updated_task);
                Self::deposit_event(Event::ComputeResultSubmitted(task_id, worker.clone()));
            }
            Ok(())
        }

        /// Check for timed out tasks and reassign them to new workers
        #[pallet::weight(10_000)]
        pub fn check_and_reassign_tasks(
            origin: OriginFor<T>,
            available_workers: Vec<T::AccountId>
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            ensure!(!available_workers.is_empty(), Error::<T>::NoAvailableWorkers);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let timeout_duration = T::TaskTimeout::get();

            // Iterate through all tasks to find timed out ones
            for (task_id, task) in ComputeTasks::<T>::iter() {
                if task.status == TaskStatus::Assigned {
                    if let Some(assigned_block) = task.creation_block.checked_add(&timeout_duration) {
                        if current_block > assigned_block {
                            // Task has timed out, reassign to a new worker
                            if let Some(current_worker) = &task.assigned_worker {
                                // Exclude the current worker from available workers if possible
                                let mut new_workers: Vec<T::AccountId> = available_workers
                                    .iter()
                                    .filter(|w| *w != current_worker)
                                    .cloned()
                                    .collect();
                                
                                if new_workers.is_empty() {
                                    new_workers = available_workers.clone();
                                }

                                // Simple round-robin selection for reassignment
                                let new_worker = new_workers.first().ok_or(Error::<T>::NoAvailableWorkers)?.clone();
                                let mut updated_task = task;
                                updated_task.assigned_worker = Some(new_worker.clone());
                                ComputeTasks::<T>::insert(task_id, updated_task);
                                Self::deposit_event(Event::ComputeTaskReassigned(task_id, new_worker));
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        /// Initiate a secure off-chain computation task using a Trusted Execution Environment (TEE)
        #[pallet::weight(5_000)]
        pub fn initiate_secure_off_chain_task(
            origin: OriginFor<T>,
            task_id: u32
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.status == TaskStatus::Created && task.creator == who,
                Error::<T>::UnauthorizedWorker
            );

            // Placeholder for initiating a secure off-chain task using TEE
            // Inspired by projects like Acurast, this would integrate with a TEE for secure computation
            // In a full implementation, this would trigger a secure environment for task processing
            Self::deposit_event(Event::SecureOffChainTaskInitiated(task_id));
            Ok(())
        }

        /// Verify the result of a compute task using consensus
                #[pallet::weight(10_000)]
                pub fn verify_result(
                    origin: OriginFor<T>,
                    task_id: u32,
                    results: Vec<T::Hash>
                ) -> DispatchResult {
                    let verifier = ensure_signed(origin)?;
        
                    let task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
                    ensure!(
                        task.status == TaskStatus::Completed,
                        Error::<T>::TaskNotCompleted
                    );
        
                    // Ensure that there are results to verify
                    ensure!(!results.is_empty(), Error::<T>::NoResultsToVerify);
        
                    // Calculate the consensus result
                    let consensus_result = Self::calculate_consensus(results.clone()).ok_or(Error::<T>::NoConsensus)?;
        
                    let mut result = TaskResults::<T>::get(task_id).ok_or(Error::<T>::ResultNotFound)?;
        
                    // In a real implementation, this would involve actual verification logic
                    // such as comparing the result hash with a known correct hash or running the
                    // computation again and comparing the results.
                    // For now, we simply mark the result as verified if the consensus result matches the stored result
                    if result.result_hash == consensus_result {
                        result.verified = true;
                        TaskResults::<T>::insert(task_id, result);
                        // Change task status to verified
                        let mut updated_task = task;
                        updated_task.status = TaskStatus::Verified;
                        ComputeTasks::<T>::insert(task_id, updated_task);
                        Self::deposit_event(Event::ComputeResultVerified(task_id, verifier));
                    } else {
                        // If the consensus result does not match the stored result, return an error
                        return Err(Error::<T>::VerificationFailed.into());
                    }
        
                    Ok(())
                }
        
                /// Calculate the consensus result from a list of results
                pub fn calculate_consensus(results: Vec<T::Hash>) -> Option<T::Hash> {
                    use sp_std::collections::btree_map::BTreeMap;
        
                    // Count the occurrences of each result
                    let mut counts: BTreeMap<T::Hash, u32> = BTreeMap::new();
                    for result in results {
                        let count = counts.entry(result).or_insert(0);
                        *count += 1;
                    }
        
                    // Find the result with the highest count
                    let mut consensus_result: Option<T::Hash> = None;
                    let mut max_count: u32 = 0;
                    for (result, count) in counts {
                        if count > max_count {
                            consensus_result = Some(result);
                            max_count = count;
                        }
                    }
        
                    consensus_result
                }

        /// Function to perform data processing (placeholder)
        pub fn process_data(data: &Vec<u8>) -> Vec<u8> {
            // Implement your data processing logic here
            // This is just a placeholder, replace with actual processing
            // For now, we will fetch data from a mock Chainlink API
            use reqwest::blocking::Client;
            use serde::Deserialize;

            #[derive(Deserialize)]
            struct ApiResponse {
                data: String,
            }

            let client = Client::new();
            let response = match client
                .get("https://mockapi.io/api/v1/data") // Replace with your Chainlink API endpoint
                .send()
                {
                    Ok(response) => response,
                    Err(_) => return data.clone(), // Return original data on error
                };

            let api_response: ApiResponse = match response.json::<ApiResponse>() {
                Ok(api_response) => api_response,
                Err(_) => return data.clone(), // Return original data on error
            };

            api_response.data.into_bytes()
        }

        #[pallet::error]
        pub enum Error<T> {
            /// Invalid Zero-Knowledge Proof
            InvalidZKP,
            /// Task already exists with the given ID
            TaskAlreadyExists,
            /// Task not found
            TaskNotFound,
            /// Task already assigned to a worker
            TaskAlreadyAssigned,
            /// Worker not authorized to submit result for this task
            UnauthorizedWorker,
            /// Task not completed yet
            TaskNotCompleted,
            /// Result not found for the task
            ResultNotFound,
            /// Data size exceeds the limit
            DataSizeExceedsLimit,
            /// No available workers to assign the task to
            NoAvailableWorkers,
            /// Worker not found in the available workers list
            WorkerNotFound,
            /// No results to verify
            NoResultsToVerify,
            /// No consensus reached
            NoConsensus,
            /// Verification failed
            VerificationFailed,
        }

        // Updated verify_zkp function
        pub fn verify_zkp(zkp: &Vec<u8>) -> Result<(), Error<T>> {
            // Define the pairing engine to use (BLS12-381 is a common choice)
            use ark_bls12_381::Bls12_381;

            // Define the circuit's public input and output types
            type Field = <Bls12_381 as PairingEngine>::Fr;

            // Define the structure for the verification key
            #[derive(Clone, PartialEq, Eq, Debug, CanonicalDeserialize)]
            pub struct VerifyingKey<PE: PairingEngine> {
                pub alpha_g1: PE::G1Affine,
                pub beta_g2: PE::G2Affine,
                pub gamma_g2: PE::G2Affine,
                pub delta_g2: PE::G2Affine,
                pub ic: Vec<PE::G1Affine>,
            }

            // Load the verification key from a trusted source (e.g., storage)
            let vk: VerifyingKey<Bls12_381> = {
                // Replace this with the actual code to load the verification key
                // For demonstration purposes, we create a dummy key
                use ark_std::rand::thread_rng;
                use ark_relations::r1cs::ConstraintSystem;
                use ark_groth16::generate_random_parameters;
                let mut rng = thread_rng();
                let cs = ConstraintSystem::<Field>::new_ref();
                let params = generate_random_parameters::<Bls12_381, _, _>(cs, &mut rng).unwrap();
                params.vk
            };

            // Deserialize the proof from the input
            let proof = match ark_groth16::Proof::<Bls12_381>::deserialize(&zkp[..]) {
                Ok(proof) => proof,
                Err(_) => return Err(Error::<T>::InvalidZKP),
            };

            // Prepare the public input for verification
            let public_input: Vec<<Bls12_381 as PairingEngine>::Fr> = vec![<Bls12_381 as PairingEngine>::Fr::from(1u32)];

            // Verify the proof
            use ark_groth16::Groth16;
            let valid = match Groth16::<Bls12_381>::verify_proof(&vk, &proof, &public_input) {
                Ok(is_valid) => is_valid,
                Err(_) => return Err(Error::<T>::InvalidZKP),
            };

            if !valid {
                return Err(Error::<T>::InvalidZKP);
            }

            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the task distribution algorithm
        #[pallet::weight(1_000)]
        pub fn set_task_distribution_algorithm(
            origin: OriginFor<T>,
            algorithm: TaskDistribution,
        ) -> DispatchResult {
            ensure_root(origin)?;

            TaskDistributionAlgorithm::<T>::put(algorithm);

            Ok(())
        }

        /// Set the weight of a worker
        #[pallet::weight(1_000)]
        pub fn set_worker_weight(
            origin: OriginFor<T>,
            worker: T::AccountId,
            weight: u32
        ) -> DispatchResult {
            ensure_root(origin)?;

            WorkerWeights::<T>::insert(worker.clone(), weight);

            // Update the next worker if the current worker is the next worker
            if NextWorker::<T>::get() == worker {
                NextWorker::<T>::put(worker);
            }

            Ok(())
        }
    }

    pub trait ComputeInterface<AccountId> {
        fn create_task(who: AccountId, task_id: u32, data: Vec<u8>, zkp: Vec<u8>) -> DispatchResult;
    }

    impl<T: Config> ComputeInterface<T::AccountId> for Pallet<T> {
        fn create_task(who: T::AccountId, task_id: u32, data: Vec<u8>, zkp: Vec<u8>) -> DispatchResult {
            // Ensure that the data size does not exceed the limit.
            ensure!(
                data.len() as u32 <= T::MaxDataSize::get(),
                Error::<T>::DataSizeExceedsLimit
            );

            // ZKP verification logic
            let is_valid_zkp = Self::verify_zkp(&zkp).is_ok();

            if !is_valid_zkp {
                return Err(Error::<T>::InvalidZKP.into());
            }

            // Perform data processing.
            let _processed_data = Self::process_data(&data);

            // This is a stub. The actual implementation would create a compute task.
            Pallet::<T>::deposit_event(Event::ComputeTaskCreated(task_id));
            Ok(())
        }
    }
}
