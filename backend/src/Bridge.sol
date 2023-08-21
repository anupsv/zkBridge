// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract zkBridge {

    // @dev Total deposits in the pool
    uint256 public _total_coins;

    // @dev Each pool has a specific deposit amount
    uint256 public _pool_deposit_amount;


    // @dev Setup pool parameters
    constructor(uint256 _pool_deposit_amount_) {

        _pool_deposit_amount = _pool_deposit_amount_;

    }

    // @dev Deposit function
    // TODO: How can we make this anonymous
    function deposit() public {
        
        _total_coins = _total_coins += _pool_deposit_amount;
    }

    // @dev Withdraw function
    // TODO: How can we make this anonymous
    function withdraw() public {
        // @dev 
    }
}
