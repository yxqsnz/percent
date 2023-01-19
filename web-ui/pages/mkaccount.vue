<template>
  <form>
    <div
      class="grid place-items-stretch gap-6 m-4 rounded-lg bg-slate-900 p-8 border-solid border-slate-700 border-2"
    >
      <h1 className="text-slate-200 font-medium">Account creation</h1>

      <RegisterField
        type="text"
        placeholder="Your nickname"
        min-length="2"
        max-length="16"
        v-model:value="nick"
      />

      <RegisterField
        type="password"
        placeholder="Your password"
        min-length="2"
        max-length="72"
        v-model:value="password"
      />

      <YoButton type="submit"> Create your account </YoButton>
    </div>
  </form>
</template>

<script setup>
const nick = ref("");
const password = ref("");
</script>
