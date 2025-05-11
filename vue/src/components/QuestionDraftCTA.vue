<template>
  <div class="mb-8 text-sm cta-box">

    <div class=" max-w-lg mx-auto text-start text-slate-600 dark:text-slate-200">
      <h3 class="text-center mb-4">This question is a draft</h3>
      <p class="mb-4">Draft questions are visible to their authors, site moderators and anyone with the direct link to it.</p>
      <p class="mb-4">Questions become publicly available and are sent to subscribers after a review by other members.</p>
      <p>Reach out to the <a href="mailto:max@bitesized.info">site maintainer</a> if you need help or have feedback.</p>
      <ul v-if="hydrated == LoadingStatus.Loaded && !questionReady" class="mt-4">
        <li class="mb-2">
          <h3>TODO:</h3>
        </li>
        <li class="ms-4 mb-2 text-sm" v-if="!questionReadiness.answers"><i class="pi pi-stop text-xs" style="font-size: 0.8rem"></i> provide at least 2 answers</li>
        <li class="ms-4 mb-2 text-sm" v-if="!questionReadiness.correct"><i class="pi pi-stop text-xs" style="font-size: 0.8rem"></i> have at least 1 correct answer</li>
        <li class="ms-4 mb-2 text-sm" v-if="!questionReadiness.explanations"><i class="pi pi-stop" style="font-size: 0.8rem"></i> add detailed explanations to all answers</li>
      </ul>
      <div class="mt-8 w-full flex justify-center">
        <LinkButton v-if="reviewLink" :href="reviewLink" label="Review question source"/>
      </div>
      <LoadingMessage v-if="hydrated == LoadingStatus.Loading" msg="Checking question readiness ..." class="mb-0 text-center" />
      <h3 v-if="hydrated == LoadingStatus.Error" class="mt-8 mb-8 text-slate-500 dark:text-slate-100">Sorry, something went wrong. Try again.</h3>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, watch, watchEffect, computed } from "vue";
import { useMainStore } from '@/store';
import { storeToRefs } from 'pinia'
import type { Question } from "@/interfaces";
import { fetchQuestionMD } from "@/data-loaders/fetch-question";
import { LoadingStatus } from "@/interfaces";
import { URL_PARAM_TOPIC, URL_PARAM_QID } from "@/constants";
import { PageIDs } from "@/router";

import LoadingMessage from "./LoadingMessage.vue";
import LinkButton from "./LinkButton.vue";

const store = useMainStore();
const { token, question, user } = storeToRefs(store);

const questionMD = ref<Question | undefined | null>();
const hydrated = ref(LoadingStatus.Loading);

/// used to inform the user what steps are required
/// affects questionReady
/// updated via watch
const questionReadiness = ref({
  answers: false,
  correct: false,
  explanations: false,
});
const questionReady = computed(() => {
  if (!questionMD.value) return false;

  // assess question readiness
  questionReadiness.value.answers = questionMD.value.answers.length >= 2 && questionMD.value.answers.every((answer) => answer.a.length > 0);
  questionReadiness.value.correct = questionMD.value.answers.some((answer) => answer.c);
  questionReadiness.value.explanations = questionMD.value.answers.every((answer) => answer.e?.length > 10);

  return Object.values(questionReadiness.value).every((value) => value);

});

// available to moderators only
const reviewLink = computed(() => {
  if (user.value?.isMod) return `/${PageIDs.REVIEW}?${URL_PARAM_TOPIC}=${question.value?.topic}&${URL_PARAM_QID}=${question.value?.qid}`;
});

watchEffect(async () => {
  if (!question.value || !token.value) return;

  // get the same question in markdown format
  // this is only available to the author of the question
  questionMD.value = await fetchQuestionMD(question.value.topic, question.value.qid, token.value);

  // indicate to the markup if the MD question is ready
  if (questionMD.value) {
    hydrated.value = LoadingStatus.Loaded;
  } else {
    hydrated.value = LoadingStatus.Error;
  }
});

</script>