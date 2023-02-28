#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
};
use pallet_evm::{AddressMapping, Precompile, ExitSucceed};
use precompile_utils::{
	 EvmDataWriter, RuntimeHelper, PrecompileHandleExt, FunctionModifier,
};

use fp_evm::{ PrecompileOutput, PrecompileHandle, PrecompileResult};

use sp_std::{
	fmt::Debug,
	marker::PhantomData,
};

/// Each variant represents a method that is exposed in the public Solidity interface
/// The function selectors will be automatically generated at compile-time by the macros
#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
enum Action {
	DoSomething = "do_something(uint256)",
	GetValue = "get_value()",
}

pub struct PalletTemplatePrecompile<R>(PhantomData<R>);

impl<R> Precompile for PalletTemplatePrecompile<R> 
where
	R: pallet_template::Config + pallet_evm::Config,
	R::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<R::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<R::AccountId>>,
	R::RuntimeCall: From<pallet_template::Call<R>>,
{
	fn execute(
		handle: &mut impl PrecompileHandle
	) ->  PrecompileResult {
        let selector = handle.read_selector()?;
		match selector { 
			// Check for accessor methods first. These return results immediately
			Action::DoSomething => Self::do_something(handle),
			Action::GetValue => Self::get_value(handle),
		}
	}
}

impl<R> PalletTemplatePrecompile<R>
where
	R: pallet_template::Config + pallet_evm::Config,
	R::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
    <R::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<R::AccountId>>,
    R::RuntimeCall: From<pallet_template::Call<R>>,
{
	fn do_something(
        handle: &mut impl PrecompileHandle,
	) -> PrecompileResult {
        handle.check_function_modifier(FunctionModifier::NonPayable)?;
	
        let mut input = handle.read_input()?;

		// Bound check. We expect a single argument passed in.
		input.expect_arguments(1)?;
		
		// Parse the u32 value that will be dispatched to the pallet.
		let value = input.read::<u32>()?.into();

		// Use pallet-evm's account mapping to determine what AccountId to dispatch from.
		let origin = R::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_template::Call::<R>::do_something{something: value};

		// Dispatch the call into the runtime.
		// The RuntimeHelper tells how much gas was actually used.
		RuntimeHelper::<R>::try_dispatch(
            handle,
			Some(origin).into(),
			call,
		)?;

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Stopped,
			output: Default::default(),
		})
	}

	fn get_value(
        handle: &mut impl PrecompileHandle,
	) -> PrecompileResult {
        handle.check_function_modifier(FunctionModifier::View)?;

        let input = handle.read_input()?;
		// Bound check
		input.expect_arguments(0)?;

		// fetch data from pallet
		let stored_value = pallet_template::Something::<R>::get().unwrap_or_default();

		// Construct to Solidity-formatted output data
		let output = EvmDataWriter::new().write(stored_value).build();

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			output,
		})
	}
}
