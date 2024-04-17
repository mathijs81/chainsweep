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
                <ClientOnly>
                    <GameBoard v-if="currentBoard && gameState" :clickEnabled="gameState === GameState.PLAYING" :board="currentBoard"
                        :state="gameState" @clickCell="click" />
                </ClientOnly>
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
    </div>
</template>

<style lang="scss" scoped>
.game-result {
    margin: 1rem auto;
    max-width: 30rem;
}
</style>
