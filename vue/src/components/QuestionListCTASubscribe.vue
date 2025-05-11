<template>
  <div>
    <h3 class="mb-4 text-slate-700 dark:text-slate-300 w-full">Question fatigue</h3>
    <p class="mb-4 text-start max-w-md">You will learn more and remember it better if you answer only one or two questions per day.</p>
    <p class="mb-4 text-start max-w-md">Subscribe to new questions about your topics of interest to be delivered to your inbox:</p>
    <ul class="signup-pitch mb-8 ">
      <li><i class="pi pi-sparkles"></i>learn in small bites</li>
      <li><i class="pi pi-clock"></i>when you have a spare moment</li>
      <li><i class="pi pi-spinner-dotted"></i>or need a little distraction</li>
    </ul>
    <div class="text-center mx-auto">
      <Button label="Choose your subscription topics" icon="pi pi-list-check" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="router.push({ name: PageIDs.SUBSCRIPTION })" />
    </div>
  </div>
</template>

<script setup lang="ts">
import router from '@/router';
import { PageIDs } from '@/router';
import Button from 'primevue/button';

</script>