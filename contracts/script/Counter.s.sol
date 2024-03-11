// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/Counter.sol";

contract CounterScript is Script {
    function setUp() public view {
        console2.log("setup");
    }

    // forge script script/Counter.s.sol --rpc-url=$LOCAL_RPC --broadcast --sender=account --private-key=key
    function run() public {
        // vm.broadcast();
        vm.startBroadcast();
        Counter c = new Counter();
        c.setNumber(100);
        vm.stopBroadcast();
    }

    // forge script script/Counter.s.sol --sig "foo(uint256)" 10
    // function foo(uint256 x) public view {
    //     console2.log("foo =>", x);
    // }
}
