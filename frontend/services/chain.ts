
import { mainnet, arbitrum } from 'viem/chains'
import { createWalletClient, custom, defineChain, type Client, type WalletClient } from 'viem';

export const targetChain = defineChain({
	id: 412346,
	name: "Stylus Devnet",
	network: "Arbitrum Stylus",
	nativeCurrency: {
		decimals: 18,
		name: "Ether",
		symbol: "ETH",
	},
	rpcUrls: {
		default: {
			http: ["http://localhost:8547"],
			webSocket: [
				"wss://mainnet.infura.io/ws/v3/68c04ec6f9ce42c5becbed52a464ef81",
			],
		},
		public: {
			http: ["http://localhost:8547"],
			webSocket: [
				"wss://mainnet.infura.io/ws/v3/68c04ec6f9ce42c5becbed52a464ef81",
			],
		},
	},
	blockExplorers: {
		default: {
			name: "Explorer",
			url: "https://stylus-testnet-explorer.arbitrum.io/",
		},
	},
});

export enum GameState {
    UNSTARTED,
    PLAYING,
    WON,
    LOST
}

export class Game {
    state: GameState = GameState.UNSTARTED;
    field: string[] = [];

    width() {
        return this.field[0]?.length ?? 0;
    }
    height() {
        return this.field.length;
    }
}

class Web3Service {
    private client: WalletClient | null = null;
    private address: Ref<string | null> = ref(null);
    private currentGame: Ref<Game | null> = ref(null);

    getCurrentGame() {
        return readonly(this.currentGame);
    }

    async onConnect() {
        if (window.ethereum == null) {
            this.client = null;
            this.address.value = null;
        } else {
            console.log('creating client');
            this.client = createWalletClient({
                chain: targetChain,
                transport: custom(window.ethereum)
            })
            const addresses = await this.client?.getAddresses();
            this.address.value = addresses?.[0] ?? null;
            
        }   
    }
}

export const web3Service = new Web3Service();