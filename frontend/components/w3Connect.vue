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
	web3Service.onConnect();
}

function handleDisconnect() {
	console.log('handleDisconnect')
}
</script>

<template>
	<div>
		<VueDappProvider @connect="handleConnect" @disconnect="handleDisconnect">
			<button @click="onClickConnectBtn">{{ isConnected ? 'Disconnect' : 'Connect' }}</button>

			<div>status: {{ status }}</div>
			<div>isConnected: {{ isConnected }}</div>
			<div>error: {{ error }}</div>

			<div v-if="isConnected">
				<div>chainId: {{ chainId }}</div>
				<div>address: {{ address }}</div>
			</div>

			<VueDappModal v-model="isModalOpen" dark auto-connect />
		</VueDappProvider>
	</div>
</template>