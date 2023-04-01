#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, Randomness};
    use frame_system::pallet_prelude::*;

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

    #[pallet::config]
    pub trait Config: frame_system::Config {

		type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	
		#[pallet::constant]
		type MaximumOwned: Get<u32>;
    }

	#[pallet::error]
	pub enum Error<T> {
		GenericError,
	}

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn register_account(origin: OriginFor<T>) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn request_new_plant(origin: OriginFor<T>) -> DispatchResult {
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


}