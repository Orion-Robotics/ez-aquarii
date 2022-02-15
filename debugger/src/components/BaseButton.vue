<template>
  <button class="btn">
    <slot />
  </button>
</template>

<script setup lang="ts">
</script>

<style scoped lang="postcss">
.btn {
  @apply rounded-md bg-primary-500 hover:bg-primary-600 transition-colors duration-100 px-3 py-2;
}
</style>