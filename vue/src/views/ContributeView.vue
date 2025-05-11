<template>
  <h1 class="mb-4 md:mb-12 text-slate-700 dark:text-neutral-200 w-full">Let's make this free learning tool bigger and better</h1>

  <div class="contribute-card">
    <div class="abstract text-neutral-300 dark:text-neutral-200 ">
      <i class="pi pi-pen-to-square"></i>
    </div>
    <h3>Write a question</h3>
    <p>You will be surprised how much you can learn by writing questions,
      exploring possible answers and making sure the explanations will withstand the scrutiny of your peers.</p>
    <p>Give it a try. Our moderators will help you along the way.</p>
    <p class="cta text-sm">
      <LinkButton :href="PageIDs.ADD"  label="Write a draft question" icon="pi pi-pen-to-square" />
    </p>
  </div>

  <div class="contribute-card">
    <div class="abstract text-amber-600 dark:text-amber-200 ">
      <i class="pi pi-lock-open" id="abstract-sponsor"></i>
    </div>
    <h3>Sponsor a question</h3>
    <p>Your donation will go towards writing quality questions to help the community become a tiny bit more knowledgeable.</p>
    <p>Your name or the name of your business will appear as the question contributor.</p>
    <p class="cta text-sm">
      <LinkButton :href="PageIDs.SPONSORSHIP"  label="Sponsor now" icon="pi pi-lock-open" />
    </p>
  </div>

  <div class="contribute-card">
    <div class="abstract text-green-200 dark:text-green-200">
      <i class="pi pi-comments"></i>
    </div>
    <h3>Give feedback</h3>
    <p>Did you come across a bug or have a suggestion?</p>
    <p>Consider making a <a href="https://github.com/rimutaka/bitesized">pull request</a> or <a href="mailto:max@bitesized.info">email the maintainer</a> with your ideas,
      corrections or whatever other feedback you may have.</p>
  </div>

  <div class="contribute-card">
    <div class="abstract text-indigo-200 dark:text-indigo-200">
      <i class="pi pi-megaphone" style="transform: rotate(-12deg);"></i>
    </div>
    <h3>Spread the word</h3>
    <p>Share your favourite questions on social media or forward our emails to colleagues
      to challenge them with a bite-sized piece of fun and knowledge.</p>
    <p>
      Questions are the new funny cat videos.
    </p>
    <p class="cta">
      <a href="https://www.linkedin.com/feed/" target="_blank"><i class="pi pi-linkedin text-sky-300 dark:text-sky-500"></i></a>
      <a href="https://www.reddit.com/submit?type=LINK" target="_blank"><i class="pi pi-reddit text-red-300 dark:text-red-500"></i></a>
      <a href="https://twitter.com/" target="_blank"><i class="pi pi-twitter text-gray-500"></i></a>
      <a href="https://www.facebook.com/" target="_blank"><i class="pi pi-facebook text-blue-300 dark:text-blue-500"></i></a>
      <a href="https://www.instagram.com/" target="_blank"><i class="pi pi-instagram" style="color: #ff006999"></i></a>
      <a href="https://www.tiktok.com" target="_blank"><i class="pi pi-tiktok text-teal-300 dark:text-teal-500"></i></a>
    </p>
  </div>

  <!-- Needed to enable button styles for LinkButton -->
  <Button label="" severity="primary" style="display: none" />


</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { PageIDs } from '@/router';

import LinkButton from "@/components/LinkButton.vue";
import Button from 'primevue/button';


</script>