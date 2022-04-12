<template>
  <div class="h-full flex">
    <div class="p-4 bg-base-200 flex flex-col gap-2 w-80">
      <p class="text-xl mb-2">Nebula</p>
      <p class="text-lg">Data Source</p>
      <div>
        <base-radio-button
          label="Server"
          @click="source_type = 'server'"
          :checked="source_type === 'server'"
        />
        <base-radio-button
          label="Text"
          @click="source_type = 'text'"
          :checked="source_type === 'text'"
        />
      </div>
      <template v-if="source_type === 'text'">
        <BaseTextField v-model="textData" multiline @input="onStart" placeholder="Paste data in here..." />
      </template>
      <template v-else-if="source_type === 'server'">
        <p class="text-sm">Stream Host</p>
        <BaseTextField placeholder="127.0.0.1" v-model="host" />
        <p class="text-sm">Camera Server Port</p>
        <BaseTextField placeholder=":8000" v-model="camera_port" />
        <p class="text-sm">Controller Port</p>
        <BaseTextField placeholder=":1337" v-model="controller_port" />
      </template>
      <BaseButton @click="() => started ? onStop() : onStart()">{{ started ? 'Stop' : 'Start' }}</BaseButton>
    </div>
    <div class="flex flex-col flex-1">
      <div class="flex items-center gap-4 p-3 bg-base-200">
        <div class="flex">
          <BaseButton class="btn-sm" @click="source?.back()">
            <i-mdi-arrow-left />
          </BaseButton>
          <BaseButton class="btn-sm" @click="source?.next()">
            <i-mdi-arrow-right />
          </BaseButton>
        </div>
        <p class="text-lg text-gray-300">Viewer</p>
        <input
          @input="source?.goTo($event.target.value)"
          type="range"
          min="0"
          :max="frame_count"
          :value="frame_number"
          class="range range-xs"
        />
        <BaseButton class="btn-circle btn-sm text-lg" @click="onClear">
          <i-mdi:delete-empty-outline />
        </BaseButton>
      </div>
      <base-tabs class="w-full bg-base-300" :items="tabs" v-model="active_tab" />
      <div class="w-full relative flex-1 flex-col flex">
        <div
          class="absolute bottom-0 border-4 font-mono text-primary-content border-4"
          :style="{
            color: '#ffff'
          }"
        >
          <div
            class="px-2 py-1 w-full"
            :style="{ backgroundColor: 'rgb(91, 206, 250)' }"
          >{{ frame_number }}</div>
          <div
            class="px-2 py-1 bg-secondary w-full"
            :style="{ backgroundColor: 'rgb(245, 169, 184)' }"
          >
            {{
              frame_count
            }}
          </div>
        </div>
        <img
          class="h-full"
          v-if="active_tab === 'Camera'"
          :src="`http://${host}${camera_port}/stream.mjpg`"
        />
        <LineView v-if="current_frame && active_tab === 'Line'" :data="current_frame" />
      </div>
      <div class="text-xs overflow-y-auto bg-neutral font-mono p-4 h-40">{{ current_frame }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onKeyStroke, useLocalStorage } from '@vueuse/core';
import debounce from 'lodash.debounce';
import { ref } from 'vue';
import BaseButton from './components/BaseButton.vue';
import BaseRadioButton from './components/BaseRadioButton.vue';
import BaseTabs from './components/BaseTabs.vue';
import BaseTextField from './components/BaseTextField.vue';
import LineView from './LineView.vue';
import { DataObject, DataSource, ServerSource, TextSource } from './logic/dataManager';

const host = useLocalStorage("host", "127.0.0.1")
const camera_port = useLocalStorage("camera_port", ref(":8000"))
const controller_port = useLocalStorage("controller_port", ref(":7272"))
const source_type = ref<'server' | 'text'>('server')

let source = ref<DataSource | undefined>(undefined)

const tabs = ['Line', 'Camera']
const active_tab = ref(tabs[0])

const started = ref(false)
const textData = ref("");
const current_frame = ref<DataObject | undefined>(undefined);
const frame_count = ref(0);
const frame_number = ref(0);

const onStart = () => {
  started.value = true
  try {
    if (source_type.value === 'text') {
      source.value = new TextSource(textData.value)
    } else {
      source.value = new ServerSource(`ws://${host.value}${controller_port.value}/state`)
    }
  } catch (e) {
    console.error(e)
  }
  source.value?.onFrame(debounce(frame => {
    current_frame.value = frame
    frame_count.value = source.value?.numFrames() ?? 0
    frame_number.value = source.value?.currentFrame() ?? 0
  }, 20, {
    maxWait: 20
  }))
  source.value?.next();
};

const onStop = () => {
  started.value = false;
  source.value?.stop();
}

const onClear = () => {
  source.value?.clear();
}

onKeyStroke('ArrowLeft', () => source.value?.back());
onKeyStroke('ArrowRight', () => source.value?.next());
</script>

<style scoped lang="postcss">
</style>

