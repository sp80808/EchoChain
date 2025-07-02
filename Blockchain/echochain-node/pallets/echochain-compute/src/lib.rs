#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use sp_runtime::traits::{CheckedAdd, Saturating};
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::vec::Vec;
    use sp_std::prelude::*;
    use frame_support::log;
    use sp_runtime::offchain::{self as rt_offchain, Duration};
    use sp_runtime::transaction_validity::{
        InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
    };

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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

        /// Storage for worker loads.
        #[pallet::storage]
        #[pallet::getter(fn worker_load)]
        pub type WorkerLoad<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

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
        /// Request for external data initiated [task_id, endpoint]
        ExternalDataRequested(u32, Vec<u8>),
        /// External data submitted [task_id]
        ExternalDataSubmitted(u32),
    }

    /// Storage for compute tasks
    #[pallet::storage]
    #[pallet::getter(fn compute_tasks)]
    pub type ComputeTasks<T> = StorageMap<_, Blake2_128Concat, u32, TaskInfo<T::AccountId, T::Hash, T::BlockNumber>, OptionQuery>;

    /// Storage for task results
    #[pallet::storage]
    #[pallet::getter(fn task_results)]
    pub type TaskResults<T> = StorageMap<_, Blake2_128Concat, u32, ResultInfo<T::AccountId, T::Hash>, OptionQuery>;

    /// Storage for verification receipts
    #[pallet::storage]
    #[pallet::getter(fn verification_receipts)]
    pub type VerificationReceipts<T> = StorageMap<_, Blake2_128Concat, u32, VerificationReceipt<T::AccountId, T::Hash>, OptionQuery>;

    /// Task information structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct TaskInfo<AccountId, Hash, BlockNumber> {
        creator: AccountId,
        task_hash: Hash,
        data: Option<Vec<u8>>,
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

    /// Verification receipt structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct VerificationReceipt<AccountId, Hash> {
        verifier: AccountId,
        result_hash: Hash,
        signature: Vec<u8>,
    }

    /// Verification error enum
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum VerificationError {
        SignatureMismatch,
        ResultMismatch,
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
                data: None,
                assigned_worker: None,
                status: TaskStatus::Created,
                creation_block: <frame_system::Pallet<T>>::block_number(),
            };
            ComputeTasks::<T>::insert(task_id, task_info);
            Self::deposit_event(Event::ComputeTaskCreated(task_id));
            Ok(())
        }

        /// Assign a compute task to a worker
        #[pallet::weight(5_000)]
        pub fn assign_task(
            origin: OriginFor<T>,
            task_id: u32,
            available_workers: Vec<T::AccountId>
        ) -> DispatchResult {
            let _promoter = ensure_signed(origin)?;

            ensure!(!available_workers.is_empty(), Error::<T>::NoAvailableWorkers);

            let mut task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.status == TaskStatus::Created && task.assigned_worker.is_none(),
                Error::<T>::TaskAlreadyAssigned
            );

            let worker = match TaskDistributionAlgorithm::<T>::get() {
                TaskDistribution::WeightedRoundRobin => {
                    // Simple Round-Robin implementation
                    let next_worker_account = NextWorker::<T>::get();
                    let current_index = available_workers.iter().position(|w| *w == next_worker_account).unwrap_or(0);
                    let selected_worker = available_workers[current_index].clone();
                    let next_index = (current_index + 1) % available_workers.len();
                    NextWorker::<T>::put(available_workers[next_index].clone());
                    Ok(selected_worker)
                }
                TaskDistribution::LeastLoaded => {
                    // Find the worker with the minimum load
                    available_workers
                        .into_iter()
                        .min_by_key(|w| WorkerLoad::<T>::get(w))
                        .ok_or(Error::<T>::NoAvailableWorkers.into())
                }
            }?;

            task.assigned_worker = Some(worker.clone());
            task.status = TaskStatus::Assigned;
            ComputeTasks::<T>::insert(task_id, task);

            // Increment the worker's load
            WorkerLoad::<T>::mutate(&worker, |load| *load = load.saturating_add(1));

            Self::deposit_event(Event::ComputeTaskAssigned(task_id, worker));

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

            // Decrement the worker's load
            WorkerLoad::<T>::mutate(&worker, |load| *load = load.saturating_sub(1));

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

                                // Decrement the timed-out worker's load and increment the new worker's load
                                WorkerLoad::<T>::mutate(current_worker, |load| *load = load.saturating_sub(1));
                                WorkerLoad::<T>::mutate(&new_worker, |load| *load = load.saturating_add(1));

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

        /// Request data from an external source for a compute task
        #[pallet::weight(5_000)]
        pub fn request_external_data(
            origin: OriginFor<T>,
            task_id: u32,
            endpoint: Vec<u8>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.status == TaskStatus::Created && task.creator == who,
                Error::<T>::UnauthorizedWorker
            );

            // Emit an event to signal the need for external data retrieval
            // An off-chain worker or oracle service should listen for this event and fetch the data
            Self::deposit_event(Event::ExternalDataRequested(task_id, endpoint.clone()));
            
            // Logic for triggering off-chain worker integration
            // In a full implementation, this would:
            // 1. Create a unique request ID for tracking the external data request
            // 2. Store the request details in a storage map for off-chain workers to access
            // 3. Trigger an off-chain worker task to fetch data from the specified endpoint
            // 4. Implement a callback mechanism (via another extrinsic) for the worker to submit data back to the pallet
            // For now, the event emission serves as the trigger point for off-chain workers.
            // Future development will include:
            // - Integration with a service like Chainlink or Acurast for secure and reliable data retrieval
            // - Storage for tracking pending external data requests and their statuses
            // - Timeout mechanisms to handle cases where data retrieval fails or takes too long
            Ok(())
        }

        /// Submit external data for a compute task
        #[pallet::weight(5_000)]
        pub fn submit_external_data(
            origin: OriginFor<T>,
            task_id: u32,
            data: Vec<u8>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let mut task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.creator == who,
                Error::<T>::UnauthorizedWorker
            );

            task.data = Some(data);
            ComputeTasks::<T>::insert(task_id, task);
            Self::deposit_event(Event::ExternalDataSubmitted(task_id));
            Ok(())
        }

        /// Verify the result of a compute task using consensus
        #[pallet::weight(10_000)]
        pub fn verify_result(
            origin: OriginFor<T>,
            task_id: u32,
            result_hash: T::Hash,
            signature: Vec<u8>
        ) -> DispatchResult {
            let verifier = ensure_signed(origin)?;

            let task = ComputeTasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            ensure!(
                task.status == TaskStatus::Completed,
                Error::<T>::TaskNotCompleted
            );

            let result = TaskResults::<T>::get(task_id).ok_or(Error::<T>::ResultNotFound)?;

            if result.result_hash != result_hash {
                return Err(Error::<T>::VerificationFailed.into());
            }

            let receipt = VerificationReceipt {
                verifier: verifier.clone(),
                result_hash,
                signature,
            };

            VerificationReceipts::<T>::insert(task_id, receipt);

            let mut updated_task = task;
            updated_task.status = TaskStatus::Verified;
            ComputeTasks::<T>::insert(task_id, updated_task);

            Self::deposit_event(Event::ComputeResultVerified(task_id, verifier));

            Ok(())
        }
        
                /// Function to perform data processing (placeholder for off-chain integration)
        pub fn process_data(data: &Vec<u8>) -> Vec<u8> {
            // This function implements a basic data processing logic as a starting point.
            // In a production environment, direct HTTP requests are not feasible due to no_std constraints.
            // Instead, data retrieval from external sources should be handled by off-chain workers or oracles.
            // The processed data would then be submitted to the pallet for on-chain verification and storage.
            // Future development will include:
            // - Integration with off-chain workers or oracle services (e.g., Chainlink, Acurast) for external data retrieval.
            // - Mechanisms to process data securely and efficiently before returning results.
            // For now, implement a simple transformation by reversing the input data bytes as a basic processing example.
            let mut processed = data.clone();
            processed.reverse();
            processed
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
            /// Network request failed
            NetworkRequestFailed,
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

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::submit_external_data { task_id, .. } = call {
                ValidTransaction::with_tag_prefix("EchochainCompute")
                    .and_provides(task_id)
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            for event in frame_system::Pallet::<T>::events() {
                if let Event::ExternalDataRequested(task_id, endpoint) = event.event {
                    if let Err(e) = Self::fetch_and_log_external_data(task_id, &endpoint) {
                        log::error!("Error fetching data for task {}: {:?}", task_id, e);
                    }
                }
            }
        }
    }

    impl<T: Config> Pallet<T> {
        fn fetch_and_log_external_data(task_id: u32, endpoint: &[u8]) -> Result<(), Error<T>> {
            let endpoint_str = sp_std::str::from_utf8(endpoint).map_err(|_| Error::<T>::NetworkRequestFailed)?;
            log::info!("Fetching data for task {} from endpoint: {}", task_id, endpoint_str);

            let response = reqwest::blocking::get(endpoint_str).map_err(|_| Error::<T>::NetworkRequestFailed)?;
            let data = response.bytes().map_err(|_| Error::<T>::NetworkRequestFailed)?.to_vec();

            let call = Call::submit_external_data { task_id, data };
            let _ = rt_offchain::submit_unsigned_transaction(call.into()).map_err(|_| Error::<T>::NetworkRequestFailed);

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

            // Ensure task does not already exist
            ensure!(
                !ComputeTasks::<T>::contains_key(task_id),
                Error::<T>::TaskAlreadyExists
            );

            // Perform data processing
            let _processed_data = Self::process_data(&data);

            // Create and store the task
            let task_info = TaskInfo {
                creator: who.clone(),
                task_hash: T::Hashing::hash_of(&data),
                data: Some(data),
                assigned_worker: None,
                status: TaskStatus::Created,
                creation_block: <frame_system::Pallet<T>>::block_number(),
            };
            ComputeTasks::<T>::insert(task_id, task_info);
            Pallet::<T>::deposit_event(Event::ComputeTaskCreated(task_id));
            Ok(())
        }
    }
}
