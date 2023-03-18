// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

/// @title UVM interface.
/// @dev Execute external VM call. WASM smart contract is currently supported.
interface IUVM {
    /**
     * @param to - the public key of the target contract to call.
     * @param input - SCALE encoded call arguments.
     */
    function uvmCall(
        bytes calldata to,
        bytes calldata input
    ) external;
}
