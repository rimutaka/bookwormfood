<template>
  <div class="mb-4 md:mb-8 text-start">
    <h1 class="mb-2 text-2xl">Question preview about <em class="italic">{{ question?.title || topicName }}</em></h1>
  </div>
  <QuestionCard :next="false" :is-preview="true" />
  <QuestionRefresherPreview v-if="sortedRefresherLinks.length" :links="sortedRefresherLinks" />
  <ContributorCard class="mb-12 mt-8 md:mt-16" />
</template>

<script setup lang="ts">
import { computed, watch, ref, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { TOPICS, PREVIEW_QUESTION_LS_KEY } from "@/constants";
import type { Question, Answer } from "@/interfaces";
import { LoadingStatus } from '@/interfaces';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import initWasmModule, { md_to_html, sort_links } from "@/wasm-rust/isbn_mod";

import QuestionCard from "../components/QuestionCard.vue";
import ContributorCard from "../components/ContributorCard.vue";
import QuestionRefresherPreview from '@/components/QuestionRefresherPreview.vue';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { question, questionStatus } = storeToRefs(store);

// Populated by the MD to HTML converter in random order.
// The object inside cannot be used with WASM because it lacks the `free` method implementation.
const correctAnswerLinks = ref<string[]>([]);
const incorrectAnswerLinks = ref<string[]>([]);
const questionLinks = ref<string[]>([]);

// Links extracted from the markdown, truncated, deduped and sorted by their priority.
// This is what actually shows on the page.
const sortedRefresherLinks = computed(() => {
  console.log("Sorting links entered");
  if (correctAnswerLinks.value.length + incorrectAnswerLinks.value.length + questionLinks.value.length == 0) return [];

  console.log("Found links to sort");

  try {
    return sort_links(questionLinks.value, correctAnswerLinks.value, incorrectAnswerLinks.value);
  } catch (e) {
    console.error("Error sorting links", e);
    return [];
  }
});

const topicName = computed(() => {
  return (TOPICS.find((t) => t.id == topic.value))?.t;
});

const qid = computed(() => question.value ? question.value.qid : "");
const topic = computed(() => question.value ? question.value.topic : "...");

/// updates the the vue store with the question data received in the message
function questionUpdateListener(event: MessageEvent) {
  // console.log("event: ", event);
  // discard any messages not coming from the main window
  if (event.origin !== window.origin) return;

  console.log("Msg received");

  if (!event.data) {
    console.log("No question payload in message");
    question.value = undefined;
    return;
  }

  renderQuestion(event.data);
}

window.removeEventListener("message", questionUpdateListener, false);
window.addEventListener("message", questionUpdateListener, false);

/// Renders a markdown question to HTML and updates the store
async function renderQuestion(qMarkdown: string) {
  const parsedQuestion: Question = JSON.parse(qMarkdown);

  // preload the parsed question if it's not in the store yet
  // remove MD/HTML fields
  if (!question.value || question.value.qid == undefined || question.value.qid !== parsedQuestion.qid) {
    const preload: Question = JSON.parse(qMarkdown);
    preload.question = "";
    preload.answers = [];
    question.value = preload;
  }

  if (question.value == undefined) {
    console.log("Question not loaded from the LS or the message");
    return;
  }

  // render the question
  md_to_html(parsedQuestion.question).then((md) => {
    console.log("Wasm rendered `question`");
    const { html, links } = md;
    console.log("Q-Links: ", links);
    question.value!.question = html; // markdown converted to HTML
    questionLinks.value = links; // links extracted from the markdown
  }).catch((e) => {
    console.error("Error rendering `question`", e);
  });

  // render answers one by one
  question.value.answers = await Promise.all(parsedQuestion.answers.map(async (answerToRender): Promise<Answer> => {

    // the render returns the HTML and links extracted from the markdown
    // that need to be sorted later depending on the origin so they have to preserve
    // which answer they belong to
    const renderedAnswer = await md_to_html(answerToRender.a);
    console.log("A-Links: ", renderedAnswer.links);

    const renderedExplanation = await md_to_html(answerToRender.e);
    console.log("E-Links: ", renderedExplanation.links);

    if (answerToRender.c) {
      renderedAnswer.links.map((link) => correctAnswerLinks.value.push(link));
      renderedExplanation.links.map((link) => correctAnswerLinks.value.push(link));
    } else {
      renderedAnswer.links.map((link) => incorrectAnswerLinks.value.push(link));
      renderedExplanation.links.map((link) => incorrectAnswerLinks.value.push(link));
    }

    return {
      a: renderedAnswer.html,
      c: answerToRender.c,
      e: renderedExplanation.html,
      sel: answerToRender.sel,
    }
  }));
}

// Load the question from local storage on mount, then listen for updates
(async () => {
  console.log("Loading question from LS");
  const qLS = localStorage.getItem(PREVIEW_QUESTION_LS_KEY);
  // console.log(qLS);

  if (!qLS) {
    console.log("No question in LS");
    question.value = undefined;
    return;
  }

  // load the Wasm module and set its readiness flag in the store
  console.log("Loading Wasm module");
  try {
    await initWasmModule();
    console.log("Wasm module loaded");
    // isWasmReady.value = true;
  } catch (e) {
    console.error("Error loading wasm module", e);
  };

  // convert to a Question object
  renderQuestion(qLS);
  questionStatus.value = LoadingStatus.Loaded;
})();

</script>
