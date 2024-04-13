<script setup lang="ts">

import { web3Service, GameState } from '~/services/chain';

const currentBoard = computed(() => {
    const game = web3Service.getCurrentGame().value;
    if (game == null || game.state == GameState.UNSTARTED) {
        return null;
    }
    return game.field.map(row => [...row]);
});

function click(data: { x: number, y: number }) {
    //console.log(`Clicked at ${data.x}, ${data.y}`);
    web3Service.clickCell(data.x, data.y);
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
        <GameBoard v-if="currentBoard" :board="currentBoard" @clickCell="click" />
    </div>
</template>

<style lang="scss" scoped></style>