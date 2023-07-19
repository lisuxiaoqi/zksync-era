const { ProxyAgent, setGlobalDispatcher } = require('undici');
const proxyAgent = new ProxyAgent('http://127.0.0.1:1087');
setGlobalDispatcher(proxyAgent);

import '@matterlabs/hardhat-zksync-solc';

export default {
    zksolc: {
        version: '1.3.1',
        compilerSource: 'binary',
        settings: {
            isSystem: true
        }
    },
    networks: {
        hardhat: {
            zksync: true
        }
    },
    solidity: {
        version: '0.8.16'
    }
};
