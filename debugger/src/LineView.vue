<template>
  <canvas ref="canvas" class="w-32" width="200" height="200"></canvas>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { DataObject } from './logic/dataManager';
import { circle } from './logic/canvas';

const props = defineProps<{
  data?: DataObject['line_detections']
}>();

const canvas = ref<HTMLCanvasElement>();
let ctx: CanvasRenderingContext2D | undefined = undefined

const RADIUS = 80;

function rerender() {
  if (!ctx) return;
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.strokeStyle = '#ffffff';
  ctx.lineWidth = 2;

  const cX = ctx.canvas.width / 2
  const cY = ctx.canvas.height / 2

  circle(ctx, ctx.canvas.width / 2, ctx.canvas.height / 2, RADIUS);
  ctx.stroke();
  if (!props.data) return;
  for (let i = 0; i < props.data.length; i++) {
    const offset = (i / props.data.length) * 2 * Math.PI;
    const x = Math.cos(offset) * RADIUS;
    const y = Math.sin(offset) * RADIUS;
    if (props.data[i]) ctx.fillStyle = '#fff67d';
    else ctx.fillStyle = '#000000';
    circle(ctx, cX + x, cY + y, 5);
    ctx.stroke();
    ctx.fill();
  }
  ctx.fillStyle = ''
}

onMounted(() => {
  ctx = canvas.value?.getContext('2d')!;
  rerender();
});

watch(() => props.data, () => {
  rerender();
})
</script>

<style scoped lang="postcss">
</style>