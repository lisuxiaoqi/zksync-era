## 说明

本文件描述如何把 zksync 部署到本地 eth 私有网络。

使用 zk init 指令，会由脚本运行一个 eth docker 容器，然后把合约部署在该容器中。本文解决如何使用自己的 eth 网络，而不是脚
本运行的 eth 网络中

## 启动 eth 网络

首先本地启动一个 geth 网络作为 L1，可参考项目 https://github.com/lisuxiaoqi/privnets/tree/main/eth/single

这里确定以下几项：

- chainId
- web3 URL
- 初始化账户

其中 chainId 和 web3 URL 需要配置在下面 zk 的环境变量中，初始化账户用于部署 zk 合约，得保证由一定数量的 eth。

## zk 对应步骤

1. 修改部署脚本目的是为了取消脚本启动自己的 geth 容器

这里假设已经运行过 zk init。因此修改 zk reinit 功能，在 infrastructure/zk/src/init.ts 文件中取消掉 container 的启动

```azure
//await announced('Setting up containers', up());
```

更新完后，执行指令编译脚本：

```azure
zk
```

2. 修改配置文件修改环境配置文件位于 etc/env/base/eth_client.toml 文件，更新 L1 的 chain id 以及 URL:

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

3. 启动 docker 容器手动启动 postgreSQL 容器：

```azure
docker-compose up -d postgre
```

4. 启动 zk server

```azure
zk server
```

5. 关闭 docker 容器停止运行时，手动关闭 docker 容器

```azure
zk down
```

## 待调研

- 替换默认的 account
