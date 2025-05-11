<template>
  <div>
    <LoadingMessage v-if="(questionListStatus == undefined || questionListStatus == LoadingStatus.Loading) && loadingMsg" :msg="loadingMsg" />
    <div v-if="questionListStatus == LoadingStatus.Loaded" class="q-list">
      <QuestionListItem v-for="(question, index) in questionsWithHistory" :question="question" :key="index" :show-topic="topic == undefined" :user_email_hash="user?.emailHash" />
    </div>
    <div v-if="questionListStatus == LoadingStatus.Error">Cannot load the list of questions - something went wrong.</div>
    <!-- Show nothing if LoadingStatus.NoData -->
  </div>
</template>

<script setup lang="ts">
import { computed, watchEffect, ref } from 'vue';
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { LoadingStatus } from '@/interfaces';
import type { QuestionWithHistory } from '@/interfaces';
import { fetchQuestions } from '@/data-loaders/fetch-questions';

import QuestionListItem from '@/components/QuestionListItem.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';

const store = useMainStore();
const { token, questionListStatus, questionsWithHistory, user } = storeToRefs(store);

const props = defineProps<{ topic?: string }>();

const topicName = computed(() => constants.findTopicById(props.topic));

const loadingMsg = computed(() => topicName.value ? `Loading questions about ${topicName.value}` : undefined);

/// The topic always comes from props.topic
/// The qid comes from props.qid if random is false.
const loadQuestions = async (paramTopic?: string) => {
  // make sure nothing is showing while it's loading or if the component is reused
  questionListStatus.value = LoadingStatus.Loading;
  questionsWithHistory.value = undefined;

  // load the data
  questionsWithHistory.value = await fetchQuestions(paramTopic, token.value);

  // update the status
  if (questionsWithHistory.value && questionsWithHistory.value.length) {
    questionListStatus.value = LoadingStatus.Loaded;
  } else if (questionsWithHistory.value && questionsWithHistory.value.length == 0) {
    questionListStatus.value = LoadingStatus.NoData;
  } else {
    questionListStatus.value = LoadingStatus.Error;
  }
};

watchEffect(async () => {
  await loadQuestions(props.topic);
});

</script>
