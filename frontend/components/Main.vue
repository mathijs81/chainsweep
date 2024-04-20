<script setup lang="ts">

import { web3Service, GameState } from '~/services/chain';

const currentBoard = computed(() => {
    const game = web3Service.getCurrentGame().value;
    if (game == null || game.state == GameState.UNSTARTED) {
        return null;
    }
    return game.field.map(row => [...row]);
});

const gameState = computed(() => {
    return web3Service.getCurrentGame().value?.state;
});

function click(data: { x: number, y: number }) {
    //console.log(`Clicked at ${data.x}, ${data.y}`);
    web3Service.clickCell(data.x, data.y);
}
function newGame() {
    web3Service.newGame();
}

const recentGames = web3Service.getRecentGames();
const formatAddress = (address: string) => `${address.slice(0, 6)}...${address.slice(-4)}`;
const address = web3Service.getAddress();
</script>

<template>
    <Header></Header>
    <div class="container-md">
        <div class="row">
            <div class="col-12 col-md-4">
                <h2 class="fs-4 mt-3 mb-1">How to play</h2>
                <p>
                    Our chain has a number of proposed blocks but some of them have bugs!<br>
                    Please validate blocks by clicking them.<br>
                    When you validate a buggy block, you lose!<br>
                    A bugfree block that's validated will show you the number of buggy blocks around it.<br>
                    When you've validated all bugfree blocks, you win!
                </p>

                <h2 class="fs-4 mt-3 mb-1">About this game</h2>
                <p>
                    This game was developed using <a href="https://arbitrum.io/stylus" target="_blank">Arbitrum Stylus</a> for the
                    <a href="https://ethglobal.com/events/scaling2024" target="_blank">Scaling Ethereum 2024 hackathon</a>.<br>
                    The source code is available on <a href="https://github.com/mathijs81/chainsweep" target="_blank">GitHub</a>.
                </p>
            </div>
            <div class="col col-md-8 text-center mt-3">
                <template v-if="currentBoard && gameState">
                    <ClientOnly>
                        <GameBoard :clickEnabled="gameState === GameState.PLAYING" :board="currentBoard"
                            :state="gameState" @clickCell="click" class="m-3" />
                    </ClientOnly>
                </template>
                <img v-else class="my-3" src="../img/example.png" alt="Example game">
                <div v-if="gameState === GameState.WON" class="alert alert-success game-result" role="alert">
                    You won!
                </div>
                <div v-if="gameState === GameState.LOST" class="alert alert-warning game-result" role="alert">
                    You lost!
                </div>
                <div v-if="gameState !== GameState.PLAYING" class="text-center">
                    <button class="btn btn-primary" @click="newGame()">Start New Game</button>
                </div>
            </div>

        </div>
        <div class="row">
            <div class="col">
                <h2 class="fs-4 mt-3 mb-1">Recent games</h2>
                <div class="d-flex gap-2">
                    <div v-for="(game, index) in recentGames" :key="index" class="text-center">
                        <div>{{ formatAddress(game.player) }}<span class="badge text-bg-success ms-2" v-if="game.player == address">YOU</span></div>
                        <div>@ block {{ game.lastChange }}</div>
                        <div>
                            <div v-if="game.game.state === GameState.WON" class="text-success" role="alert">
                                Won</div>
                            <div v-else-if="game.game.state === GameState.LOST" class="text-danger" role="alert">
                                Lost</div>
                                <div v-else>In progress</div>
                        </div>
                        <GameBoard :clickEnabled="false" :board="game.game.field.map(row => [...row])" :state="game.game.state" class="m-0" />
                    </div>  
                </div>
            </div>
        </div>  
    </div>
</template>

<style lang="scss" scoped>
.game-result {
    margin: 1rem auto;
    max-width: 30rem;
}
</style>
