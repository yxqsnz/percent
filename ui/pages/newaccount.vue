<template>
    <form>
        <div class="grid m-12
            rounded-lg gap-6 mb-6 md:grid-cols-2 bg-slate-900 p-8 border-solid border-slate-700 border-2">

            <FormInput placeholder="Nick" type="text" />

            <FormInput placeholder="Password" type="password" />

            <FormInput placeholder="Confirm Password" type="password" />

            <ClickableButton text="Submit" />

        </div>
    </form>
</template>