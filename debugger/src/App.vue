<template>
  <div class="h-full p-4 flex flex-col gap-2">
    <p class="text-xl">Robotics data viewer</p>
    <div class="p-4 bg-dark-800 flex flex-col gap-2">
      <p class="text-lg text-gray-300">Data Source</p>
      <p class="text-sm text-gray-300">Text Data</p>
      <BaseTextField
        v-model="textData"
        multiline
        class="w-full"
        placeholder="Paste data in here..."
      />
      <hr />
      <div class="w-min">
        <p class="text-sm text-gray-300">Stream Host</p>
        <BaseTextField placeholder="127.0.0.1" v-model="host" />
        <p class="text-sm text-gray-300">Camera Server Port</p>
        <BaseTextField placeholder=":8000" v-model="camera_port" />
        <p class="text-sm text-gray-300">Controller Port</p>
        <BaseTextField placeholder=":1337" v-model="controller_port" />
      </div>
      <BaseButton class="w-min" @click="onStart">Start</BaseButton>
    </div>
    <div class="p-4 bg-dark-800 flex flex-col gap-2">
      <p class="text-lg text-gray-300">Viewer</p>
      <div v-if="started">
        <img :src="'http://' + host + camera_port + '/stream.mjpg'" />
        <LineView v-if="currentFrame" :data="currentFrame?.line_detections" />
      </div>
      <div
        class="h-60 text-xs overflow-y-auto bg-dark-900 rounded-lg font-mono p-4"
      >{{ currentFrame }}</div>
      <div class="flex justify-between">
        <BaseButton @click="source?.back()">Prev</BaseButton>
        <BaseButton @click="source?.next()">Next</BaseButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onKeyStroke } from '@vueuse/core';
import { computed, ref } from 'vue';
import BaseButton from './components/BaseButton.vue';
import BaseTextField from './components/BaseTextField.vue';
import LineView from './LineView.vue';
import { DataSource, TextSource } from './logic/dataManager';

const host = ref("127.0.0.1")
const camera_port = ref(":8000")
const controller_port = ref(":1337")

let source = ref<DataSource | undefined>(undefined)

const started = ref(false)
const textData = ref("");
const currentFrame = computed(() => source.value?.current());

const onStart = () => {
  started.value = true
  try {
    source.value = new TextSource(textData.value)
  } catch (e) {
    console.error(e)
  }
};
onKeyStroke('ArrowLeft', () => source.value?.back());
onKeyStroke('ArrowRight', () => source.value?.next());
</script>

<style scoped lang="postcss">
</style>

