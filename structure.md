## 说明
本文件描述zk sync核心模块

## 核心模块
zksync-era的核心模块：
1. 部署脚本
2. 环境配置
3. L1智能合约
4. L2智能合约
5. ZK Server
6. ZK Prover

## 部署脚本
部署脚本位于infrastructure/zk目录下。是node+ts项目。zksync-era使用部署脚本执行后续的环境配置，代码编译以及部署等。
首先需要编译部署脚本：
```
zk
```
会把infrastructure/zk/src代码编译到infrastructure/zk/build目录下。

如果修改了infrastructure/zk/src中代码，需要重新执行zk指令编译脚本。

编译好之后，就可以运行部署脚本，部署脚本的 核心逻辑在infrastructure/zk/src/init.ts中，第一次启动时运行zk init。zk init会下载所有工具以及
依赖包，执行编译和部署工作，耗时最久。
后续可运行zk reinit或者zk lightweight-init。区别在于不再执行重复操作。

以zk reinit为例,执行的操作如下：
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
环境配置文件位于etc/env/base目录下，是toml文件。
比如eth_client.toml中，保存了L1的chain id以及URL:
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

编译完后会生成dev.env文件。
zk config compile指令很多时候不用手动执行，在执行zk init或者zk reinit时，会首先检测dev.env文件是否存在，
不存在的话就执行zk config compile。

如果更改了配置文件，那么需要手动执行zk config compile确保配置文件生效

## L1智能合约
L1智能合约位于contracts/ethereum目录下。具体的部署脚本位于contracts/ethereum/scripts中，可添加日志观察部署过程。

部署L1智能合约的相关指令为：
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

## L2智能合约
L2智能合约位于contracts/zksync目录下，具体的部署脚本位于contracts/zksync/src，可添加日志观察部署过程。
L2合约的部署在执行zk contract deploy时，就会和L1合约一起部署。因此没有提供单独的部署指令。
L2的合约部署的整体流程为：
1. 把L2的合约代码保存在L1的合约中。
2. zk server第一次运行时，从L1获取L2合约代码，部署在zk server上。（待进一步确认）

因此执行zk contract deploy部署L2合约时，仅仅是把往L1发送交易，把L2的合约代码保存在L1的合约中，并不是往L1上部署具体的合约。

## ZK Server
ZK Server代码位于core目录下，是rust代码，从Cargo.toml中可以看到具体的workspace。
其中入口代码文件路径为core/bin/zksync_core/src/bin/zksync_server.rs

合约部署好后，可以通过启动zkServer。
```azure
zk server
```
实际执行的指令为：
```azure
cargo run --bin zksync_server --release
```
zk server指令还配置了环境变量，本项目中记录为：runServer.sh
