#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod default_weights;

pub use crate::{default_weights::WeightInfo, pallet::*};

#[frame_support::pallet]
pub mod pallet {
	use super::WeightInfo;
	use frame_support::traits::FindAuthor;
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	pub type NegativeImbalanceOf<T> =
		<<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
		type FindAuthor: FindAuthor<Self::AccountId>;
		type WeightInfo: WeightInfo;
	}

	// #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]

	#[pallet::storage]
	pub(super) type RewardAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// setting storage value failed
		RewardAmountNotSet,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New reward amount set
		RewardAmountSet { value: BalanceOf<T> },
		/// Mining reward given to author
		MiningRewardForAuthor {
			value: BalanceOf<T>,
			account: Option<<T as frame_system::Config>::AccountId>,
		},
	}

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_reward(origin: OriginFor<T>, new_reward: BalanceOf<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			RewardAmount::<T>::put(new_reward);
			Self::deposit_event(Event::RewardAmountSet { value: new_reward });
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_: T::BlockNumber) -> Weight {
			let reward = T::Currency::issue(RewardAmount::<T>::get());
			// T::Beneficiary::on_unbalanced(reward);

			let block_digest = <frame_system::Pallet<T>>::digest();
			let digests = block_digest.logs.iter().filter_map(|d| d.as_pre_runtime());
			let author = T::FindAuthor::find_author(digests);
			let author_copy = author.clone();

			T::Currency::resolve_creating(&author.unwrap(), reward);
			Self::deposit_event(Event::MiningRewardForAuthor {
				value: RewardAmount::<T>::get(),
				account: author_copy,
			});

			<T as Config>::WeightInfo::on_initialize_mint_to_treasury()
		}
	}
}
