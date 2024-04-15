
import { createPublicClient, createWalletClient, custom, defineChain, http, type Address, type Client, type PublicClient, type WalletClient, type WatchEventReturnType } from 'viem';

import { getContract } from 'viem';
import { chainsweepAbi } from '../src/generated';

const contractAddress = '0xd9140951d8aE6E5F625a02F5908535e16e3af964';

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

class Web3Service extends EventTarget {
    private publicClient: PublicClient = createPublicClient({
        chain: targetChain,
        transport: http()
    });
    private client: WalletClient | null = null;
    private address: Ref<Address | null> = ref(null);
    private currentGame: Ref<Game | null> = ref(null);
    //private watcher: WatchContractEventReturnType | null = null;
    private watcher: WatchEventReturnType | null = null;

    private fireShouldConnect() {
        this.dispatchEvent(new Event('should-connect'));
    }

    private contract() {
        if (this.client == null) {
            this.fireShouldConnect();
            return null;
        }
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

    async onConnect(chainId: number) {
        this.client = null;
        this.address.value = null;
        if (window.ethereum != null) {
            // window.ethereum.enable();
            const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
            const account = accounts[0] as Address;
            console.log('got account', account);
            // defer next step for a second, seems to work better with switch chain
            setTimeout(async () => {
                this.client = createWalletClient({
                    chain: targetChain,
                    transport: custom(window.ethereum!!),
                    account,
                });
                if (chainId !== targetChain.id) {
                    this.client!!.switchChain(targetChain);
                } else {
                    if (this.watcher != null) {
                        this.watcher();
                    }
                    // TODO: fix watching for proper contract events, currently not possible
                    // because `cargo stylus export-abi` doesn't export the event data
                    // this.watcher = this.publicClient.watchContractEvent({
                    //     address: contractAddress,
                    //     abi: chainsweepAbi,
                    //     onLogs: logs => console.log(logs),
                    //     pollingInterval: 1000
                    // })
                    this.watcher = this.publicClient.watchEvent({
                        address: contractAddress,
                        onLogs: logs => {
                            console.log('Event from contract, updating game state');
                            this.loadGameState();
                        }
                    })
                    const addresses = await this.client?.getAddresses();
                    this.address.value = addresses?.[0] ?? null;
                    console.log('got addresses', addresses);
                    if (this.address.value) {
                        await this.loadGameState();
                    }
                }
            }, 1000);
        }
    }

    private async loadGameState() {
        const result = await this.contract()?.read?.viewFor([this.address.value!!]);
        if (result != null) {
            console.log('viewFor result:', result);
            this.onGameUpdate(result);
        }
    }

    private onGameUpdate(result: string) {
        if (result.includes('not started')) {
            this.currentGame.value = null;
        }
        const lines = result.trimEnd().split('\n');
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

    clickCell(x: number, y: number) {
        this.contract()?.write.makeGuess([x, y], { account: this.client!!.account!!, chain: targetChain }).then(result => {
            console.log('Click on cell', x, y, 'result:', result);
        });
    }

    newGame() {
        this.contract()?.write.newGame({ account: this.client!!.account!!, chain: targetChain });
    }
}

export const web3Service = new Web3Service();