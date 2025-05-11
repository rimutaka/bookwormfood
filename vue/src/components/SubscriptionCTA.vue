<template>
  <div>
    <div class="flex">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="card flex-grow md:flex-shrink cta-box">
        <h3 class="text-justify md:text-center text-slate-700 dark:text-slate-200">Your free subscription to getting 1% better every day</h3>
        <ul class="signup-pitch">
          <li><i class="pi pi-question-circle"></i>one question with detailed answers</li>
          <li><i class="pi pi-envelope"></i>emailed daily to <strong>{{ store.email }}</strong> </li>
          <li><i class="pi pi-pause-circle"></i>auto-paused if you get a backlog of questions</li>
          <li><i class="pi pi-ban"></i>one-click unsubscribe</li>
        </ul>
      </div>
      <div class="flex-shrink md:flex-grow">
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMainStore } from '@/store';

const store = useMainStore();

</script>
