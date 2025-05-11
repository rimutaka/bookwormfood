<template>
  <h1 v-if="question?.title" class="mb-4 md:mb-8 text-2xl text-start">{{ question?.title }}</h1>
  <h1 v-else class="mb-4 md:mb-8 text-2xl text-start">Question about <em class="italic">{{ topicName }}</em></h1>
  <QuestionDraftCTA v-if="question?.stage == PublishStage.Draft" />
  <LoadingMessage v-if="questionStatus == LoadingStatus.Loading" />
  <div v-else>
    <QuestionCard :next="true" @next-question="loadNextQuestion" />
    <div v-if="ctaBlockVisible" class="mb-12 md:mt-12 cta-box">
      <PostAnswerCTA />
    </div>
    <ContributorCard class="mb-12 mt-8 md:mt-16" />
  </div>
</template>

<script setup lang="ts">
import { computed, watch, ref, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { randomTopicId, findTopicById } from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { LoadingStatus, PublishStage } from '@/interfaces';

import QuestionCard from "../components/QuestionCard.vue";
import PostAnswerCTA from '@/components/PostAnswerCTA.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';
import ContributorCard from "../components/ContributorCard.vue";
import QuestionDraftCTA from '@/components/QuestionDraftCTA.vue';

const route = useRoute();
const router = useRouter();
const store = useMainStore();
const { question, email, questionStatus } = storeToRefs(store);

// save the initial values to maintain state later 
// the topic never changes after the value is set to what is in the URL or to a random one
// qid will change if another question is loaded
const topic = route.query.topic ? <string>route.query.topic : randomTopicId();
const initialQid = route.query.qid ? <string>route.query.qid : undefined;
// this flag tells to not store pages that have no question ID in history because they redirect to a new 
// random question catching the user in a loop
let replaceRouter = initialQid ? false : true;

// route.query.qid can potentially be an array, but it should not happen in this app,
// so it is safe to cast them into a string
const qid = ref<string | undefined>(initialQid);

// a user-friendly name for the topic
const topicName = computed(() => {
  return findTopicById(topic);
});

const ctaBlockVisible = computed(() => {
  // checking user subs does not work because this page does not load user details
  // if  (!user.value?.topics.length && question.value?.answers?.[0].e) { return true } else { return false };
  if (!email.value && question.value?.answers?.[0].e) { return true } else { return false };
});

// reset the question ID and reload to show a new question from the same topic
const loadNextQuestion = () => {
  console.log("Load next question");
  qid.value = undefined;
  replaceRouter = true;
  store.loadNextQuestion(topic);
};

// update the query string with the next question topic and id when the question changes in the store
watch(question, (newQuestion) => {
  // console.log("newQuestion: ", newQuestion);
  if (newQuestion) {
    // topic.value = newQuestion.topic;
    qid.value = newQuestion.qid;
    console.log("New question topic/qid: ", topic, qid.value);

    // update page title
    if (question.value?.title) {
      window.document.title = question.value?.title;
    }

    // update the URL query string to match the current question
    if (route.query.qid != qid.value) {
      if (replaceRouter) {
        console.log("replace-navigating to ", topic, qid.value);
        replaceRouter = false; // only replace the first time, subsequent calls should have qid
        router.replace({ query: { topic: topic, qid: qid.value } });
      } else {
        console.log("push-navigating to ", topic, qid.value);
        router.push({ query: { topic: topic, qid: qid.value } });
      }
    }
  }
  else {
    console.log("question removed from store ");
  }
});

// this watch is needed to load questions on back/forward navigation
watch(route, (newQuery) => {
  const newQid = newQuery.query.qid as string | undefined;
  console.log(`Route query changed to ${newQuery.query.topic} / ${newQid}`);
  if (newQid != qid.value) {
    // load the new question if it is different from the current one
    store.loadQuestion(topic, newQid);
  }
});

// the initial topic has a random value assigned if no topic was provided
// update the URL query string to match the random topic
if (!route.query.topic) {
  console.log("Navigating to a random topic: ", topic);
  router.replace({ query: { topic: topic } });
}

if (topic && initialQid) {
  // a particular question was requested
  (async () => await store.loadQuestion(topic, initialQid))();
} else {
  // a random question was requested
  loadNextQuestion();
}

</script>
