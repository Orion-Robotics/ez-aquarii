<template>
  <div class="form-control">
    <label class="label cursor-pointer justify-start gap-4">
      <input type="radio" name="radio-6" class="radio checked:bg-red-500" :checked="checked" />
      <span class="label-text">{{ label }}</span>
    </label>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  label: string;
  checked: boolean;
}>()
</script>