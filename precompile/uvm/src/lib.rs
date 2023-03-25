#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, HasCompact, PostDispatchInfo},
	pallet_prelude::Encode,
	scale_info::TypeInfo,
	traits::Currency,
	weights::Weight,
};
use pallet_contracts::chain_extension::UncheckedFrom;
use pallet_evm::{AddressMapping, ExitSucceed, Precompile};
use precompile_utils::{Bytes, FunctionModifier, PrecompileHandleExt, RuntimeHelper};
use sp_runtime::AccountId32;

use fp_evm::{PrecompileHandle, PrecompileOutput, PrecompileResult};
use sp_runtime::traits::AccountIdLookup;

use sp_std::{fmt::Debug, marker::PhantomData};

type BalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

const GAS_LIMIT: Weight = Weight::from_parts(20000000000, 10000000);

/// Each variant represents a method that is exposed in the public Solidity interface
/// The function selectors will be automatically generated at compile-time by the macros
#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
enum Action {
	UvmCall = "uvm_call(bytes,bytes)",
}

pub struct PalletUvmPrecompile<R>(PhantomData<R>);

impl<R> Precompile for PalletUvmPrecompile<R>
where
	R: pallet_uvm::Config,
	R: pallet_evm::Config,
	R: frame_system::Config<Lookup = AccountIdLookup<AccountId32, ()>>,
	<R as frame_system::Config>::AccountId: AsRef<[u8]> + UncheckedFrom<R::Hash>,

	<R as frame_system::Config>::RuntimeCall:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<R as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin:
		From<Option<R::AccountId>>,
	<R as frame_system::Config>::RuntimeCall: From<pallet_uvm::Call<R>>,

	<BalanceOf<R> as HasCompact>::Type: Clone + Eq + PartialEq + Debug + TypeInfo + Encode,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let selector = handle.read_selector()?;
		handle.check_function_modifier(FunctionModifier::NonPayable)?;
		match selector {
			Action::UvmCall => Self::uvm_call(handle),
		}
	}
}

impl<R> PalletUvmPrecompile<R>
where
	R: pallet_uvm::Config,
	R: pallet_evm::Config,
	R: frame_system::Config<Lookup = AccountIdLookup<AccountId32, ()>>,
	<R as frame_system::Config>::AccountId: AsRef<[u8]> + UncheckedFrom<R::Hash>,

	<R as frame_system::Config>::RuntimeCall:
		Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<<R as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin:
		From<Option<R::AccountId>>,
	<R as frame_system::Config>::RuntimeCall: From<pallet_uvm::Call<R>>,
	<BalanceOf<R> as HasCompact>::Type: Clone + Eq + PartialEq + Debug + TypeInfo + Encode,
{
	fn uvm_call(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let mut input = handle.read_input()?;

		input.expect_arguments(2)?;

		let dest = input.read::<Bytes>()?.0;

		let input = input.read::<Bytes>()?.0;

		// Use pallet-evm's account mapping to determine what AccountId to dispatch from.
		let origin = R::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_uvm::Call::<R>::uvm_call { dest, input, gas_limit: GAS_LIMIT };

		// Dispatch the call into the runtime.
		RuntimeHelper::<R>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(PrecompileOutput { exit_status: ExitSucceed::Stopped, output: Default::default() })
	}
}
