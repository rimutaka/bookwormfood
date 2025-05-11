<template>
  <div class="flex flex-col items-center">
    <Button :label="buttonLabel" icon="pi pi-sparkles" size="small" severity="secondary" class="whitespace-nowrap" @click.capture="loadNextQuestion" />
    <p v-if="mustSelectTopic && !currentTopic" class="text-red-500 text-sm mt-2 max-w-fit">Select a topic first</p>
  </div>
</template>


<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { fetchQuestions } from '@/data-loaders/fetch-questions';

import Button from 'primevue/button';

const store = useMainStore();
const { currentTopic, showingRandomQuestion, questionsWithHistory, question } = storeToRefs(store);

// True if the user requested a random question without selecting a topic
const mustSelectTopic = ref(false);

const buttonLabel = computed(() => {
  const partOfButtonLabel = showingRandomQuestion.value ? `Try another` : 'View';
  return `${partOfButtonLabel} random question`;
});

// pull together currently selected topics and reload the question to show a new random one
const loadNextQuestion = async () => {
  console.log("Load next question");

  if (!currentTopic.value) {
    mustSelectTopic.value = true;
    return;
  }

  await store.loadNextQuestion(currentTopic.value);

  // enable the sample question component via this flag
  showingRandomQuestion.value = true;
};

// change the URL to match the current topic
watch(currentTopic, (newCurrentTopic) => {
  mustSelectTopic.value = newCurrentTopic ? true : false;
  // load the list of questions if none exists, or exists for a different topic
  if (newCurrentTopic && questionsWithHistory.value?.[0].question.topic !== newCurrentTopic) {
    showingRandomQuestion.value = false;
    question.value = undefined;
    // load the list of questions for the given topic
    fetchQuestions(newCurrentTopic).then((fetchedQuestions) => {
      questionsWithHistory.value = fetchedQuestions;
    });
  }
});

</script>
