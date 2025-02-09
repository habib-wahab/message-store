#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_message)]
    pub(super) type Messages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u8, ConstU32<1024>>,
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageStored { account: T::AccountId, message: BoundedVec<u8, ConstU32<1024>> },
    }

    #[pallet::error]
    pub enum Error<T> {
        MessageTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn store_message(
            origin: OriginFor<T>,
            message: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            let bounded_message = BoundedVec::<u8, ConstU32<1024>>::try_from(message)
                .map_err(|_| Error::<T>::MessageTooLong)?;
            
            Messages::<T>::insert(&sender, bounded_message.clone());
            
            Self::deposit_event(Event::MessageStored { 
                account: sender,
                message: bounded_message,
            });
            
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_message_store;
    use frame_support::{assert_ok, parameter_types};
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            MessageStore: pallet_message_store,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const SS58Prefix: u8 = 42;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeEvent = RuntimeEvent;
        type RuntimeCall = RuntimeCall;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = SS58Prefix;
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
        type Nonce = u64;
    }

    impl pallet::Config for Test {
        type RuntimeEvent = RuntimeEvent;
    }

    pub fn new_test_ext() -> sp_io::TestExternalities {
        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    #[test]
    fn store_message_works() {
        new_test_ext().execute_with(|| {
            let message = b"Hello, World!".to_vec();
            assert_ok!(MessageStore::store_message(RuntimeOrigin::signed(1), message.clone()));
            
            let stored_message: Vec<u8> = MessageStore::get_message(1).into();
            assert_eq!(stored_message, message);
        });
    }
}
