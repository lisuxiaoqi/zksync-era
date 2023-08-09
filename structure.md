## 说明

本文件描述 zk sync 核心模块

## 核心模块

zksync-era 的核心模块：

1. 部署脚本
2. 环境配置
3. L1 智能合约
4. L2 智能合约
5. ZK Server
6. ZK Prover

## 部署脚本

部署脚本位于 infrastructure/zk 目录下。是 node+ts 项目。zksync-era 使用部署脚本执行后续的环境配置，代码编译以及部署等。
首先需要编译部署脚本：

```
zk
```

会把 infrastructure/zk/src 代码编译到 infrastructure/zk/build 目录下。

如果修改了 infrastructure/zk/src 中代码，需要重新执行 zk 指令编译脚本。

编译好之后，就可以运行部署脚本，部署脚本的 核心逻辑在 infrastructure/zk/src/init.ts 中，第一次启动时运行 zk init。zk
init 会下载所有工具以及依赖包，执行编译和部署工作，耗时最久。后续可运行 zk reinit 或者 zk lightweight-init。区别在于不再
执行重复操作。

以 zk reinit 为例,执行的操作如下：

```azure
//启动geth和postgreSQL 容器
await announced('Setting up containers', up());
//编译环境变量，以及一些工具脚本
await announced('Compiling JS packages', run.yarn());
//编译L2系统合约
await announced('Compile l2 contracts', compiler.compileAll());
//重置postgreSQL
await announced('Drop postgres db', db.drop());
await announced('Setup postgres db', db.setup());
//重置rockdb
await announced('Clean rocksdb', clean('db'));
await announced('Clean backups', clean('backups'));
//编译L1和L2合约
await announced('Building contracts', contract.build());
//初始化zk server
await announced('Running server genesis setup', server.genesisFromSources());
//部署L1合约
await announced('Deploying L1 contracts', contract.redeployL1([]));
//初始化L1合约
await announced('Initializing validator', contract.initializeValidator());
await announced('Initializing L1 Allow list', contract.initializeL1AllowList());
//部署L2合约
await announced('Deploying L2 contracts', contract.deployL2());
```

## 环境配置

环境配置文件位于 etc/env/base 目录下，是 toml 文件。比如 eth_client.toml 中，保存了 L1 的 chain id 以及 URL:

```
[eth_client]
chain_id=66
# Addresses of the Ethereum node API, separated by comma
web3_url="http://127.0.0.1:8545"
```

执行以下指令手动编译配置文件：

```
zk config compile
```

编译完后会生成 dev.env 文件。 zk config compile 指令很多时候不用手动执行，在执行 zk init 或者 zk reinit 时，会首先检测
dev.env 文件是否存在，不存在的话就执行 zk config compile。

如果更改了配置文件，那么需要手动执行 zk config compile 确保配置文件生效

## L1 智能合约

L1 智能合约位于 contracts/ethereum 目录下。具体的部署脚本位于 contracts/ethereum/scripts 中，可添加日志观察部署过程。

部署 L1 智能合约的相关指令为：

```
zk contract
      redeploy [deploy-opts...]         redeploy contracts
      deploy [deploy-opts...]           deploy contracts
      build                             build contracts
      initilize-validator               initialize validator
      initilize-l1-allow-list-contract  initialize L1 allow list contract
      verify                            verify L1 contracts
      help [command]                    display help for command
```

## L2 智能合约

L2 智能合约位于 contracts/zksync 目录下，具体的部署脚本位于 contracts/zksync/src，可添加日志观察部署过程。 L2 合约的部署
在执行 zk contract deploy 时，就会和 L1 合约一起部署。因此没有提供单独的部署指令。 L2 的合约部署的整体流程为：

1. 把 L2 的合约代码保存在 L1 的合约中。
2. zk server 第一次运行时，从 L1 获取 L2 合约代码，部署在 zk server 上。（待进一步确认）

因此执行 zk contract deploy 部署 L2 合约时，仅仅是把往 L1 发送交易，把 L2 的合约代码保存在 L1 的合约中，并不是往 L1 上部
署具体的合约。

编译后合约的日志保存于zksync-era/deployL1.log，zksync-era/deployL2.log以及deployed_contracts.log文件中

## ZK Server

ZK Server 代码位于 core 目录下，是 rust 代码，从 Cargo.toml 中可以看到具体的 workspace。其中入口代码文件路径为
core/bin/zksync_core/src/bin/zksync_server.rs

合约部署好后，可以通过启动 zkServer。

```azure
zk server
```

实际执行的指令为：

```azure
cargo run --bin zksync_server --release
```

zk server 指令还配置了环境变量，本项目中记录为：runServer.sh
