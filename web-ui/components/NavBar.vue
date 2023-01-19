<template>
  <nav class="absolute top-0 backdrop-blur-sm bg-slate-900 w-full">
    <div class="p-3">
      <div class="flex justify-between">
        <NavTitle />
        <div class="space-x-4">
          <NuxtLink href="/mkaccount">
            <YoButton> Create Account </YoButton>
          </NuxtLink>
        </div>
      </div>
    </div>
  </nav>
</template>
