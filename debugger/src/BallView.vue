<template>
  <div class="w-80 z-1">
    <BaseSlider label="Radius" v-model.number="radius" :min="0" :max="400" />
  </div>
  <div class="flex-1 absolute top-0 left-0 w-full h-full z-0">
    <canvas ref="canvas" />
  </div>
</template>

<script setup lang="ts">
import { useLocalStorage } from "@vueuse/core";
import { onMounted, ref, watch } from "vue";
import BaseSlider from "./components/BaseSlider.vue";
import { circle } from "./logic/canvas";
import { DataObject } from "./logic/dataSources";

const props = defineProps<{
  data?: DataObject;
}>();

const radius = useLocalStorage("radius", 100);
const line_size = useLocalStorage("line_size", 0.8);

const canvas = ref<HTMLCanvasElement>();
let ctx: CanvasRenderingContext2D | undefined = undefined;

function rerender() {
  const RADIUS = radius.value;
  const LINE_SIZE = line_size.value;

  if (!ctx) return;
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.strokeStyle = "#ffffff";
  ctx.lineWidth = 2;

  const cX = ctx.canvas.width / 2;
  const cY = ctx.canvas.height / 2;

  circle(ctx, ctx.canvas.width / 2, ctx.canvas.height / 2, RADIUS);
  ctx.stroke();
  if (!props.data) return;
  const ball_follow_vec = props.data.ball_follow_vector;
  const orbit_offset = props.data.orbit_offset;
  const dampen_amount = props.data.dampen_amount;

  const draw_vector = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    color: string,
    length: number
  ) => {
    x = x * RADIUS * LINE_SIZE * length;
    y = y * RADIUS * LINE_SIZE * length;
    ctx.beginPath();
    ctx.strokeStyle = color;
    ctx.moveTo(cX, cY);
    ctx.lineTo(cX + x, cY - y);
    ctx.stroke();
  };

  if (ball_follow_vec)
    draw_vector(ctx, ball_follow_vec.x, ball_follow_vec.y, "#eb4034", 1);

  draw_vector(
    ctx,
    Math.cos(orbit_offset),
    Math.sin(orbit_offset),
    "#00ff00",
    0.5
  );

  draw_vector(
    ctx,
    Math.cos(orbit_offset + dampen_amount),
    Math.sin(orbit_offset + dampen_amount),
    "#106080",
    1
  );

  // if (previous_vec)
  //   draw_vector(ctx, previous_vec.x, previous_vec.y, "#eb6090", 0.5);

  ctx.fillStyle = "";
}

onMounted(() => {
  ctx = canvas.value!.getContext("2d")!;
  ctx.canvas.height = canvas.value!.parentElement!.clientHeight;
  ctx.canvas.width = canvas.value!.parentElement!.clientWidth;
  rerender();
});

watch([() => props.data, radius, line_size], () => {
  requestAnimationFrame(rerender);
});
</script>

<style scoped lang="postcss"></style>
