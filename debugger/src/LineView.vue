<template>
  <div class="w-80">
    <BaseSlider label="Radius" v-model.number="radius" :min="0" :max="400" />
    <BaseSlider label="Line Size" v-model.number="line_size" :min="0" :max="1" :step="0.01" />
    <BaseSlider label="Magnitude Radius" v-model.number="magnitude_radius" :min="0" :max="10" :step="0.01" />
    <BaseSlider label="Magnitude Threshold" v-model.number="magnitude_threshold" :min="1" :max="255" :step="1" />
  </div>
  <div class="flex-1">
    <canvas ref="canvas" />
  </div>
</template>

<script setup lang="ts">
import { useLocalStorage } from '@vueuse/core';
import { onMounted, ref, watch } from 'vue';
import BaseSlider from './components/BaseSlider.vue';
import { circle } from './logic/canvas';
import { DataObject } from './logic/dataSources';

const props = defineProps<{
  data?: DataObject
}>();

const radius = useLocalStorage('radius', 100);
const line_size = useLocalStorage('line_size', 0.8);
const magnitude_radius = useLocalStorage('magnitude_radius', 5);
const magnitude_threshold = useLocalStorage('magnitude_threshold', 128);

const canvas = ref<HTMLCanvasElement>();
let ctx: CanvasRenderingContext2D | undefined = undefined

function rerender() {
  const RADIUS = radius.value;
  const LINE_SIZE = line_size.value;
  const SENSOR_SIZE = RADIUS * 0.03;

  if (!ctx) return;
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.strokeStyle = '#ffffff';
  ctx.lineWidth = 2;

  const cX = ctx.canvas.width / 2
  const cY = ctx.canvas.height / 2

  circle(ctx, ctx.canvas.width / 2, ctx.canvas.height / 2, RADIUS);
  ctx.stroke();
  if (!props.data) return;
  const line_detections = props.data.line_detections;
  const line_vector = props.data.line_vector;

  {
    const x = line_vector.x * RADIUS * LINE_SIZE;
    const y = line_vector.y * RADIUS * LINE_SIZE;

    ctx.beginPath();
    ctx.strokeStyle = '#eb4034';
    ctx.moveTo(cX, cY);
    ctx.lineTo(cX + x, cY + y);
    ctx.stroke();
  }

  for (let i = 0; i < line_detections.length; i++) {
    const offset = (i / line_detections.length) * 2 * Math.PI;
    const x = cX + Math.cos(offset) * RADIUS;
    const y = cY + Math.sin(offset) * RADIUS;
    const magnitude = props.data.data.sensor_data[i] / magnitude_threshold.value;
    ctx.fillStyle = `rgba(${128 * magnitude}, ${50 * magnitude}, ${50 * magnitude}, ${magnitude / 3})`;
    circle(ctx, x, y, SENSOR_SIZE * magnitude_radius.value);
    ctx.fill();
    if (line_detections[i]) ctx.fillStyle = '#eb4034';
    else ctx.fillStyle = '#787878';
    circle(ctx, x, y, SENSOR_SIZE);
    ctx.fill();
  }
  ctx.fillStyle = ''
}

onMounted(() => {
  ctx = canvas.value!.getContext('2d')!;
  ctx.canvas.height = canvas.value!.parentElement!.clientHeight
  ctx.canvas.width = canvas.value!.parentElement!.clientWidth
  rerender();
});

watch([() => props.data, radius, line_size, magnitude_radius, magnitude_threshold], () => {
  requestAnimationFrame(rerender);
})
</script>

<style scoped lang="postcss">
</style>