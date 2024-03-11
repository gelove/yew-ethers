// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract Grandpa {
    uint256 public a;

    constructor(uint256 _a) {
        a = _a;
    }

    function hi() public pure virtual returns (string memory) {
        return "Grandpa";
    }

    function grandpa() public pure returns (string memory) {
        return "Grandpa";
    }
}

// 构造函数的继承
contract Dad is Grandpa {
    bool public b;

    constructor(uint256 _a, bool _b) Grandpa(_a) {
        b = _b;
    }

    modifier exactDivided(uint256 _v) virtual {
        require(_v % 2 == 0 && _v % 3 == 0, "must to be divisible by 2 and 3");
        _;
    }

    function hi() public pure virtual override returns (string memory) {
        return "Dad";
    }

    function dad() public pure returns (string memory) {
        return "Dad";
    }

    function getExactDivided(uint256 _dividend)
        public
        pure
        virtual
        Dad.exactDivided(_dividend)
        returns (uint256, uint256)
    {
        return _getExactDividedWithoutModifier(_dividend);
    }

    //计算一个数分别被2除和被3除的值
    function _getExactDividedWithoutModifier(uint256 _dividend) internal pure virtual returns (uint256, uint256) {
        uint256 div2 = _dividend / 2;
        uint256 div3 = _dividend / 3;
        return (div2, div3);
    }
}

contract Son is Dad {
    uint8 public c;

    modifier exactDivided(uint256 _v) override {
        require(_v % 2 == 0 && _v % 3 == 0 && _v % 5 == 0, "must to be divisible by 2 and 3 and 5");
        _;
    }

    constructor(uint256 _a, bool _b, uint8 _c) Dad(_a * _a, _b) {
        c = _c;
    }

    function hi() public pure override returns (string memory) {
        return "Son";
    }

    function getExactDividedByDad(uint256 _dividend)
        public
        pure
        Dad.exactDivided(_dividend)
        returns (uint256, uint256)
    {
        return Dad._getExactDividedWithoutModifier(_dividend);
    }

    function getExactDividedNew(uint256 _dividend)
        public
        pure
        exactDivided(_dividend)
        returns (uint256, uint256, uint256)
    {
        return _getExactDividedWithoutModifierNew(_dividend);
    }

    // //计算一个数分别被2除和被3除的值
    function _getExactDividedWithoutModifierNew(uint256 _dividend) internal pure returns (uint256, uint256, uint256) {
        uint256 div2 = _dividend / 2;
        uint256 div3 = _dividend / 3;
        uint256 div5 = _dividend / 5;
        return (div2, div3, div5);
    }
}
