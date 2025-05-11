<template>
  <div id="answerTopElement"></div>
  <TransitionSlot>
    <div v-if="question && questionStatus == LoadingStatus.Loaded" class="q-card">
      <div class="qna">
        <div class="q-text">
          <div class="" v-html="question?.question"></div>
        </div>

        <div v-for="(answer, index) in question?.answers" :key="index">
          <!-- Show the header in the first loop so that it appears once only  -->
          <h3 v-if="isAnswered && index === 0 && question?.correct == 1" class="mb-4">Your answer</h3>
          <h3 v-if="isAnswered && index === 0 && question?.correct > 1" class="mb-4">Your answers</h3>
          <h3 v-else-if="(props.isPreview || !isAnswered) && index === 0" class="mb-4">Answers</h3>
          <!-- This one should appear once after user selection -->
          <h3 v-else-if="isAnswered && index === question?.correct" class="mb-4">Other options</h3>

          <div class="mb-8 border-2 rounded-md" :class="{ 'border-4': isAnswered, 'border-green-100 dark:border-green-700': answer?.c, 'border-red-100 dark:border-red-700': !answer?.c && isAnswered, 'border-slate-100': !isAnswered, 'hide-explanation': isAnswered && !answer.c && !answer.sel && !isPreview }">
            <div class="flex items-center" :class="{ 'border-b-2': isAnswered, 'border-green-100 dark:border-green-700': answer?.c, 'border-red-100 dark:border-red-700': !answer?.c && isAnswered }">
              <input type="radio" v-if="question?.correct == 1" :name="question?.qid" :value="index" :disabled="isAnswered" v-model="answerRadio" @click="handleRadioClick(index)" />
              <input type="checkbox" v-if="question?.correct && question.correct > 1" :name="question?.qid" :disabled="isAnswered" :value="index" v-model="answersCheckbox" />
              <div class="q-answer" v-html="answer.a" @click.prevent="handleQuestionAreaClick(index)"></div>
            </div>
            <div v-if="answer?.c" class="px-2 my-2">Correct.</div>
            <div v-else-if="isAnswered" class="px-2 my-2">
              <!-- Expand by default and hide the tags in Preview mode -->
              <Tag value="Incorrect" severity="secondary" class="me-2 incorrect-tag" />
              <Tag value="Explain" icon="pi pi-sort-down-fill" severity="secondary" class="explain-link" @click="showExplanation" />
              <span class="incorrect-label">Incorrect.</span>
            </div>
            <div class="q-explainer" v-if="answer?.e" v-html="answer.e"></div>
          </div>
        </div>

        <div v-if="shouldShowRefresher" class="mb-8 refresher no-print">
          <h3 class="mb-4">Not sure?</h3>
          <div class="border-2 rounded-md border-slate-100">
            <QuestionRefresherCard v-if="refresherLinksToggleFlag" />
            <div v-else class="flex items-center q-answer">
              <Tag value="Refresher material" icon="pi pi-sort-down-fill" severity="secondary" class="explain-link" @click.prevent="refresherLinksToggleFlag = true" />
            </div>
          </div>
        </div>

        <div v-if="feedbackStatus !== undefined" class="mb-12">
          <h3 class="mb-4">Can you help us improve this question?</h3>
          <div class="border-2 rounded-md border-slate-100 p-2 mb-8">
            <QuestionFeedback class="" />
          </div>
        </div>

        <div v-if="!props.isPreview && feedbackStatus == undefined" class="flex no-print">
          <!-- Hide this block in Preview mode -->
          <div class="flex-grow text-start">
            <LinkButton v-if="hasToken" :href="editPageUrl" label="Edit" class="me-2 mb-2" icon="pi pi-pencil" />
            <LinkButton :href="questionTopicAndPageUrl" label="Feedback" class="me-2 mb-2" icon="pi pi-megaphone" @click.capture="enableFeedbackForm" />
            <LinkButton :href="questionTopicAndPageUrl" label="Share" class="me-2 mb-2" icon="pi pi-share-alt" @click.capture="copyLinkToClipboard" />
            <LinkButton v-if="!isAnswered && next" :href="questionTopicOnlyUrl" label="Skip" class="me-2 mb-2" icon="pi pi-angle-double-right" @click.prevent="emit(NEXT_QUESTION_EMIT)" />
            <p v-if="linkCopiedFlag" class="text-xs text-slate-500">Link copied to the clipboard</p>
            <p v-if="!linkCopiedFlag">&nbsp;</p>
          </div>
          <div class="flex-shrink text-end">
            <Button v-if="!isAnswered" label="Submit" :icon="isQuestionReady ? 'pi pi-check' : 'pi pi-ellipsis-h'" raised class="font-bold px-24 py-4 my-auto whitespace-nowrap" :class="{ 'opacity-50': !isQuestionReady }" @click.prevent="getAnswers()" />
            <LinkButton v-if="isAnswered && next" :href="questionTopicOnlyUrl" label="Try another question" class="mb-2" icon="pi pi-sparkles" :primary="token != null" @click.prevent="emit(NEXT_QUESTION_EMIT)" />
            <p v-if="!isAnswered" class="mt-2" :class="emphasizedSubmitReminder ? 'text-red-500 text-base' : 'text-slate-500 dark:text-slate-300 text-xs'">{{ howManyOptionsLeftToSelect }}</p>
          </div>
        </div>
      </div>
    </div>
    <LoadingMessage v-else-if="questionStatus == LoadingStatus.Loading" />
    <h3 v-else class="mt-8 mb-8 text-slate-500 dark:text-slate-100">Sorry, something went wrong. Try again.</h3>
  </TransitionSlot>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import router, { PageIDs } from "@/router";

