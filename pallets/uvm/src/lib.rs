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

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UvmCall { from: T::AccountId, to: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		OutOfGas,
		// ExecutionErreor((u64)),
		InvalidInput,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: frame_system::Config<Lookup = AccountIdLookup<AccountId32, ()>>,
		T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
		<BalanceOf<T> as codec::HasCompact>::Type:
			Clone + Eq + PartialEq + Debug + TypeInfo + Encode,
	{
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn call_wasm(
			origin: OriginFor<T>,
			contract_address: Vec<u8>,
			input: Vec<u8>,
			gas_limit: Option<Weight>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			log::info!("=========== call_wasm =============");

			let contract_account_id = AccountId32::try_from(&contract_address[..])
				.map_err(|_| Error::<T>::InvalidInput)?;
			log::info!("contract_account_id{:?}", contract_account_id);

			let dest =
				<AccountIdLookup<AccountId32, ()> as StaticLookup>::unlookup(contract_account_id);
			log::info!("dest{:?}", dest);

			let res = pallet_contracts::Pallet::<T>::call(
				RawOrigin::Signed(from).into(),
				dest,
				Default::default(),
				gas_limit.unwrap_or(Weight::from_parts(20000000000, 10000000)),
				None,
				input,
			)
			.map_err(|e| {
				let consumed_weight = if let Some(weight) = e.post_info.actual_weight {
					weight.ref_time()
				} else {
					gas_limit.map_or(0, |g| g.ref_time())
				};
				return Error::<T>::OutOfGas;
				// return Error::<T>::ExecutionError(consumed_weight);
			});
			log::info!("res{:?}", res);

			return match res {
				Err(e) => {
					return Err(e.into());
				},
				_ => Ok(()),
			};
		}
	}
}
