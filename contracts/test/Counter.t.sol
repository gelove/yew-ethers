// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract CounterTest is Test {
    event MyEvent(uint256 indexed a, uint256 indexed b, uint256 indexed c, uint256 d, uint256 e);

    Counter public counter;
    Helper public help;
    address public alice;
    IERC20 public dai;

    function setUp() public {
        counter = new Counter();
        counter.setNumber(0);
        help = new Helper();
        alice = address(10086);
        dai = IERC20(0x6B175474E89094C44Da98b954EedeAC495271d0F);
    }

    function testIncrement() public {
        counter.increment();
        assertEq(counter.number(), 1);
    }

    function testSetNumber(uint256 x) public {
        counter.setNumber(x);
        assertEq(counter.number(), x);
    }

    // function testDouble(uint256 x) public {
    //     counter.setNumber(x);
    //     counter.double();
    //     assertEq(counter.number(), 2 * x);
    // }

    // invariant
    // function invariant_some() public {
    //     // assertTrue(counter.number() < type(uint128).max);
    //     assertTrue(counter.number() <= type(uint256).max);
    // }

    // 时间戳
    // function testTimestamp() public {
    //     console2.log("timestamp before =>", block.timestamp);
    //     vm.warp(10000);
    //     console2.log("timestamp after =>", block.timestamp);
    // }

    // 区块高度
    // function testBlockNumber() public {
    //     console2.log("number before =>", block.number);
    //     vm.roll(10000);
    //     console2.log("number after =>", block.number);
    // }

    // 模拟调用者
    // function testCaller() public {
    //     emit log_address(address(msg.sender));
    //     // 测试时模拟管理员
    //     // vm.prank(alice); // 只影响下次调用
    //     vm.startPrank(alice); // 持续到stop
    //     address caller = help.whoCalled();
    //     console2.log("caller =>", caller);
    //     console2.log("caller =>", address(this));
    //     address caller1 = help.whoCalled();
    //     console2.log("caller =>", caller1);
    //     vm.stopPrank();
    // }

    // function testDeal() public {
    //     console2.log("eth balance before =>", alice.balance);
    //     vm.deal(alice, 1 ether);
    //     console2.log("eth balance after =>", alice.balance);
    // }

    // fork节点并指定区块高度
    // function testForkByRpc() public {
    //     // 代码设置fork
    //     string memory rpc = vm.envString("ETH_RPC_URL");
    //     uint256 mainNet = vm.createFork(rpc);
    //     vm.selectFork(mainNet);
    //     console2.log("block.number =>", block.number);
    //     vm.rollFork(15550000);
    //     console2.log("block.number =>", block.number);
    //     vm.makePersistent(address(dai), alice);

    //     // testERC20Deal
    //     console2.log("before =>", dai.balanceOf(alice));
    //     // function deal(address token, address to, uint256 give) internal
    //     deal(address(dai), alice, 1 ether);
    //     console2.log("after =>", dai.balanceOf(alice));
    // }

    // 判断内部函数执行结果是否与外部程序执行结果一致
    // function testFFI() public {
    //     string memory message = "hello";
    //     bytes32 hash1 = keccak256(abi.encodePacked(message));
    //     console2.logBytes32(hash1);

    //     string[] memory cmd = new string[](3);
    //     cmd[0] = "cast";
    //     cmd[1] = "keccak";
    //     cmd[2] = message;
    //     bytes memory result = vm.ffi(cmd);
    //     bytes32 hash2 = abi.decode(result, (bytes32));
    //     console2.logBytes32(hash2);
    //     assertEq(hash1, hash2);
    // }

    // 测试事件日志
    // function testEmit() public {
    //     // bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData
    //     vm.expectEmit(true, true, true, true);
    //     // 触发期望的测试事件
    //     // emit MyEvent(5, 5, 5, 5, 5);
    //     emit MyEvent(1, 2, 3, 4, 5);
    //     // 调用具体的函数实现业务逻辑 比对测试结果
    //     help.emitEvent();
    // }

    // 测试错误
    // function testFail() public view {
    //     help.revertIt();
    // }

    // 测试错误具体原因
    // function testRevert() public {
    //     vm.expectRevert("some reason");
    //     help.revertIt();
    // }

    // function testRevertCustom() public {
    //     // vm.expectRevert(Helper.CustomError.selector);
    //     vm.expectRevert(abi.encodeWithSelector(Helper.CustomError.selector, 100));
    //     help.revertByCustom();
    // }
}

contract Helper {
    // 类似于 rust 中的单元结构体
    // error CustomError();
    // 类似于 rust 中的元组结构体
    error CustomError(uint256);

    event MyEvent(uint256 indexed a, uint256 indexed b, uint256 indexed c, uint256 d, uint256 e);

    function whoCalled() public view returns (address) {
        return msg.sender;
    }

    function emitEvent() public {
        emit MyEvent(1, 2, 3, 4, 5);
    }

    function revertIt() public pure {
        revert("some reason");
    }

    function revertByCustom() public pure {
        // revert CustomError();
        revert CustomError(100);
    }
}
