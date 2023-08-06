## 说明

本项目 fork 自[https://github.com/matter-labs/zksync-era](https://github.com/matter-labs/zksync-era)。详细的文档可参考
zksync 官方。本项目用于学习和研究 zksync-era，目标如下：

- 理解 zksync-era 的核心模块组成。
- 能够手动编译和部署各核心模块。
- 能够对核心模块做功能调整。

## 相关文档

- [代码结构](structure.md)
- [使用自己的 eth 私链搭建 zksync-era](eth-zksync.md)

## 快速启动 zk sync 网络

1. 初始化编译脚本

```azure
zk
```

2. 编译部署 zk sync 第一次时运行：

```azure
zk init
```

zk init 会下载所有工具，编译代码和部署合约。其中工具下载后，不需要重复下载，因此后续的编译部署往往不再使用 zk init。

如果需要重新编译 zk server，或者合约，但不需要重新下载所有工具，可运行:

```azure
zk reinit
```

如果未改动任何代码，仅重新初始化网络，可运行:

```azure
zk lightweight-init
```

3. 启动 zk 网络启动容器：

```azure
zk up
```

启动 zk server

```azure
zk server
```

4. 停止 docker 容器结束运行时，需要停止 docker 容器

```azure
zk down
```

## 其他待调研：

- .init.env 文件
- ZK Prover
