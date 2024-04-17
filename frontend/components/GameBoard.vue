<script setup lang="ts">
import { GameState } from '~/services/chain';

const props = defineProps<{
    board: string[][];
    clickEnabled: boolean;
    state: GameState;
}>();
const emit = defineEmits(['clickCell']);

const width = computed(() => props.board[0].length);
const height = computed(() => props.board.length);

function getClass(cell: string) {
    return {
        'unopened': cell === ' ',
        'openable': cell === ' ' && props.clickEnabled,
        'avoided-bug': cell === ' ' && props.state === GameState.WON,
        'bug': cell === 'X',
        ...[...Array(8).keys()].reduce((acc, i) => {
            acc[`around-${i}`] = cell === i.toString();
            return acc;
        }, {} as Record<string, boolean>),
    };
}
function formatCell(cell: String) {
    if (cell === '0') {
        return '';
    }
    if (cell === 'X') {
        return '';
    }
    if (cell === ' ') {
        return '?';
    }
    return cell;
}
function click(x: number, y:number) {
    if (!props.clickEnabled) {
        return;
    }
    const cell = props.board[y][x];
    if (cell !== ' ') {
        return;
    }
    emit('clickCell', { x, y })
}
</script>

<template>
    <div class="board d-inline-block m-3">
        <div v-for="(row, y) in props.board" :key="y" class="d-flex gridrow">
            <div v-for="(cell, x) in row" :key="x" class="gridcell" @click="click(x,y)">
                <div :class="getClass(cell)">
                    {{ formatCell(cell) }}
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
$borderColor: #ccc;
$cellSize: 50px;
.board {
    width: fit-content;
    border-left: 1px solid $borderColor;
    border-bottom: 1px solid $borderColor;
}

.gridrow {
    border-top: 1px solid $borderColor;
}
.gridcell {
    border-right: 1px solid $borderColor;
    width: $cellSize;
    height: $cellSize;
    display: flex;
    align-items: stretch;
    font-weight: bold;

    & div {
      flex-grow: 1;
      line-height: $cellSize;  
    }
}
.unopened {
    background-color: #ddd;
    color: transparent;
    cursor: pointer;
    transition: transform 0.1s ease;
    &.openable:hover {
        font-family: monospace;
        background-color: #eee;
        color: #833;
        z-index: 3;
        transform: scale(1.1);
    }
}
.bug, .avoided-bug {
    &::before {
        content: '';
        display: block;
        width: 100%;
        padding-top: 100%;
        background: url('../img/bug.png');
        background-size: cover;
    }
}
.bug {
    background-color: #fee;
}
.avoided-bug {
    background-color: #efe;
}
</style>