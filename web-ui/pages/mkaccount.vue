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
      />
      <RegisterField
        type="password"
        placeholder="Your password"
        min-length="2"
        max-length="72"
      />
      <YoButton type="submit"> Create your account </YoButton>
    </div>
  </form>
</template>
