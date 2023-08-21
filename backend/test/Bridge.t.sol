// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Bridge.sol";

contract CounterTest is Test {
    zkBridge public bridge;

    function setUp() public {
        bridge = new zkBridge(1);
        bridge.deposit();
    }

    function testDeposit() public {
        bridge.deposit();
        // assertEq(bridge._total_coins, 1);
    }

    function testWithdraw() public {
        bridge.withdraw();
        // assertEq(bridge._total_coins, 1);
    }
}