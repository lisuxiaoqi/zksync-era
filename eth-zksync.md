## 说明
本文件描述如何把zksync部署到本地eth私有网络。

使用zk init指令，会由脚本运行一个eth docker容器，然后把合约部署在该容器中。本文解决
如何使用自己的eth网络，而不是脚本运行的eth网络中

## 启动eth网络
首先本地启动一个geth网络作为L1，可参考项目
https://github.com/lisuxiaoqi/privnets/tree/main/eth/single

这里确定以下几项：
* chainId
* web3 URL
* 初始化账户

其中chainId和web3 URL需要配置在下面zk的环境变量中，初始化账户用于部署zk合约，得保证由一定数量的eth。

## zk对应步骤
1. 修改部署脚本
目的是为了取消脚本启动自己的geth容器

这里假设已经运行过zk init。因此修改zk reinit功能，
在infrastructure/zk/src/init.ts文件中取消掉container的启动
```azure
//await announced('Setting up containers', up());
```
更新完后，执行指令编译脚本：
```azure
zk
```

2. 修改配置文件
修改环境配置文件位于etc/env/base/eth_client.toml文件，更新L1的chain id以及URL:
```
[eth_client]
chain_id=66
# Addresses of the Ethereum node API, separated by comma
web3_url="http://127.0.0.1:8545"
```
更新完成后，执行编译部署指令：
```azure
zk reinit
```
3. 启动docker容器
手动启动postgreSQL容器：
```azure
docker-compose up -d postgre
```
4. 启动zk server
```azure
zk server
```
5. 关闭docker容器
停止运行时，手动关闭docker容器
```azure
zk down
```

## 待调研
* 替换默认的account