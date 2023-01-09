#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Currency;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
	}

	// #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]

	#[pallet::storage]
	pub(super) type RewardAmount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// setting storage value failed
		RewardAmountNotSet,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New reward amount set
		RewardAmountSet { value: u64 },
	}

	// // Pallet internal functions
	// impl<T: Config> Pallet<T> {
	// 	pub fn set_reward(new_reward: u64) -> Result<u64, DispatchError> {
	// 		RewardAmount::<T>::put(new_reward);
	// 		Ok(new_reward)
	// 	}
	// }

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_reward(origin: OriginFor<T>, new_reward: u64) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			RewardAmount::<T>::put(new_reward);
			Self::deposit_event(Event::RewardAmountSet { value: new_reward });
			Ok(())
		}
	}
}
