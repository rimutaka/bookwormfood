<template>
  <div>
    <h3 class="mb-4 text-slate-700 dark:text-slate-300 w-full">Get more questions like this to find and fill gaps in your knowledge</h3>

    <div class="flex w-full">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="card flex-grow md:flex-shrink mb-8 text-slate-700 dark:text-slate-200">
        <ul class="signup-pitch">
          <li><i class="pi pi-list-check"></i>questions about your topics of interest</li>
          <li><i class="pi pi-envelope"></i>delivered to your inbox</li>
          <li><i class="pi pi-info-circle"></i>with <em>bite-sized</em> explanations</li>
          <li><i class="pi pi-face-smile"></i>for free</li>
          <li><i class="pi pi-sparkles"></i>to help you become a better dev</li>
        </ul>
      </div>
      <div class="flex-shrink md:flex-grow">
      </div>
    </div>
    <div class="text-center mx-auto">
      <Button label="Choose your topics" icon="pi pi-list-check" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="router.push({ name: 'subscription' })" />
      <p class="text-xs text-center md:mb-auto text-slate-500 dark:text-slate-200">You will be asked to
        <a :href="`/${PageIDs.SUBSCRIPTION}`" @click.prevent="router.push(PageIDs.SUBSCRIPTION)">sign in</a>
        with <i class="pi pi-github ms-1 me-2  dark:opacity-85"></i><i class="pi pi-google me-2  dark:opacity-85"></i>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import router from '@/router';
import { PageIDs } from '@/router';
import Button from 'primevue/button';

</script>