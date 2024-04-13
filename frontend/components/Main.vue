<script setup lang="ts">

const currentBoard = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', 'X', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', 'X', ' ', 'X', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
]

function click(data: {x: number, y: number}) {
    console.log(`Clicked at ${data.x}, ${data.y}`);
}
</script>

<template>
    <div class="container-md text-center">
        <h1 class="py-3">ChainSweep</h1>
        <p class="fs-4">
            A number of blocks have been mined but some of them have bugs!

            You need to validate all blocks that don't have bugs. A block without bugs will tell you how many blocks around it have bugs.
            If you click a block with a bug, the game is over. When you've validated all blocks without bugs, you win!
        </p>
        <GameBoard :board="currentBoard" @clickCell="click"/>
    </div>
</template>

<style lang="scss" scoped></style>