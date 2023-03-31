#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_defaults)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, Randomness};
    use frame_system::pallet_prelude::*;
	const MAX_PLOTS:u32 = 9;
	const MAX_ID_LEN:u32 = 16;
	type MaxIdLen = ConstU32<MAX_ID_LEN>;
	type MaxPlots = ConstU32<MAX_PLOTS>;
	type Uuid = u32;
	type PlantCollection = BoundedVec<Uuid, MaxIdLen>;
	type PlotCollection<T> = BoundedVec<Option<Cell<T>>,MaxPlots>;


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
			LandPlot::<T>::try_mutate(|land_plot|{

				for _ in 0..MAX_PLOTS {
					land_plot.try_push(None).ok();
				}

				Ok::<(), Error<T>>(())
			}).ok();
		}
	}

    #[pallet::config]
    pub trait Config: frame_system::Config {

		type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	
		#[pallet::constant]
		type MaximumOwned: Get<u32>;
    }
	
	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Plant {
		// Unsigned integers of 16 bytes to represent a unique identifier
		pub unique_id: Uuid,
		pub age: u32,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Land plot is not vacant for entering
		LandplotOccupied,
		GenericError,
		AlreadyRegistered
	}

	#[pallet::storage]
	pub(super) type PlantGenerationCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub(super) type LandPlot<T: Config> = StorageValue<_, PlotCollection<T>, ValueQuery>;

	#[pallet::storage]
	pub type OwnerMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, PlantCollection, ValueQuery>;
	
	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Cell<T: Config> {
		pub occupant: T::AccountId,
		pub plant_id: Uuid,
	}

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn register_account(origin: OriginFor<T>) -> DispatchResult {

			let sender = ensure_signed(origin)?;

			ensure!(!OwnerMap::<T>::contains_key(&sender), Error::<T>::AlreadyRegistered);

			OwnerMap::<T>::insert(sender, PlantCollection::default());

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn request_new_plant(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;
			
			ensure!(OwnerMap::<T>::contains_key(&sender), Error::<T>::GenericError);

			let new_id = PlantGenerationCount::<T>::get();

			OwnerMap::<T>::try_mutate(&sender, |plants| {
				plants.try_push(new_id)
			}).ok();

			PlantGenerationCount::<T>::put(new_id + 1);

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn enter_plot(origin: OriginFor<T>, plot_index:u8, plant_id: Uuid) -> DispatchResult {
			
			let sender = ensure_signed(origin)?;

			ensure!(plot_index < MAX_PLOTS as u8, Error::<T>::GenericError);

			LandPlot::<T>::try_mutate(|cells|{
				let plot_index = plot_index as usize;

				ensure!(cells[plot_index].is_none(), Error::<T>::LandplotOccupied);

				cells[plot_index] = Some(Cell {
					occupant: sender,
					plant_id: plant_id
				});

				Ok(())
			})
		}
	}


}