import type { Question } from "@/interfaces";
import * as constants from "@/constants";
import { LoadingStatus } from "@/interfaces";

import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import Button from 'primevue/button';
import Tag from "primevue/tag";
import TransitionSlot from "./TransitionSlot.vue";
import LinkButton from "./LinkButton.vue";
import LoadingMessage from "./LoadingMessage.vue";
import QuestionFeedback from "./QuestionFeedback.vue";
import QuestionRefresherCard from "./QuestionRefresherCard.vue";

const props = defineProps<{
  // topic: string,// must have a value or "any" for any topic
  // qid?: string, // returns a random question if omitted
  next?: boolean, // true if the question can show the next button
  isPreview?: boolean, // true if the question should be taken from the store and not fetched
}>()

// tell the parent component the user wants to skip this question
const NEXT_QUESTION_EMIT = 'nextQuestion';
const emit = defineEmits([NEXT_QUESTION_EMIT]);

const store = useMainStore();
const { token, question, questionStatus, user, feedbackStatus, refresherLinksToggleFlag } = storeToRefs(store);

// as fetched from the server
const answersCheckbox = ref<string[]>([]);
const answerRadio = ref<string | undefined>();
const linkCopiedFlag = ref(false); // controls share button: f: Copy link, t: Link copied
const emphasizedSubmitReminder = ref(false); // Toggles the class of Submit button block to reminder to select the right number of answers

// only the author of the question can edit it
const hasToken = computed(() => question.value?.author && question.value.author === user.value?.emailHash);

const isAnswered = computed(() => question.value?.answers?.[0].e ? true : false);

const isQuestionReady = computed(() => {
  emphasizedSubmitReminder.value = false;
  // console.log("isQuestionReady", answerRadio.value, answersCheckbox.value, question.value?.correct);
  // must be either a single radio answer or multiple checkbox answers matching the number of correct answers
  return answerRadio.value !== undefined && question.value?.correct == 1 || answersCheckbox.value.length == question.value?.correct;
});

// this fn abstracts the complicated logic of informing the user
// how many answers they need to select
const howManyOptionsLeftToSelect = computed(() => {
  // this line is needed for type checking
  if (!question.value) return "";

  // is this a single choice question with radio buttons?
  if (question.value.correct == 1) {
    // if a radio button is selected it can be submitted
    // the user can change the answer, but not deselect it
    if (answerRadio.value == undefined) {
      return "Select one of the answers";
    } else {
      return "Check your selection and submit";
    }
  }

  // assume it is a multi-choice question from now on

  // perform an exhaustive match by how many answers were selected
  const remainingNumber = question.value.correct - answersCheckbox.value.length;
  if (remainingNumber == 0) {
    // if the required number of answers is selected, the user can submit
    return "Check your selection and submit";
  }
  else if (remainingNumber < 0) {
    // too many answers selected
    return `Only ${question.value.correct} answers should be selected`;
  }
  else {
    // more answers should be selected
    const wordMore = answersCheckbox.value.length ? "more" : "";
    const wordAnswers = remainingNumber > 1 ? "answers" : "answer";
    return `Select ${remainingNumber} ${wordMore} ${wordAnswers}`;
  }
});

/// A URL to the page with the question on its own
/// without the question ID to display a random question
const questionTopicOnlyUrl = computed(() => `/${PageIDs.QUESTION}?${constants.URL_PARAM_TOPIC}=${question.value?.topic}`);

/// A URL to the page with the question on its own
/// for sharing or opening separately
const questionTopicAndPageUrl = computed(() => `${questionTopicOnlyUrl.value}&${constants.URL_PARAM_QID}=${question.value?.qid}`);

