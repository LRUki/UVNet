// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

// The pallet template precompile is at ad address 0x801
contract CallPalletTemplate {
    
    function call_do_something(uint256 val) external {
        bytes4 selector = getSelector("do_something(uint256)");
        assembly {
            // 0x40 is free memory.
            let p := mload(0x40)

            // The first four bytes of the input is the function selector.
            mstore(p, selector)
            // Load the value to store in pallet_template.
            mstore(add(p, 0x4), val)
        
            if iszero(call(gas(), 0x801, 0, p, 0x24, p, 0)) {
                revert(0, 0)
            }
        }
    }

    function call_get_value() external view returns (uint256 res) {
        bytes4 selector = getSelector("get_value()");
        assembly {
            let p := mload(0x40)

            mstore(p, selector)

            if iszero(staticcall(gas(), 0x801, p, 0x4, p, 0x20)) {
                revert(0, 0)
            }

            res := mload(p)
        }
    }

    function getSelector(string memory _func) private pure returns (bytes4) {
        return bytes4(keccak256(bytes(_func)));
    }
}
