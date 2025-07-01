#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
    use ink::storage::traits::StorageLayout;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order to add new static storage to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on the contract in order to flip the current value.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool` storage item.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within the same file as the module they're testing.
    /// It's a good practice to set up a separate configuration for these tests, so they're only compiled when the `test`
    /// feature is active.
    ///
    /// The code below is only compiled when the `test` feature is enabled.
    #[cfg(test)]
    mod tests {
        /// Imports all the items from the outer scope.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }

    /// This is how you'd write end-to-end (E2E) tests for your smart contract.
    ///
    /// These tests are designed to simulate the interaction between a smart contract and a real
    /// blockchain environment. They operate by deploying the contract onto a Substrate blockchain.
    ///
    /// The code below is only compiled when the `e2e-tests` feature is enabled.
    /// `cargo test --features e2e-tests`
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Instantiate our contract.
            let constructor = FlipperRef::new(false);
            let contract_acc_id = client
                .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Get the initial value.
            let get = build_message::<FlipperRef>(contract_acc_id.clone())
                .call(|flipper| flipper.get());
            let get_res = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_res.return_value(), false));

            // Flip the value.
            let flip = build_message::<FlipperRef>(contract_acc_id.clone())
                .call(|flipper| flipper.flip());
            let _flip_res = client
                .call(&ink_e2e::alice(), flip, 0, None)
                .await
                .expect("flip failed");

            // Get the new value.
            let get = build_message::<FlipperRef>(contract_acc_id.clone())
                .call(|flipper| flipper.get());
            let get_res = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_res.return_value(), true));

            Ok(())
        }

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Instantiate our contract.
            let constructor = FlipperRef::default();
            let contract_acc_id = client
                .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Get the initial value.
            let get = build_message::<FlipperRef>(contract_acc_id.clone())
                .call(|flipper| flipper.get());
            let get_res = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_res.return_value(), false));

            Ok(())
        }
    }
}