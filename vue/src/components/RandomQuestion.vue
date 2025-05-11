<template>
  <div class="mb-12 border-t-2 border-b-2 border-slate-300">
    <h3 class="mt-8 mb-8 text-center">Random question about <em class="italic">{{ findTopicById(currentTopic) }}</em></h3>
    <QuestionCard />
    <ContributorCard class="mb-12 mt-8 md:mt-16" />
  </div>
</template>


<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { findTopicById } from "@/constants";

import QuestionCard from "./QuestionCard.vue";
import ContributorCard from "./ContributorCard.vue";

const store = useMainStore();
const { currentTopic } = storeToRefs(store);

</script>
