<template>
  <textarea
    :value="modelValue"
    @input="$emit('update:modelValue', $event.target?.value)"
    v-bind="$attrs"
    v-if="multiline"
  />
  <input
    @input="$emit('update:modelValue', $event.target?.value)"
    :value="modelValue"
    v-bind="$attrs"
    v-else
  />
</template>

<script lang="ts" setup>
defineProps<{
  multiline?: boolean
  modelValue?: string
}>()
defineEmits(['update:modelValue'])
</script>

<style lang="postcss" scoped>
textarea,
input {
  @apply input input-secondary;
}

textarea {
  @apply textarea textarea-bordered input-secondary;
}
</style>