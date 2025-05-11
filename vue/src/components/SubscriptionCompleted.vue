<template>
  <div class="card flex-grow md:flex-shrink cta-box">
    <h3 class="text-center text-slate-700 dark:text-slate-200">About your subscription</h3>
    <ul class="signup-pitch">
      <li><i class="pi pi-envelope"></i>Bite-Sized sends one email per day to <strong>{{ store.email }}</strong> with a random question about one of your selected topics.</li>
      <li><i class="pi pi-clock"></i>The next email is scheduled for the following day when you answer the previous question.</li>
      <li><i class="pi pi-pause-circle"></i>Emails are automatically paused if you stop answering questions to minimize your backlog.</li>
      <li><i class="pi pi-ban"></i>You can change your topics on this page and every email has a one-click unsubscribe link.</li>
    </ul>
    <p class="mt-8">Thank you for subscribing and happy learning!</p>
    <SignatureForCTAs />
  </div>
</template>

<script setup lang="ts">
import { useMainStore } from '@/store';
import SignatureForCTAs from './SignatureForCTAs.vue';

const store = useMainStore();

</script>
