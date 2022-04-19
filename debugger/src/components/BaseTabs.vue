<template>
  <div class="tabs">
    <a
      v-for="item in items"
      :key="item"
      class="tab tab-bordered"
      :class="{ 'tab-active': modelValue === item }"
      @click="$emit('update:modelValue', item)"
    >{{ item }}</a>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  items: string[]
  modelValue: string
}>();
defineEmits(['update:modelValue'])

</script>

<style scoped lang="postcss">
</style>