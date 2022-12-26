<template>
    <nav class="sticky top-0 z-40 backdrop-blur-sm bg-slate-900/10 w-full">
        <div class="p-3">
            <div class="flex justify-between">
                <PageTitle />

                <div class="space-x-4">
                    <NuxtLink href="/newaccount">
                        <ClickableButton text="Create account" />
                    </NuxtLink>
                    <ClickableButton text="Login" />
                </div>

            </div>

        </div>
    </nav>
</template>