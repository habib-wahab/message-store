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
