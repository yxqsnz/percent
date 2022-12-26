<template>
    <div>
        <input
            class="rounded-lg p-2 bg-slate-800 text-slate-200 border border-slate-700 focus:border-blue-500 focus:ring-blue-500 block w-full outline-0"
            :type="type" :id="id" :name="name" :placeholder="placeholder" required />
    </div>
</template>


<script>
export default {
    props: [`type`, `id`, `name`, `placeholder`]
}
</script>