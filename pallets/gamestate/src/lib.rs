#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{*, ValueQuery};
	use frame_support::traits::{Currency, Randomness};
    use frame_system::pallet_prelude::*;
	type Uuid = u32;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) { 
		}
	}

    #[pallet::storage]
    pub type Sessions<T: Config> = StorageMap<_, Twox64Concat, Uuid, Session<T>, OptionQuery>;
    
	#[pallet::storage]
	pub type Participants<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Uuid, ValueQuery>;

	#[pallet::storage]
    pub type UuidGenerationCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SessionStarted { who: T::AccountId, session_id: Uuid },
	}

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	
		#[pallet::constant]
		type MaxPlayerPerSession: Get<u32>;
    }

	#[pallet::error]
	pub enum Error<T> {
		GenericError,
		AlreadyInActiveSession
	}

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
    pub struct Session<T:Config> {
        pub current_actor_index: u8,
		pub actors: T::AccountId//BoundedVec<T::AccountId, T::MaxPlayerPerSession>,
    }
	
	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn start_session(origin: OriginFor<T>) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			ensure!(Participants::<T>::contains_key(&sender) == false, Error::<T>::AlreadyInActiveSession);

			let new_uuid = Self::generate_uuid();
			Participants::<T>::insert(&sender, new_uuid);

			Sessions::<T>::insert(new_uuid, Session {
				current_actor_index: 0,
				actors: sender.clone() //frame_support::bounded_vec!(sender)
			});

			Self::deposit_event(Event::SessionStarted{ who: sender, session_id: new_uuid});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn end_session(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn enter_plot(origin: OriginFor<T>) -> DispatchResult {
			
			let sender = ensure_signed(origin)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn generate_uuid() -> Uuid {
			let new_id = UuidGenerationCount::<T>::get();
			UuidGenerationCount::<T>::put(new_id + 1);

			new_id
		}
	}
}

