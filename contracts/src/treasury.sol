// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "@openzeppelin/contracts/access/Ownable.sol";

interface IERC20 {
    function balanceOf(address account) external view returns (uint256);

    function transfer(address to, uint256 amount) external returns (bool);
}

contract Treasury is Ownable {
    // withdraw ETH
    function withdraw(address recipient_) public onlyOwner {
        require(address(this).balance > 0, "Treasury: insufficient balance");
        address payable recipient = payable(recipient_);

        // solhint-disable-next-line avoid-low-level-calls, avoid-call-value
        (bool success, ) = recipient.call{value: address(this).balance}("");
        require(success, "Treasury: faild");
    }

    // withdraw ERC20
    function withdrawERC20(address recipient, address token) public onlyOwner {
        IERC20 erc20Token = IERC20(token);
        require(
            erc20Token.balanceOf(address(this)) > 0,
            "Treasury: insufficient balance"
        );
        erc20Token.transfer(recipient, erc20Token.balanceOf(address(this)));
    }

    fallback() external payable {}

    // solhint-disable-next-line no-empty-blocks
    receive() external payable {}
}
