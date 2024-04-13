<script setup lang="ts">
const props = defineProps<{
    board: string[][];
}>();

const width = computed(() => props.board[0].length);
const height = computed(() => props.board.length);
</script>

<template>
    <div class="board d-inline-block">
        <div v-for="(row, y) in props.board" :key="y" class="d-flex gridrow">
            <div v-for="(cell, x) in row" :key="x" class="gridcell" @click="$emit('clickCell', { x, y })">
                {{ cell }}
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
$borderColor: #aaa;
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
    justify-content: center;
    align-items: center;
    &:hover {
        background-color: #eee;
    }
}
</style>