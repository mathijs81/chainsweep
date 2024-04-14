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
    <div class="container-md text-center">
        <w3Connect />
        <h1 class="py-3">ChainSweep</h1>
        <p class="fs-4">
            A number of blocks have been mined but some of them have bugs!

            You need to validate all blocks that don't have bugs. A block without bugs will tell you how many blocks
            around it have bugs.
            If you click a block with a bug, the game is over. When you've validated all blocks without bugs, you win!
        </p>
        <GameBoard v-if="currentBoard" :clickEnabled="gameState === GameState.PLAYING" :board="currentBoard"
            @clickCell="click" />
        <div v-if="gameState === GameState.WON" class="alert alert-success" role="alert">
            You won!
        </div>
        <div v-if="gameState === GameState.LOST" class="alert alert-warning" role="alert">
            You lost!
        </div>
        <div v-if="gameState !== GameState.PLAYING" class="text-center">
            <button class="btn btn-primary" @click="newGame()">Start New Game</button>
        </div>

    </div>
</template>

<style lang="scss" scoped></style>