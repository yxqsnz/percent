<template>
  <button
    :class="`bg-emerald-200
    border-slate-700
    border-solid
    border-2
    rounded-lg
    p-2
    text-slate-900
    outline-none
    outline-0
    transition-colors
    duration-500
    ${props.moreClasses}`"
    v-bind="$attrs"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
interface Props {
  moreClasses?: string;
}

const props = defineProps<Props>();
</script>