/// A URL to the question edit page
const editPageUrl = computed(() => `/${PageIDs.ADD}?${constants.URL_PARAM_TOPIC}=${question.value?.topic}&${constants.URL_PARAM_QID}=${question.value?.qid}`);

/** True if the question has refresher links and they should be made available. */
const shouldShowRefresher = computed(() => question.value?.refresherLinks?.length && !props.isPreview && !isAnswered.value);

/// Toggles selections when the user clicks on the answer area
const handleQuestionAreaClick = (index: number) => {
  // do nothing if the question was answered
  // the inputs will be disabled anyway
  if (isAnswered.value) return;

  if (question.value?.correct == 1) {
    // deselect the radio button if it was already selected
    if (answerRadio.value == index.toString()) {
      answerRadio.value = undefined;
    } else {
      answerRadio.value = index.toString();
    }
  } else {
    const answerIndex = answersCheckbox.value.indexOf(index.toString());
    if (answerIndex > -1) {
      // deselect
      answersCheckbox.value.splice(answerIndex, 1);
    } else {
      // select
      answersCheckbox.value.push(index.toString());
    }
  }
}

/// Deselects the radio button if it was already selected
const handleRadioClick = (index: number) => {
  if (answerRadio.value == index.toString()) {
    answerRadio.value = undefined;
  }
}

/// Copies the direct link to the question to the clipboard
/// and changes the button message flag to display link copied msg
const copyLinkToClipboard = (e: MouseEvent) => {
  e.preventDefault();
  const shareUrl = document.location.origin + questionTopicAndPageUrl.value;
  navigator.clipboard.writeText(shareUrl);
  linkCopiedFlag.value = true;
  setTimeout(() => {
    linkCopiedFlag.value = false;
  }, 3000);
}

/** Turns the feedback form on. No effect if already turned on. */
const enableFeedbackForm = () => {
  if (feedbackStatus.value === undefined) feedbackStatus.value = LoadingStatus.NoData;
}

/// Changes the parent class of the answer to toggle the explanation
/// Explanations for incorrect answers NOT selected by the user are hidden by default
function showExplanation(evt: Event | undefined) {
  const src = <HTMLElement>evt?.currentTarget;
  console.log("showExplanation", src);
  if (src) {
    src.parentElement?.parentElement?.classList.remove("hide-explanation");
  }
}

/** Submits the user answers to the server and gets back the answers with explanations */
async function getAnswers() {
  // double-check there are answers to submit
  if (!isQuestionReady.value) {
    // console.log("Must select answers:", question.value?.correct);
    emphasizedSubmitReminder.value = true;
    return;
  }

  // the lambda expects a list of answers in the URL
  const answers = question.value?.correct == 1 ? answerRadio.value : answersCheckbox.value.join(constants.URL_PARAM_LIST_SEPARATOR);

  const url = `${constants.QUESTION_HANDLER_URL}${constants.URL_PARAM_TOPIC}=${question.value?.topic}&${constants.URL_PARAM_QID}=${question.value?.qid}&${constants.URL_PARAM_ANSWERS}=${answers}`;
  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token.value) headers.append(constants.TOKEN_HEADER_NAME, token.value);

  try {
    const response = await fetch(url,
      {
        headers: headers,
      });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        // update the question with the full details
        question.value = <Question>await response.json();
        console.log("Full question received"); //, question.value);

        // reset the user selection because the answers got rearranged with the correct ones at the top
        answersCheckbox.value = [];
        question.value.answers.forEach((answer, index) => {
          if (answer.sel) {
            if (question.value?.correct == 1) {
              answerRadio.value = index.toString();
              console.log("answerRadio", answerRadio.value);
            } else {
              answersCheckbox.value.push(index.toString());
              // console.log("answersCheckbox", answersCheckbox.value);
            }
          }
        });

        // scroll to the top of the answers section if the answers are not visible
        if (question.value.answers[0].e) {
          const answerTopElement = document.getElementById("answerTopElement")?.parentElement;
          if (answerTopElement) {
            const rect = answerTopElement.getBoundingClientRect();
            // console.log(rect.top, rect.right, rect.bottom, rect.left);
            if (rect.top < 0) {
              // this will only fire up if the view is below the top of the answers section
              // no scrolling if the user scrolled up past the answers
              answerTopElement.scrollIntoView({ behavior: "smooth" });
            }
          }
        }

      } catch (error) {
        console.error(error);
      }
    } else {
      console.error("Failed to submit answers: ", response.status);
    }
  } catch (error) {
    console.error("Failed to submit answers.");
    console.error(error);
  }
}


</script>
