#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::RawOrigin, pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		app_crypto::UncheckedFrom,
		traits::{AccountIdLookup, StaticLookup},
		AccountId32,
	};
	use sp_std::{fmt::Debug, vec::Vec};

	type BalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_contracts::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UvmCallOk,
		UvmCallError(DispatchError),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidInput,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: frame_system::Config<Lookup = AccountIdLookup<AccountId32, ()>>,
		T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
		<BalanceOf<T> as codec::HasCompact>::Type:
			Clone + Eq + PartialEq + Debug + TypeInfo + Encode,
	{
		#[pallet::weight(0)]
		pub fn uvm_call(
			origin: OriginFor<T>,
			dest: Vec<u8>,
			input: Vec<u8>,
			gas_limit: Weight,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let account_id =
				AccountId32::try_from(&dest[..]).map_err(|_| Error::<T>::InvalidInput)?;

			let res = pallet_contracts::Pallet::<T>::call(
				RawOrigin::Signed(from).into(),
				<AccountIdLookup<AccountId32, ()> as StaticLookup>::unlookup(account_id),
				Default::default(),
				gas_limit,
				None,
				input,
			);

			match res {
				Ok(_) => Self::deposit_event(Event::UvmCallOk),
				Err(e) => Self::deposit_event(Event::UvmCallError(e.error)),
			}

			Ok(())
		}
	}
}
