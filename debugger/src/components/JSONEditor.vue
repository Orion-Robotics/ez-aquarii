<template>
  <h1>
    {{ props.label }}
  </h1>
  <div class="p-2" :class="{ 'ml-2': !root }">
    <template v-for="(entry, name) of modelValue" :key="name">
      <template v-if="typeof entry === 'object'" :config="value">
        <template v-if="Array.isArray(entry)">
          <JSONEditor
            :settings="settings?.[name]"
            v-for="(element, i) in entry"
            e-else
            v-model="(entry[i] as JSONObject)"
            :label="name.toString()"
          />
        </template>
        <JSONEditor
          v-else
          v-model="modelValue[name]"
          :settings="settings?.[name]"
          :label="name.toString()"
        />
      </template>
      <div class="flex gap-3 items-center w-full" v-else>
        <span>{{ name }}</span>
        <template v-if="typeof entry === 'number'">
          <BaseSlider
            :step="settings?.[name]?.step || 0.01"
            :min="settings?.[name]!.min! || 0"
            :max="settings?.[name]!.max! || 100"
            v-model.number="modelValue[name]"
          />
          <input
            :step="settings?.[name]?.step || 0.01"
            :min="settings?.[name]!.min! || 0"
            :max="settings?.[name]!.max! || 100"
            type="number"
            v-model.number="modelValue[name]"
            class="w-fit bg-transparent focus:bg-primary/5"
          />
        </template>
        <BaseTextField
          v-else-if="typeof entry === 'string'"
          v-model="modelValue[name]"
        />
        <input
          v-model.boolean="modelValue[name]"
          v-else-if="typeof entry === 'boolean'"
          type="checkbox"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useVModel } from "@vueuse/core";
import { reactive } from "vue";
import type { JSONObject } from "../logic/dataSources";
import BaseTextField from "./BaseTextField.vue";

type EditorSettings = {
  [key: string]:
    | {
        min?: number;
        max?: number;
        step?: number;
      }
    | EditorSettings
    | undefined;
};

const props = defineProps<{
  modelValue: JSONObject;
  settings?: EditorSettings;
  label: string;
  root?: boolean;
}>();
const emits = defineEmits(["update:modelValue"]);
const ranges = reactive<Record<string | number, number | undefined>>({});

const getRange = (name: string | number, initial: number) => {
  if (!ranges[name])
    ranges[name] = Math.pow(10, Math.ceil(Math.log10(initial))) || 1;
  return ranges[name]!;
};

const modelValue = useVModel(props, "modelValue", emits);
</script>

<style scoped lang="postcss"></style>
