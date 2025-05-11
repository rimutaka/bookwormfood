<template>
  <div class="my-12">
    <div v-if="hydrated == LoadingStatus.Loaded || hydrated == LoadingStatus.NoData" class="hidden lg:block q-review">
      <h4>Topic: <em>{{ findTopicById(questionMD?.topic) }}</em></h4>

      <h4 class="mt-auto">Question</h4>
      <pre>{{ questionMD?.question }}</pre>

      <h4>One line summary for the title</h4>
      <pre>{{ questionMD?.title }}</pre>

      <h4>Answers</h4>
      <div class="mb-6" v-for="(answer, idx) in questionMD?.answers" :key="idx">
        <pre>{{ answer.a }}</pre>
        <pre>{{ answer.e }}</pre>

        <p v-if="answer.c" class="text-green-600 font-bold">Correct</p>
        <p v-else class="text-red-600 font-bold">Incorrect</p>
      </div>

      <h4 class="text-start mb-4">Contributor</h4>
      <pre v-if="questionMD?.contributor?.name">{{ questionMD?.contributor?.name }}</pre>
      <pre v-if="questionMD?.contributor?.about">{{ questionMD?.contributor?.about }}</pre>
      <pre v-if="questionMD?.contributor?.url">{{ questionMD?.contributor?.url }}</pre>
      <pre v-if="questionMD?.contributor?.imgUrl">{{ questionMD?.contributor?.imgUrl }}</pre>

      <p class="mt-8"><strong>Last updated:</strong> <em>{{ questionMD?.updated }}</em></p>

      <div class="mt-8 flex gap-4 justify-end">
        <LinkButton label="Edit" icon="pi pi-pen-to-square" :href="editLink" />
        <LinkButton label="Try" icon="pi pi-sparkles" :href="tryLink" />
        <Button label="Publish" @click="approveQuestion()" />
      </div>
    </div>

    <LoadingMessage v-if="hydrated == LoadingStatus.Loading" msg="Loading question source ..." />
    <h3 v-if="hydrated == LoadingStatus.Error" class="mt-8 mb-8 text-slate-500 dark:text-slate-100">Sorry, something went wrong. Try again.</h3>
    
    <!-- Do not allow reviewing and approval of questions on small screens because it is easy to overlook issues. -->
    <div class="my-12 lg:hidden">
      <h4 class="mb-4">Minimum 1024px screen is required to review and approve a question</h4>
      <p>Resize your browser window or try a different device.</p>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, watchEffect, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { useRouter } from "vue-router";
import _ from "lodash";

import { QUESTION_STAGE_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID, URL_PARAM_STAGE, TOKEN_HEADER_NAME, findTopicById } from "@/constants";
import { fetchQuestionMD } from "@/data-loaders/fetch-question";
import { LoadingStatus, PublishStage } from "@/interfaces";
import { PageIDs } from "@/router";

import Button from 'primevue/button';
import LinkButton from "./LinkButton.vue";
import LoadingMessage from "./LoadingMessage.vue";

const props = defineProps<{
  topic: string | undefined,
  qid?: string | undefined,
}>()

const router = useRouter();
const store = useMainStore();
const { token, questionMD } = storeToRefs(store);

const hydrated = ref(LoadingStatus.Loading); // toggles the form between loading and the full form

const tryLink = computed(() => `/${PageIDs.QUESTION}?${URL_PARAM_TOPIC}=${questionMD.value?.topic}&${URL_PARAM_QID}=${questionMD.value?.qid}`);
const editLink = computed(() => `/${PageIDs.ADD}?${URL_PARAM_TOPIC}=${questionMD.value?.topic}&${URL_PARAM_QID}=${questionMD.value?.qid}`);

/** Save question in the cloud */
async function approveQuestion() {
  if (!token.value) {
    console.log("Missing token.");
    return;
  }

  const response = await fetch(`${QUESTION_STAGE_HANDLER_URL}${URL_PARAM_TOPIC}=${questionMD.value?.topic}&${URL_PARAM_QID}=${questionMD.value?.qid}&${URL_PARAM_STAGE}=${PublishStage.Published}`, {
    headers: {
      [TOKEN_HEADER_NAME]: token.value,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the error occurred
  console.log("Response status: ", response.status);
  if (response.status === 204) {
    console.log("Question approved. Redirecting to the list of questions: ", questionMD.value?.topic);
    router.push({ name: PageIDs.QUESTION, query: { [URL_PARAM_TOPIC]: questionMD.value?.topic, [URL_PARAM_QID]: questionMD.value?.qid } });
  } else {
    console.error("Failed to save the question: ", response.status);
  }
}

watchEffect(async () => {

  // check prerequisites
  if (!(token.value && props.topic && props.qid)) {
    console.log("Require token, topic, and qid to fetch the question.");
    hydrated.value = LoadingStatus.Loaded; // enable the form
    return;
  }

  // disable the form while fetching the question
  hydrated.value = LoadingStatus.Loading;
  questionMD.value = undefined;

  // fetching an existing question for review
  console.log(`Fetching MD question for ${props.topic}/${props.qid}`);

  const fetchedQuestion = await fetchQuestionMD(props.topic, props.qid, token.value);
  if (fetchedQuestion) {
    questionMD.value = JSON.parse(JSON.stringify(fetchedQuestion));; // store the question in the store

    hydrated.value = LoadingStatus.Loaded; // enable the form
  }
  else {
    hydrated.value = LoadingStatus.Error; // enable the error message
  }
});


</script>
