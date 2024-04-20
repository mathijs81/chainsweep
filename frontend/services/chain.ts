
import { createPublicClient, createWalletClient, custom, http, parseAbi, type Address, type PublicClient, type WalletClient, type WatchContractEventReturnType } from 'viem';

import { getContract } from 'viem';
import { chainsweepAbi } from '../src/generated';
import { contractAddress, targetChain } from './constants';

// Copying the events here because `cargo stylus export-abi` doesn't export the event data   
const eventAbi = parseAbi(['event GameStarted(address indexed player)',
    `event FieldOpened(address indexed player, uint8 x, uint8 y, uint8 value)`,
    `event GameOver(address indexed player, bool won)`]);

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

export interface RecentGame {
    game: Game;
    lastChange: number;
    player: Address;
}

class Web3Service extends EventTarget {
    private publicClient: PublicClient = createPublicClient({
        chain: targetChain,
        transport: http()
    });
    private client: WalletClient | null = null;
    private address: Ref<Address | null> = ref(null);
    private currentGame: Ref<Game | null> = ref(null);
    private recentGames: Ref<RecentGame[]> = ref([]);

    //private watcher: WatchContractEventReturnType | null = null;
    private watcher: WatchContractEventReturnType | null = null;

    private error: Ref<string | null> = ref(null);

    constructor() {
        super();
        if (this.watcher != null) {
            this.watcher();
        }

        this.publicClient.getBlockNumber().then(async blockNumber => {
            try {
                const logs = await this.publicClient.getContractEvents({
                    address: contractAddress,
                    abi: eventAbi,
                    fromBlock: blockNumber > 100_000n ? blockNumber - 100_000n : 1n
                });
                const lastAction = new Map<Address, number>();
                for (const log of logs) {
                    if (log.eventName === 'FieldOpened') {
                        lastAction.set(log.args['player']!!, Number(log.blockNumber));
                    }
                }

                // Select the most recently active addresses
                const sorted = Array.from(lastAction.entries()).sort((a, b) => b[1] - a[1]);
                const recent = sorted.slice(0, 10);
                const recentGames = await Promise.all(recent.map(async ([player, blockNumber]) => {
                    const result = await this.publicContract().read?.viewFor([player]);
                    if (result != null) {
                        return { game: this.parseGameState(result), lastChange: blockNumber, player };
                    } else {
                        console.log("got null for", player);
                    }
                    return null;
                }));
                this.recentGames.value = recentGames.filter(g => g != null) as RecentGame[];
                console.log(this.recentGames.value.length + ' recent games loaded');
            } catch (e) {
                console.error('error getting historic events', e);
            }
            this.publicClient.watchContractEvent({
                address: contractAddress,
                abi: eventAbi,
                onLogs: logs => {
                    logs.forEach(async log => {
                        let player = (log.args as { player: Address})['player'];
                        if (this.address.value === player) {
                            this.loadGameState();
                        }

                        const blockNumber = Number(log.blockNumber);
                        const gameBoard = await this.publicContract().read?.viewFor([player]);
                        const recentGame = { game: this.parseGameState(gameBoard), lastChange: blockNumber, player };
                        this.recentGames.value = [recentGame, ...this.recentGames.value.filter(g => g.player !== player)].slice(0, 5);
                    })
                },
                pollingInterval: 1000
            })
        });
    }


    setError(error: string | null) {
        this.error.value = error;
    }
    getError() {
        return readonly(this.error);
    }

    getRecentGames() {
        return readonly(this.recentGames);
    }

    private fireShouldConnect() {
        this.dispatchEvent(new Event('should-connect'));
    }


    private publicContract() {
        return getContract({
            address: contractAddress,
            abi: chainsweepAbi,
            client: this.publicClient,
        });
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
                        } catch (e) {
                            // Ignore error if chain already exists
                        }
                        await this.client!!.switchChain(targetChain);
                    } catch (e) {
                        console.error('switch chain error', e);
                        this.setError('Switching chain failed, please switch manually');
                    }
                } else {
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

    private parseGameState(result: string): Game {
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
        return game;
    }

    private onGameUpdate(result: string) {
        if (result.includes('not started')) {
            this.currentGame.value = null;
            return;
        }    
        this.currentGame.value = this.parseGameState(result);
    }

    async clickCell(x: number, y: number) {
        try {
            const gasEstimate = await this.contract()?.estimateGas.makeGuess([x, y], { account: this.client!!.account!! });
            if (gasEstimate === undefined) {
                throw new Error('Gas estimation failed');
            }
            const tx = await this.contract()?.write.makeGuess([x, y], {
                account: this.client!!.account!!, chain: targetChain,
                gas: gasEstimate * 2n + 100_000n
            });
        } catch (e) {
            console.error('makeGuess error', e);
            this.setError('Error: ' + (e as any).message);
            setTimeout(() => this.setError(null), 5000);
        }
    }

    async simulate() {
        let result = [];
        for (let i = 0; i < 5; i++) {
            // bigint random u64:
            const seed = BigInt(Math.floor(Math.random() * 2**64));
            const game = await this.contract()?.read?.viewCompleted([this.address.value!!, seed]);
            if (!game) {
                continue;
            }
            result.push(this.parseGameState(game));
        }
        return result;
    }

    newGame() {
        this.contract()?.write.newGame({ account: this.client!!.account!!, chain: targetChain });
    }
}

export const web3Service = new Web3Service();