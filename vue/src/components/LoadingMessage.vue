<template>
  <h3 class="my-8 text-slate-500 dark:text-slate-100">{{ msg ? msg : "Loading..." }}</h3>
</template>

<script setup lang="ts">

const props = defineProps<{ msg?: string }>();

</script>