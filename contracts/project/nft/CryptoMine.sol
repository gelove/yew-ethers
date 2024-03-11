// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

interface IXhgt is IERC20 {
    function mint(address _to, uint256 _amount) external returns (bool);
}

contract CryptoMiner is Ownable {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    // Info of each user.
    struct UserInfo {
        uint256 amount; // How many LP tokens the user has provided.
        uint256 rewardDebt; // Reward debt.
        uint256[] tokenIds;
    }

    // Info of each pool.
    struct PoolInfo {
        IERC721 lpToken; // Address of LP token contract.
        uint256 allocPoint; // How many allocation points assigned to this pool. HGTs to distribute per block.
        uint256 lastRewardBlock; // Last block number that HGTs distribution occurs.
        uint256 accXhgtPerShare; // Accumulated HGTs per share, times 1e12.
        uint256 totalAmount; // Total amount of current pool deposit.
        uint16 round; // MintRound
    }

    // The HGT Token!
    IXhgt public xhgt;
    // Info of each pool.
    PoolInfo[] public poolInfo;
    // Info of each user that stakes LP tokens.
    mapping(uint256 => mapping(address => UserInfo)) public userInfo;
    // pid corresponding address
    mapping(address => uint256) public LpOfPid;
    // Total allocation points. Must be the sum of all allocation points in all pools.
    uint256 public totalAllocPoint = 0;

    struct MintRound {
        uint16 round;
        uint256 startBlock;
        uint256 endBlock;
        uint256 mineBlockSupply;
    }

    MintRound[] public mintRounds;

    event Deposit(address indexed user, uint256 indexed pid, uint256 amount);
    event Withdraw(address indexed user, uint256 indexed pid, uint256 amount);
    event EmergencyWithdraw(address indexed user, uint256 indexed pid, uint256 amount);

    constructor(IXhgt _xhgt) {
        xhgt = _xhgt;
    }

    // Set the number of xhgt produced by each block
    function setMintRoundInfo(uint256 _newPerBlock, uint256 _startBlock, uint256 _endBlock) public onlyOwner {
        require(block.number < _startBlock && block.number < _endBlock, "Time must be future");
        require(_endBlock > _startBlock, "EndBlock must greater that startBlock");

        massUpdatePools();
        uint16 len = uint16(mintRounds.length);
        if (len > 0) {
            MintRound memory lastRound = mintRounds[len - 1];
            require(lastRound.endBlock < _startBlock, "One MineRound at a time");
        }
        mintRounds.push(
            MintRound({round: len + 1, startBlock: _startBlock, endBlock: _endBlock, mineBlockSupply: _newPerBlock})
        );
    }

    function getNearestRoundInfo() public view returns (MintRound memory) {
        uint256 len = mintRounds.length;
        require(len > 0, "no mint rounds");

        MintRound memory roundInfo;
        for (uint16 i = 0; i < len; i++) {
            roundInfo = mintRounds[i];
            if (roundInfo.startBlock > block.number || roundInfo.endBlock > block.number) {
                return roundInfo;
            }
        }
        return MintRound({round: 0, startBlock: 0, endBlock: 0, mineBlockSupply: 0});
    }

    function getRoundInfoByBlock(uint256 _blockNumber) public view returns (MintRound memory) {
        uint256 len = mintRounds.length;
        require(len > 0, "no mint rounds");
        MintRound memory roundInfo;

        for (uint16 i = 0; i < len; i++) {
            roundInfo = mintRounds[i];
            if (roundInfo.startBlock <= _blockNumber && roundInfo.endBlock > _blockNumber) {
                return roundInfo;
            }
        }
        return MintRound({round: 0, startBlock: 0, endBlock: 0, mineBlockSupply: 0});
    }

    function poolLength() public view returns (uint256) {
        return poolInfo.length;
    }

    function add(uint256 _allocPoint, IERC721 _lpToken, bool _withUpdate, uint16 round) public onlyOwner {
        require(address(_lpToken) != address(0), "_lpToken is the zero address");
        uint256 len = mintRounds.length;
        require(len > 0, "no mint rounds");

        MintRound memory roundInfo;
        if (round == 0) {
            roundInfo = getNearestRoundInfo();
            round = roundInfo.round;
        }
        roundInfo = mintRounds[round - 1];
        require(block.number < roundInfo.endBlock, "invalid mine round");
        if (_withUpdate) {
            massUpdatePools();
        }
        uint256 lastRewardBlock = block.number > roundInfo.startBlock ? block.number : roundInfo.startBlock;
        totalAllocPoint = totalAllocPoint.add(_allocPoint);
        poolInfo.push(
            PoolInfo({
                lpToken: _lpToken,
                allocPoint: _allocPoint,
                lastRewardBlock: lastRewardBlock,
                accXhgtPerShare: 0,
                totalAmount: 0,
                round: round
            })
        );
        LpOfPid[address(_lpToken)] = poolLength() - 1;
    }

    // Update the given pool's HGT allocation point. Can only be called by the owner.
    function set(uint256 _pid, uint256 _allocPoint, bool _withUpdate) public onlyOwner {
        if (_withUpdate) {
            massUpdatePools();
        }
        totalAllocPoint = totalAllocPoint.sub(poolInfo[_pid].allocPoint).add(_allocPoint);
        poolInfo[_pid].allocPoint = _allocPoint;
    }

    function getXhgtBlockReward(uint256 _lastRewardBlock) public view returns (uint256) {
        uint256 blockReward = 0;
        MintRound memory mintRound = getRoundInfoByBlock(_lastRewardBlock);

        if (mintRound.round > 0) {
            uint256 endBlock = block.number;
            if (mintRound.endBlock <= block.number) {
                endBlock = mintRound.endBlock;
            }
            blockReward = (endBlock.sub(_lastRewardBlock)).mul(mintRound.mineBlockSupply);
        }
        return blockReward;
    }

    // Update reward variables for all pools. Be careful of gas spending!
    function massUpdatePools() public {
        uint256 length = poolInfo.length;
        for (uint256 pid = 0; pid < length; ++pid) {
            updatePool(pid);
        }
    }

    // Update reward variables of the given pool to be up-to-date.
    function updatePool(uint256 _pid) public {
        PoolInfo storage pool = poolInfo[_pid];
        if (block.number <= pool.lastRewardBlock) {
            return;
        }
        uint256 lpSupply;

        lpSupply = pool.lpToken.balanceOf(address(this));
        if (lpSupply == 0) {
            pool.lastRewardBlock = block.number;
            return;
        }

        uint256 blockReward = getXhgtBlockReward(pool.lastRewardBlock);
        if (blockReward <= 0) {
            return;
        }
        uint256 xhgtReward = blockReward.mul(pool.allocPoint).div(totalAllocPoint);
        bool minRet = xhgt.mint(address(this), xhgtReward);
        if (minRet) {
            pool.accXhgtPerShare = pool.accXhgtPerShare.add(xhgtReward.mul(1e12).div(lpSupply));
        }
        MintRound memory mintRound = getRoundInfoByBlock(pool.lastRewardBlock);
        if (mintRound.round > 0 && block.number >= mintRound.endBlock) {
            pool.lastRewardBlock = mintRound.endBlock;
        } else {
            pool.lastRewardBlock = block.number;
        }
    }

    // View function to see pending HGTs on frontend.
    function pending(uint256 _pid, address _user) external view returns (uint256, uint256) {
        uint256 xhgtAmount = pendingXhgt(_pid, _user);
        return (xhgtAmount, 0);
    }

    function pendingXhgt(uint256 _pid, address _user) private view returns (uint256) {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][_user];
        uint256 accXhgtPerShare = pool.accXhgtPerShare;
        uint256 lpSupply = pool.lpToken.balanceOf(address(this));
        if (user.amount > 0) {
            if (block.number > pool.lastRewardBlock) {
                uint256 blockReward = getXhgtBlockReward(pool.lastRewardBlock);
                uint256 xhgtReward = blockReward.mul(pool.allocPoint).div(totalAllocPoint);
                accXhgtPerShare = accXhgtPerShare.add(xhgtReward.mul(1e12).div(lpSupply));
                return user.amount.mul(accXhgtPerShare).div(1e12).sub(user.rewardDebt);
            }
            if (block.number == pool.lastRewardBlock) {
                return user.amount.mul(accXhgtPerShare).div(1e12).sub(user.rewardDebt);
            }
        }
        return 0;
    }

    // Deposit LP tokens to HGTMiner for HGT allocation.
    function deposit(uint256 _pid, uint256 _tokenId) public {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];

        MintRound memory mintRound = getRoundInfoByBlock(block.number);
        require(mintRound.round > 0, "Not mine time now");
        updatePool(_pid);
        if (user.amount > 0) {
            uint256 pendingAmount = user.amount.mul(pool.accXhgtPerShare).div(1e12).sub(user.rewardDebt);
            if (pendingAmount > 0) {
                safeXhgtTransfer(msg.sender, pendingAmount);
            }
        }
        if (_tokenId > 0) {
            pool.lpToken.transferFrom(msg.sender, address(this), _tokenId);
            user.amount = user.amount.add(1);
            user.tokenIds.push(_tokenId);
            pool.totalAmount = pool.totalAmount.add(1);
        }
        user.rewardDebt = user.amount.mul(pool.accXhgtPerShare).div(1e12);
        emit Deposit(msg.sender, _pid, _tokenId);
    }

    // Withdraw LP tokens from HGTMiner.
    function withdraw(uint256 _pid, uint256 _tokenId) public {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];
        // 判断用户是否抵押了此Token
        uint256 _tokenIdIndex = 0;
        for (uint256 i = 0; i < user.tokenIds.length; i++) {
            if (user.tokenIds[i] == _tokenId) {
                _tokenIdIndex = i.add(1);
                break;
            }
        }
        require(_tokenId == 0 || _tokenIdIndex > 0, "Wrong token id");
        updatePool(_pid);
        uint256 pendingAmount = user.amount.mul(pool.accXhgtPerShare).div(1e12).sub(user.rewardDebt);
        if (pendingAmount > 0) {
            safeXhgtTransfer(msg.sender, pendingAmount);
        }
        if (_tokenIdIndex > 0) {
            user.amount = user.amount.sub(1);
            delete user.tokenIds[_tokenIdIndex.sub(1)];
            pool.totalAmount = pool.totalAmount.sub(1);
            pool.lpToken.transferFrom(address(this), msg.sender, _tokenId);
        }
        user.rewardDebt = user.amount.mul(pool.accXhgtPerShare).div(1e12);
        emit Withdraw(msg.sender, _pid, _tokenId);
    }

    // Safe HGT transfer function, just in case if rounding error causes pool to not have enough HGTs.
    function safeXhgtTransfer(address _to, uint256 _amount) internal {
        uint256 xhgtBal = xhgt.balanceOf(address(this));
        if (_amount > xhgtBal) {
            xhgt.transfer(_to, xhgtBal);
        } else {
            xhgt.transfer(_to, _amount);
        }
    }
}
