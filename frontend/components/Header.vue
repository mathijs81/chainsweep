<script lang="ts" setup>
import { web3Service } from '../services/chain';
import detectEthereumProvider from '@metamask/detect-provider';

// TODO: most of this code should move to web3Service
const error = web3Service.getError();
const address = web3Service.getAddress();
const isConnected = computed(() => address.value != null && address.value != '');

async function connect() {
    async function getChainId() {
        const chainId = await window.ethereum?.request({ method: 'eth_chainId' });
        if (chainId) {
            return parseInt(chainId, 16);
        }
        return null;
    }
    web3Service.setError(null);
    let errorMessage = 'Please install MetaMask or other browser wallet';
    try {
        const provider = await detectEthereumProvider();
        if (provider) {
            console.log('Ethereum successfully detected!');
            // From now on, this should always be true:
            // provider === window.ethereum
            // Access the decentralized web!
            const ethereum = window.ethereum;
            if (ethereum) {
                try {
                    //const accounts = await ethereum.request({ method: 'eth_requestAccounts' });
                    const chainId = await getChainId();
                    if (!chainId) {
                        web3Service.setError('Could not get chain ID');
                        return;
                    }
                    web3Service.onConnect(chainId);
                    ethereum.on('accountsChanged', async (accounts: string[]) => {
                        web3Service.onConnect((await getChainId())!!);
                    });
                    ethereum.on('chainChanged', async (chainId: string) => {
                        web3Service.onConnect((await getChainId())!!);
                    });
                    return;
                } catch (err) {
                    if ('message' in (err as any)) {
                        web3Service.setError((err as any).message);
                    } else {
                        web3Service.setError('Error! ' + JSON.stringify(err));
                    }
                }
                return;
            }
        }
    } catch(err) {
        if ('message' in (err as any)) {
            errorMessage = (err as any).message;
        } else {
            errorMessage = 'Error! ' + JSON.stringify(err);
        }
    }
    web3Service.setError(errorMessage);
}

function onClickConnectBtn() {
    if (isConnected.value) {
        window.ethereum?.request({ method: 'eth_requestAccounts', params: [{eth_accounts: {}}]});
    } else {
        connect();
    }
}

onMounted(() => {
    web3Service.addEventListener('should-connect', () => {
        connect();
    });
});
</script>

<template>
    <div class="top-header py-1">
        <div class="container-md">
            <div class="row align-items-center">
                <div class="col d-flex align-items-center">
                    <img src="../img/chainsweep.png" alt="ChainSweep" class="logo-img">
                    <div>
                        <h1 class="fs-3 p-0 m-0">ChainSweep</h1>
                        <small class="d-block" style="margin-top: -5px">An on-chain Arbitrum Stylus game</small>
                    </div>
                </div>
                <div class="col col-auto">
                    <div v-if="error" class="text-danger error">error: {{ error }}</div>
                    <div v-else-if="isConnected"> {{ address }}
                    </div>
                </div>
                <div class="col col-auto">
                    <button class="btn btn-primary" @click="onClickConnectBtn" v-if="!isConnected">Connect</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.logo-img {
    width: 3rem;
    height: 3rem;
    margin-right: 0.5rem;
}
.error {
    background: #fee;
    padding: 0.3rem;
}
</style>