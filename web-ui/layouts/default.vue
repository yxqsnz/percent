<template>
  <link rel="preconnect" href="https://rsms.me/" />
  <link rel="stylesheet" href="https://rsms.me/inter/inter.css" />

  <div
    class="bg-slate-900 transitiona-all duration-300 text-slate-200 font-sans"
  >
    <div class="h-screen flex flex-col items-center justify-center">
      <NavBar />
      <slot />
    </div>
  </div>
</template>
