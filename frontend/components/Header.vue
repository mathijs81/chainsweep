<script lang="ts" setup>
import { BrowserWalletConnector, VueDappProvider, type ConnWallet, useVueDapp } from '@vue-dapp/core'
import { VueDappModal } from '@vue-dapp/modal'
import '@vue-dapp/modal/dist/style.css'
import { web3Service } from '../services/chain';


const { status, isConnected, address, chainId, error, disconnect, addConnector } = useVueDapp()

const isModalOpen = ref(false)

function onClickConnectBtn() {
    if (isConnected.value) disconnect()
    else isModalOpen.value = true
}

if (process.client) { // only when using Nuxt 3
    addConnector(new BrowserWalletConnector({
    }))
}

function handleConnect(wallet: ConnWallet) {
    console.log('handleConnect', wallet)
    web3Service.onConnect(wallet.chainId);
}

function handleDisconnect() {
    console.log('handleDisconnect')
}

useVueDapp().onAccountOrChainIdChanged((wallet: ConnWallet) => {
    web3Service.onConnect(wallet.chainId);
});

onMounted(() => {
    web3Service.addEventListener('should-connect', () => {
        isModalOpen.value = true
    });
});
</script>

<template>
    <div class="top-header py-1">
        <div class="container-md">
            <div class="row align-items-center">
                <div class="col">
                    <h1 class="fs-3 py-1 m-0">ChainSweep</h1>
                </div>
                <VueDappProvider @connect="handleConnect" @disconnect="handleDisconnect">
                    <div class="col col-auto">
                        <div v-if="error" class="text-danger">error: {{ error }}</div>
                        <div v-if="isConnected"> {{ address }}
                        </div>
                    </div>
                    <div class="col col-auto">
                        <button class="btn btn-primary" @click="onClickConnectBtn">{{ isConnected ? 'Disconnect' :
                    'Connect'
                            }}</button>
                    </div>
                    <VueDappModal v-model="isModalOpen" auto-connect />
                </VueDappProvider>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.top-header {
    background-color: #f8f9fa;
    border-bottom: 1px solid #e9ecef;
}
</style>