
import { createPublicClient, createWalletClient, custom, http, type Address, type PublicClient, type WalletClient, type WatchEventReturnType } from 'viem';

import { getContract } from 'viem';
import { chainsweepAbi } from '../src/generated';
import { contractAddress, targetChain } from './constants';

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

    private error: Ref<string | null> = ref(null);

    setError(error: string | null) {
        this.error.value = error;
    }
    getError() {
        return readonly(this.error);
    }

    private fireShouldConnect() {
        this.dispatchEvent(new Event('should-connect'));
    }

    private contract() {
        if (this.address.value == null || this.client == null) {
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

    getAddress() {
        return readonly(this.address);
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
                console.log('getting addresses');
                const addresses = await this.client?.getAddresses();
                this.address.value = addresses?.[0] ?? null;
                console.log('got addresses', addresses);

                if (chainId !== targetChain.id) {
                    console.log('initiating chain switch', chainId, targetChain.id);
                    try {
                        try {
                            await this.client!!.addChain({ chain: targetChain });
                        } catch(e) {
                            // Ignore error if chain already exists
                        }
                        await this.client!!.switchChain(targetChain);
                    } catch (e) {
                        console.error('switch chain error', e);
                        this.setError('Switching chain failed, please switch manually');
                    }
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
                    console.log('watching for events');
                    this.watcher = this.publicClient.watchEvent({
                        address: contractAddress,
                        onLogs: logs => {
                            console.log('Event from contract, updating game state');
                            this.loadGameState();
                        }
                    })
                    this.setError(null);
                   
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
            return;
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

    async clickCell(x: number, y: number) {
        try {
            const gasEstimate = await this.contract()?.estimateGas.makeGuess([x, y], { account: this.client!!.account!!, chain: targetChain });
            if (gasEstimate === undefined) {
                throw new Error('Gas estimation failed');
            }
            const tx = await this.contract()?.write.makeGuess([x, y], { account: this.client!!.account!!, chain: targetChain,
                gas: gasEstimate * 2n + 100_000n
            });
        } catch(e) {
            console.error('makeGuess error', e);
            this.setError('Error: ' + e.message);
            setTimeout(() => this.setError(null), 5000);
        }
    }

    newGame() {
        this.contract()?.write.newGame({ account: this.client!!.account!!, chain: targetChain });
    }
}

export const web3Service = new Web3Service();