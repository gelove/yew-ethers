# yew-ethers rust全栈以太坊链上工具

## tauri写web3项目有坑, 已经放弃，做个纪念

## install tauri

### 前端项目配置文件中可以修改端口号

app/Trunk.toml

```toml
[serve]
port = 9090
```

### 合约库

```sh
# 查看合约存储字段 cast storage contract_address slot
cast storage 0xc18360217d8f7ab5e7c516566761ea12ce7f9d72 1
# 查看链上某个交易的汇编代码 cast run tx_hash --debug
cast run 0x9989a6c6c58aa68b895e39138daa5f7922cafa7927dda00644c34b87103a8a6d --debug
# 代码美化
forge fmt
# 安装合约库 forge install <git_user>/<git_repo>@<tag>
forge install Rari-Capital/solmate
forge install openzeppelin/openzeppelin-contracts
forge install Rari-Capital/solmate Openzeppelin/openzeppelin-contracts 
# 更新合约库 forge update [LIB]
forge update lib/openzeppelin-contracts
# 删除合约库 forge remove [LIB]
forge remove lib/openzeppelin-contracts
# 重新映射依赖
forge remappings > remappings.txt
# 构建并监听
forge build -w --root ./contracts/
# 生成rust代码绑定
forge bind --root ./contracts --bindings-path ./app/bindings --crate-name bindings
# 测试并监听
forge test -vvv -w
# 指定根目录
forge test -vvv -w --root ./contracts/
# 开启 ffi
forge test -vvv -w --root ./contracts/ --ffi
# fork 指定节点和区块高度
forge test -vvv -w --root ./contracts/ --ffi --fork-url $ETH_RPC_URL --fork-block-number 15550000
# 检查合约中的方法签名 methods/methodIdentifiers/method_identifiers/method-identifiers/mi
forge inspect Counter mi
# 检查合约 abi
forge inspect Counter abi
# 检查合约存储 字段插槽
forge inspect Counter storage
# 清除缓存
forge clean
# gas 报告
forge test --gas-report
# 记录快照在 .gas-snapshot
forge snapshot
vim .gas-snapshot
# 用来查看代码优化后的 gas 差异
forge snapshot --diff
# debug 汇编操作码
forge script script/Counter.s.sol --debug
# 测试代码覆盖
forge coverage
# 测试代码覆盖 生成报告
forge coverage --report lcov
```

### 项目根目录中执行以下命令

```sh
$ cargo tauri init
What is your app name?: yew-ethers
What should the window title be?: Yew Ethers
Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src| tauri/tauri.conf.json" file that will be created?: | ../app/dist |
What is the url of your dev server?: http://localhost:9090
```

### 替换以下配置文件中的**build**片段:

src-tauri/tauri.conf.json

```json
"build": {
    "distDir": "../app/dist",
    "devPath": "http://localhost:9090",
    "beforeDevCommand": "cd app && trunk serve",
    "beforeBuildCommand": "cd app && trunk build",
    "withGlobalTauri": true
},
```

### Blcok Header

| 字段            | 说明                                         |
| --------------- | -------------------------------------------- |
| parentHash      | 父区块的 Keccak 哈希                         |
| sha3Uncles      | 叔块的 sha3 哈希                             |
| stateRoot       | 状态树的根哈希                               |
| hash            | 交易树的根哈希                               |
| receiptsRoot    | 收据树的根哈希                               |
| miner           | 受益人地址，矿工地址                         |
| baseFeePerGas   | 每一个 gas 的基础费用                        |
| difficulty      | 前一个区块的难度                             |
| extraData       | 扩展值 与该区块相关的 32 字节数据            |
| gasLimit        | 当前每个区块的 gas 使用限制值                |
| gasUsed         | 该区块中用于交易的 gas 消耗值                |
| logsBloom       | 事件地址和事件 topic 的布隆滤波器            |
| mixHash         | 与 nonce 一起用来证明工作证明计算的 256 位值 |
| nonce           | 用于工作证明计算的随机数                     |
| number          | 父区块高度                                   |
| timestamp       | 时间戳                                       |
| transactions:   | 区块内所有交易的哈希数组                     |
| totalDifficulty | 47894130742508082887767                      |
| sealFields      | []                                           |
| size            | 150954                                       |

### thereum Account

以太坊账户是以太坊地址的共识代表，它由 4 部分构成

| 字段         | 说明                            |
| ------------ | ------------------------------- |
| Nonce        | 该账号的交易数量                |
| Banlance     | 账户余额（以 wei 为单位）       |
| Code Hash    | 合约字节码的 hash 值            |
| Storage Root | storage trie 的根节点的 hash 值 |

合约账户 storage 的任何变化都会影响到 "Storage Root"，进而影响到 "State Root"，进而影响到 "Block Header"。

应该说账户数据的任何变化都会影响到 "State Root"，进而影响到 "Block Header"。

### 三种数据结构

- StateAccount: 状态账户是以太坊账户的'consensus representation'，代表一个以太坊账户。
- stateObject: stateObject 代表一个正在被修改的 "Ethereum 账户"。
- StateDB: 以太坊协议内的 StateDB 结构是用来存储 Merkle trie 内的任何东西。它是检索合约和以太坊账户的查询接口。
