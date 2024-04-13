
import { mainnet, arbitrum } from 'viem/chains'
import { createPublicClient, createWalletClient, custom, defineChain, http, type Address, type Client, type WalletClient } from 'viem';

import { getContract} from 'viem';
import { chainsweepAbi } from '../src/generated';

const contractAddress = '0xCA8c8688914e0F7096c920146cd0Ad85cD7Ae8b9';


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
    private publicClient: Client = createPublicClient({
        chain: targetChain,
        transport: http()
    });
    private client: WalletClient | null = null;
    private address: Ref<Address | null> = ref(null);
    private currentGame: Ref<Game | null> = ref(null);

    private contract() {
        const contract = getContract({
            address: contractAddress,
            abi: chainsweepAbi,
            client: {
                public: this.publicClient,
                wallet: this.client!!
            }
        });
        return contract;
    }

    getCurrentGame() {
        return readonly(this.currentGame);
    }

    async onConnect() {
        this.client = null;
        this.address.value = null;
        if (window.ethereum != null) {
            this.client = createWalletClient({
                chain: targetChain,
                transport: custom(window.ethereum)
            });
            if (this.client.chain?.id !== targetChain.id) {
                this.client.switchChain(targetChain);
            } else {
                const addresses = await this.client?.getAddresses();
                this.address.value = addresses?.[0] ?? null;
                if (this.address.value) {
                    const result = await this.contract().read.viewFor([this.address.value]);
                    this.onGameUpdate(result);
                }
            }
        }   
    }

    private onGameUpdate(result: string) { 
        if (result.includes('not started')) {
            this.currentGame.value = null;
        }
        const lines = result.trim().split('\n');
        const game = new Game();
        game.state = GameState.PLAYING;
        const lastLine = lines[lines.length - 1];
        if (lastLine.includes('Won')) {
            game.state = GameState.WON;
        } else if (lastLine.includes('Lost')) {
            game.state = GameState.LOST;
        }
        for (let i = 0; i < lines.length - 1; i++) {
            game.field.push(lines[i]);
        }
        this.currentGame.value = game;
    }
}

export const web3Service = new Web3Service();