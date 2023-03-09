// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

/// @title UVM interface.
/// @dev Execute external VM call. WASM smart contract is currently supported.
interface IUVM {
    /**
     * @param contract_address - the target WASM smart contract address to call.
     * @param input - SCALE encoded call arguments.
     */
    function uvmCall(
        bytes calldata contract_address,
        bytes calldata input
    ) external;
}
