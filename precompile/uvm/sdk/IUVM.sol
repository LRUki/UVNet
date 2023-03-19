// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

/// @title UVM interface.
/// @dev Execute external VM call. WASM smart contract is currently supported.
interface IUVM {
    /**
     * @param dest - the contract to call.
     * @param input - SCALE encoded call arguments.
     */
    function uvmCall(
        bytes calldata dest,
        bytes calldata input
    ) external;
}
