<template>
  <div class="max-w-screen-sm">
    <h1 class="text-balance mb-4 text-slate-700 dark:text-slate-200">Gift a question</h1>
    <p class=" text-start text-slate-700 dark:text-slate-200">As a token of appreciation from the software engineering community for supporting a free educational resource,
      we will add</p>
    <ul class="signup-pitch w-fit mx-auto">
      <li><i class="pi pi-heart"></i>a permanent <em>thank you</em> message</li>
      <li><i class="pi pi-sparkles"></i>to your sponsored questions</li>
      <li><i class="pi pi-desktop"></i>shown on webpages and in emails</li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useMainStore } from '@/store';

const store = useMainStore();

</script>
