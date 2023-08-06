## 说明
本项目fork自[https://github.com/matter-labs/zksync-era](https://github.com/matter-labs/zksync-era)。详细的文档可参考zksync官方。
本项目用于学习和研究zksync-era，目标如下：
* 理解zksync-era的核心模块组成。
* 能够手动编译和部署各核心模块。
* 能够对核心模块做功能调整。

## 相关文档
* [代码结构](structure.md)
* [使用自己的eth私链搭建zksync-era](eth-zksync.md)

## 快速启动zk sync网络
1. 初始化编译脚本
```azure
zk
```
2. 编译部署zk sync
第一次时运行：
```azure
zk init
```
zk init会下载所有工具，编译代码和部署合约。其中工具下载后，不需要重复下载，因此后续的编译部署往往不再使用zk init。

如果需要重新编译zk server，或者合约，但不需要重新下载所有工具，可运行:
```azure
zk reinit
```
如果未改动任何代码，仅重新初始化网络，可运行:
```azure
zk lightweight-init
```
3. 启动zk网络
启动容器：
```azure
zk up
```
启动zk server
```azure
zk server
```
4. 停止docker容器
结束运行时，需要停止docker容器
```azure
zk down
```

## 其他待调研：
* .init.env文件
* ZK Prover