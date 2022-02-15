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
      <p class="text-sm text-gray-300">Stream</p>
      <div class="flex gap-3">
        <BaseTextField placeholder="127.0.0.1:1337" />
      </div>
      <BaseButton class="w-min" @click="onStart">Start</BaseButton>
    </div>
    <div class="p-4 bg-dark-800 flex flex-col gap-2">
      <p class="text-lg text-gray-300">Viewer</p>
      <div>
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
import { computed, ref } from 'vue';
import BaseButton from './components/BaseButton.vue';
import BaseTextField from './components/BaseTextField.vue';
import { TextSource, DataSource, DataObject } from './logic/dataManager';
import LineView from './LineView.vue';
import { onKeyStroke } from '@vueuse/core'

let source = ref<DataSource | undefined>(undefined)

const textData = ref("");
const currentFrame = computed(() => source.value?.current());

const onStart = () => source.value = new TextSource(textData.value);
onKeyStroke('ArrowLeft', () => source.value?.back());
onKeyStroke('ArrowRight', () => source.value?.next());

</script>

<style scoped lang="postcss">
</style>

