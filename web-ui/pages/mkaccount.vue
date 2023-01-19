<template>
  <form
    class="grid place-items-stretch gap-6 m-4 rounded-lg bg-slate-900 p-8 border-solid border-slate-700 border-2"
  >
    <h1 className="text-slate-200 font-medium">
      Account creation

      <ul className="list-disc text-red-500">
        <li v-if="validations.invalidNickname">
          Nick must be at least 2 and smaller than 16 characters.
        </li>

        <li v-if="validations.invalidPassword">
          Password must be at least 8 and smaller than 72 characters.
        </li>
      </ul>
    </h1>

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

    <YoButton
      type="submit"
      :moreClasses="moreClasses"
      @click="onSubmit"
      :disabled="!buttonEnabled"
    >
      <span v-if="validations.passed"> Create your account </span>
      <span v-else> ❌ </span>
    </YoButton>
  </form>
</template>

<script setup lang="ts">
const nick = ref("");
const password = ref("");
const buttonEnabled = ref(false);
const moreClasses = ref("");
const validations = {
  invalidPassword: true,
  invalidNickname: true,
  passed: false,
};

const onSubmit = (event: FormDataEvent) => {
  event.preventDefault();
  moreClasses.value = "cursor-wait";
  setTimeout(() => {
    moreClasses.value = "";
    const router = useRouter();
    router.push("/");
  }, 1500);
};

watchEffect(() => {
  const pw = password.value;
  const nk = nick.value;

  validations.invalidPassword = !(pw.length >= 8 && pw.length <= 72);
  validations.invalidNickname = !(nk.length >= 2 && nk.length <= 16);

  if (validations.invalidNickname || validations.invalidPassword) {
    moreClasses.value = "cursor-not-allowed bg-emerald-600 text-red-500";
    buttonEnabled.value = false;
    validations.passed = false;
  } else {
    moreClasses.value = "";
    validations.passed = true;
    buttonEnabled.value = true;
  }
});
</script>
