<template>
    <form>
        <div
            class="md:flex-1 md:items-center md:justify-center md:grid m-4 rounded-lg gap-6 bg-slate-900 p-8 border-solid border-slate-700 border-2">
            <h1 class="text-slate-200 text-center">Account creation</h1>

            <FormInput placeholder="Nick" min=2 max=16 type="text" />
            <FormInput placeholder="Password" min=8 max=72 type="password" />
            <FormInput placeholder="Confirm Password" min=8 max=72 type="password" />
            <ClickableButton text="Submit" />
        </div>
    </form>

</template>