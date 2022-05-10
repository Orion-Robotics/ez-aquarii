<template>
  <label class="label gap-4">
    {{ label }}
    <input type="range" :min="min" :max="max" :step="step" class="range range-xs" v-bind="$attrs" v-model.number="value" />
  </label>
</template>

<script setup lang="ts">
import { useVModel } from '@vueuse/core';

const props = defineProps<{
  label?: string
  max?: number
  min?: number
  step?: number
  modelValue: number
}>();
const emits = defineEmits(["update:modelValue"])

const value = useVModel(props, 'modelValue', emits)
</script>

<style scoped lang="postcss">
</style>