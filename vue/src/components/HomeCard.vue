<template>
  <div class="mt-8">
    <div class="mb-12 flex gap-6 flex-wrap justify-center align-top">
      <div class="">
        <LinkButton :href="`/${PageIDs.QUESTIONS}`" label="View all questions" icon="pi pi-list-check" class="mb-auto" />
      </div>
      <RandomQuestionButton class="align-top" />
    </div>
    <TransitionSlot>
      <RandomQuestion v-if="showingRandomQuestion" />
    </TransitionSlot>

    <div class="mx-auto">
      <Button label="Subscribe" icon="pi pi-envelope" raised rounded class="mb-2" @click="router.push({ name: PageIDs.SUBSCRIPTION })" />
      <p class="text-xs text-center md:mb-auto text-slate-500 dark:text-slate-200">
        You will be asked to
        <router-link :to="{ name: PageIDs.SUBSCRIPTION }">sign in</router-link>
        with <i class="pi pi-github ms-1 me-2"></i><i class="pi pi-google me-2"></i>
      </p>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref } from 'vue';
import router from '@/router';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { PageIDs } from '@/router';

import Button from 'primevue/button';
import LinkButton from './LinkButton.vue';
import TransitionSlot from "./TransitionSlot.vue";
import RandomQuestion from "./RandomQuestion.vue";
import RandomQuestionButton from './RandomQuestionButton.vue';

const store = useMainStore();
const { currentTopic, showingRandomQuestion } = storeToRefs(store);

// do not show the sample question when the page is first displayed
// it will be shown when the user clicks on the button requesting it
showingRandomQuestion.value = false;

</script>
