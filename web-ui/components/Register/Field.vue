<script>
export default {
  props: {
    value: {
      type: String,
      default: "",
      required: true,
    },
  },
};
</script>

<template>
  <input
    class="rounded-lg p-2 bg-slate-800 text-slate-200 border border-slate-700 focus:border-blue-500 focus:ring-blue-500 block w-full outline-0 outline-none"
    required
    v-bind="$attrs"
    :value="value"
    @input="$emit('update:value', $event.target.value)"
  />
</template>
