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
    <div class="container-md text-center">
        <div class="fs-5">
            <p>Our chain has a number of proposed blocks but some of them have bugs!
            </p>
            <p>Please validate blocks by clicking them.</p>
            <p>When you validate a buggy block, you lose!</p>
            <p>A bugfree block that's validated will show you the number of buggy blocks around it.</p>
            <p>When you've validated all bugfree blocks, you win!
            </p>
        </div>
        <ClientOnly><GameBoard v-if="currentBoard" :clickEnabled="gameState === GameState.PLAYING" :board="currentBoard"
            :state="gameState"
            @clickCell="click" />
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
</template>

<style lang="scss" scoped>
.game-result {
    margin: 1rem auto;
    max-width: 30rem;
}
</style>
