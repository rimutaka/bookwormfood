<template>
  <div class="card mt-12">
    <div v-if="hydrated == LoadingStatus.Loaded || hydrated == LoadingStatus.NoData">
      <div class="flex gap-4 mb-8 align-middle">
        <h4 class="p-1">Topic</h4>
        <div class="p-2 rounded-md flex gap-4 flex-wrap" :class="{ 'red-highlight-border border': !questionReadiness.topic && requiredHighlight }">
          <div class="flex" v-for="topic in topics" :key="topic.id">
            <RadioButton v-model="selectedTopic" name="topics" :value="topic.id" :inputId="topic.id" :disabled="props.topic != undefined" />
            <label :for="topic.id" class="ms-2">{{ topic.t }}</label>
          </div>
        </div>
      </div>

      <div class="mb-4">
        <div class="flex flex-wrap gap-4 mb-4">
          <h4 class="mt-auto">Question</h4>
          <div class="flex-grow text-end">
            <Button :label="previewWindow ? 'Refresh preview' : 'Open preview'" icon="pi pi-receipt" severity="secondary" class="whitespace-nowrap" iconPos="right" @click="showPreviewWindow()" />
          </div>
        </div>
        <div class="w-full">
          <Textarea v-model="questionText" id="questionTextInput" class="w-full" rows="3" :invalid="!questionReadiness.question && requiredHighlight" @keydown="formattingKeypress" placeholder="Enter the text of the question here as plain text or Markdown." />
          <p v-if="questionText" class=" input-help-line">Plain text or <a href="https://www.markdownguide.org" target="_blank" title="Markdown syntax guide">Markdown</a> without HTML or ![images].</p>
        </div>
      </div>

      <div class="flex flex-wrap gap-4 mb-8">
        <h4>One line summary for the title</h4>
        <div class="w-full mb-6">
          <InputText v-model="title" class="w-full mb-2" :maxlength="MAX_TITLE_LEN" :invalid="!questionReadiness.title && requiredHighlight" placeholder="A short title for the question." />
          <p v-if="title" class=" input-help-line">A short description of what the question is about. No Markdown, 100 characters max.</p>
        </div>
      </div>

      <div class="flex flex-wrap gap-4 mb-8">
        <h4>Answers</h4>
        <div class="w-full mb-6" v-for="(answer, idx) in answers" :key="idx">
          <Textarea v-model="answer.a" :value="answer.a" rows="3" :id="`answerInput${idx}`" class="w-full mb-2" placeholder="One of possible answers, always visible." @keydown="formattingKeypress" />
          <Textarea v-model="answer.e" :value="answer.e" rows="5" :id="`explanationInput${idx}`" class="w-full mb-2" placeholder="A detailed explanation, revealed after answering. " @keydown="formattingKeypress" />
          <div class="flex">
            <div class="flex-grow justify-start text-start ps-4 gap-4 flex flex-wrap">
              <div class="w-fit  whitespace-nowrap">
                <input type="radio" v-model="answer.c" :name="`ci${idx}`" :id="`c${idx}`" :value="true" class="h-8 w-8 dark:bg-neutral-700 checked:bg-green-600 p-3" />
                <label class="ms-2" :for="`c${idx}`">Correct</label>
              </div>
              <div class="w-fit whitespace-nowrap">
                <input type="radio" v-model="answer.c" :name="`ci${idx}`" :id="`i${idx}`" :value="false" :checked="!answer.c" class="h-8 w-8 dark:bg-neutral-700 checked:bg-red-600 p-3" />
                <label class="ms-2" :for="`i${idx}`">Incorrect</label>
              </div>

            </div>
            <div class="flex-shrink flex gap-4 flex-wrap justify-end">
              <Button v-if="answers.length > 1" label="Delete this answer" icon="pi pi-trash" severity="secondary" class="whitespace-nowrap" @click="deleteAnswer(idx)" />
              <Button label="Add another answer" icon="pi pi-plus" severity="secondary" class="whitespace-nowrap" @click="addAnswer(idx)" />
            </div>
          </div>
        </div>
      </div>

      <div>
        <h4 class="text-start mb-4">Contributor</h4>
        <ContributorForm class="mb-12" />
      </div>
      <div class="flex gap-12 mt-8">
        <div class="text-left flex-grow border-r border-slate-200 dark:border-slate-700 pr-4">
          <h4 class="mb-4">Question readiness</h4>
          <div class="flex flex-wrap gap-4">
            <ul class="question-readiness">
              <li class="font-semibold" :class="{ 'red-highlight': requiredHighlight }">Required: </li>
              <li :class="{ 'question-ready': questionReadiness.topic, 'question-not-ready': !questionReadiness.topic, 'red-highlight': requiredHighlight }"><i></i>Topic selected</li>
              <li :class="{ 'question-ready': questionReadiness.question, 'question-not-ready': !questionReadiness.question, 'red-highlight': requiredHighlight }"><i></i>Question text entered</li>
              <li :class="{ 'question-ready': questionReadiness.title, 'question-not-ready': !questionReadiness.title, 'red-highlight': requiredHighlight }"><i></i>One line summary</li>
            </ul>
            <ul class="question-readiness">
              <li class="font-semibold">Can enter later: </li>
              <li :class="{ 'question-ready': questionReadiness.answers, 'question-not-ready': !questionReadiness.answers }"><i></i>At least 2 answers</li>
              <li :class="{ 'question-ready': questionReadiness.correct, 'question-not-ready': !questionReadiness.correct }"><i></i>At least 1 correct answer</li>
              <li :class="{ 'question-ready': questionReadiness.explanations, 'question-not-ready': !questionReadiness.explanations }"><i></i>Detailed explanations for all answers</li>
            </ul>
          </div>
          <p class="question-ready"><i></i><a href="https://creativecommons.org/licenses/by-sa/4.0/" target="_blank">CC-BY-SA 4.0</a> license</p>

        </div>
        <div class="flex-shrink text-end flex-row space-x-4 space-y-4">
          <Button label="Cancel" icon="pi pi-times" raised severity="secondary" class="whitespace-nowrap" @click="cancelAndGoBack()" />
          <Button label="Save" icon="pi pi-check" raised class="my-auto whitespace-nowrap" @click="saveQuestion()" />
          <p v-if="requiredHighlight" class="red-highlight text-sm text-end">Fill in required fields to save.</p>
        </div>
      </div>
    </div>

    <LoadingMessage v-if="hydrated == LoadingStatus.Loading" />
    <h3 v-if="hydrated == LoadingStatus.Error" class="mt-8 mb-8 text-slate-500 dark:text-slate-100">Sorry, something went wrong. Try again.</h3>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, watchEffect, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { useRouter } from "vue-router";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import debounce from "lodash.debounce"
import _ from "lodash";

import { TOPICS, QUESTION_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID, TOKEN_HEADER_NAME, AWS_BODY_HASH_HEADER, PREVIEW_QUESTION_LS_KEY, MAX_TITLE_LEN, findTopicById } from "@/constants";
import type { Answer, Question } from "@/interfaces";
import { fetchQuestionMD } from "@/data-loaders/fetch-question";
import { LoadingStatus } from "@/interfaces";

import Button from 'primevue/button';
import RadioButton from 'primevue/radiobutton';
import Textarea from 'primevue/textarea';
import InputText from 'primevue/inputtext';
import LoadingMessage from "./LoadingMessage.vue";
import ContributorForm from "./ContributorForm.vue";
import { PageIDs } from "@/router";

const props = defineProps<{
  topic: string | undefined,
  qid?: string | undefined,
}>()

const router = useRouter();
const store = useMainStore();
const { token, questionMD, question } = storeToRefs(store);

const hydrated = ref(LoadingStatus.Loading); // toggles the form between loading and the full form
const topics = ref(TOPICS);
const selectedTopic = ref(""); // the topic of the question from TOPICS

const questionText = ref(""); // the text of the question in markdown
const answers = ref<Array<Answer>>([{ a: "", e: "", c: false, sel: false }]); // the list of answers
const title = ref<string | undefined>(); // the title of the question

// a reference to the preview window that can be opened on demand
const previewWindow = ref<Window | null>(null);

/// used to inform the user what steps are required
/// affects questionReady
/// updated via watch
const questionReadiness = ref({
  topic: false,
  question: false,
  answers: false,
  correct: false,
  explanations: false,
  title: false,
});
const questionReady = ref(false); // enables Submit button

// turned on briefly to highlight the required fields
// when the user attempts to save the question
const requiredHighlight = ref(false);

/// Adds an answer block to the form
function addAnswer(index: number) {
  answers.value.splice(index + 1, 0, { a: "", e: "", c: false, sel: false });
}

/// Removes an answer block from the form
function deleteAnswer(index: number) {
  if (answers.value.length === 1) {
    // cannot delete the last remaining answer
    return;
  }
  answers.value.splice(index, 1);
}

/// Adds or removes Markdown formatting from the selected text
/// in question, answer and explanation fields
function formattingKeypress(event: KeyboardEvent) {

  // exit early if there is no selected text
  const target = <HTMLInputElement>event.target;
  const start = target.selectionStart;
  const end = target.selectionEnd;

  // check there is a selection
  if (!start || !end || start === end) {
    return;
  }

  // check how the key should be handled
  let formatSymbolStart = "", formatSymbolEnd = "";
  let formatSymbolLength = 0; // remains 0 if the selection should not be toggled
  if (event.key === "b" && event.ctrlKey) { formatSymbolStart = "**"; formatSymbolEnd = "**"; formatSymbolLength = 2; }
  if (event.key === "i" && event.ctrlKey) { formatSymbolStart = "_"; formatSymbolEnd = "_"; formatSymbolLength = 1; }
  if (event.key === "_") { formatSymbolStart = "_"; formatSymbolEnd = "_"; }
  if (event.key === "*") { formatSymbolStart = "*"; formatSymbolEnd = "*"; }
  if (event.key === "`") { formatSymbolStart = "`"; formatSymbolEnd = "`"; }
  if (event.key === "'") { formatSymbolStart = "'"; formatSymbolEnd = "'"; }
  if (event.key === "\"") { formatSymbolStart = "\""; formatSymbolEnd = "\""; }
  if (event.key === "{") { formatSymbolStart = "{"; formatSymbolEnd = "}"; }
  if (event.key === "[") { formatSymbolStart = "["; formatSymbolEnd = "]"; }
  if (event.key === "(") { formatSymbolStart = "("; formatSymbolEnd = ")"; }

  // if no format symbol is found, exit
  if (!formatSymbolStart) return;

  event.preventDefault();

  // a reusable function to check if the formatting should be removed
  // ** are usually outside the selection and __ are inside
  // brackets and quotes are excluded from the check via formatSymbolLength = 0
  const shouldRemoveFormat = (v: string) => formatSymbolLength > 0
    && (v.slice(start - formatSymbolLength, start) === formatSymbolStart && v.slice(end, end + formatSymbolLength) === formatSymbolEnd
      || v.slice(start)?.startsWith(formatSymbolStart) && v.slice(end - formatSymbolLength)?.startsWith(formatSymbolEnd));

  // reusable functions for adding and removing formatting
  const remover = (v: string) => {
    if (v.slice(start)?.startsWith(formatSymbolStart) && v.slice(end - formatSymbolLength)?.startsWith(formatSymbolEnd))
      // the removable part is inside the selection, e.g. __text__
      return v.slice(0, start) + v.slice(start + formatSymbolLength, end - formatSymbolLength) + v.slice(end);
    else
      // the removable part is outside the selection, e.g. **text**
      return v.slice(0, start - formatSymbolLength) + v.slice(start, end) + v.slice(end + formatSymbolLength);
  };

  // a reusable function for adding formatting
  const adder = (v: string) => v.slice(0, start) + formatSymbolStart + v.slice(start, end) + formatSymbolEnd + v.slice(end);

  // find the right v-model to update
  // then check if the formatting already exists and should be added or removed (toggle)
  if (target.id == "questionTextInput") {
    questionText.value = (shouldRemoveFormat(questionText.value)) ? remover(questionText.value) : adder(questionText.value);
    // repeat the same for answers and explanations
  } else if (target.id.startsWith("answerInput")) {
    const index = parseInt(target.id.replace("answerInput", ""));
    answers.value[index].a = (shouldRemoveFormat(answers.value[index].a)) ? remover(answers.value[index].a) : adder(answers.value[index].a);
  } else if (target.id.startsWith("explanationInput")) {
    const index = parseInt(target.id.replace("explanationInput", ""));
    answers.value[index].e = (shouldRemoveFormat(answers.value[index].e)) ? remover(answers.value[index].e) : adder(answers.value[index].e);
  }
  else {
    console.error("Unknown formatting target", target);
  }
}

/** Save question in the cloud */
async function saveQuestion() {

  if (!token.value) {
    // unlikely to get here because there is a page guard in the router
    console.log("No token found. Redirecting to homepage.");
    router.push("/");
    return;
  }

  if (!questionReady.value) {
    requiredHighlight.value = true;
    return;
  }

  // the lambda gets all it needs from the serialized JSON object
  const submissionQuestion = JSON.stringify(<Question>{
    qid: props.qid,
    topic: selectedTopic.value,
    question: questionText.value,
    answers: answers.value,
    correct: 0,
    contributor: questionMD.value?.contributor, // this struct is set by a sub-component
    title: title.value,
  });

  // console.log(submissionQuestion);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(submissionQuestion);
  const bodyHash = toHex(await hash.digest());

  const response = await fetch(`${QUESTION_HANDLER_URL}`, {
    method: "PUT",
    body: submissionQuestion,
    headers: {
      [AWS_BODY_HASH_HEADER]: bodyHash,
      [TOKEN_HEADER_NAME]: token.value,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the error occurred
  console.log("Response status: ", response.status);
  if (response.status === 200) {
    try {
      questionMD.value = <Question>await response.json();
      // clear the local storage
      localStorage.removeItem(PREVIEW_QUESTION_LS_KEY);
      // console.log("Saved OK: ", savedQuestion);
      // redirect to the saved question
      // TODO: make use of the returned question details to avoid an extra fetch
      router.push(`/${PageIDs.QUESTION}?${URL_PARAM_TOPIC}=${questionMD.value.topic}&${URL_PARAM_QID}=${questionMD.value.qid}`);
    } catch (error) {
      console.error(error);
    }
  } else {
    console.error("Failed to save the question: ", response.status);
  }
}

/// Stores the current question data in local storage and opens a new popup window for live preview
/// LS is a reliable way of passing the initial data to the preview window
/// subsequent updates are sent via postMessage
const showPreviewWindow = () => {

  localStorage.setItem(PREVIEW_QUESTION_LS_KEY, JSON.stringify(packageQuestion()));

  previewWindow.value = window.open(`${window.location.origin}/preview`, PREVIEW_QUESTION_LS_KEY);
}

/// Slows down messaging the preview window for changes not covered by markdown conversion
const debouncePostMsg = debounce(() => {
  postQuestionPreview();
}, 500);

// update questionReadiness list and enable the submit button via questionReady
watch([selectedTopic, questionText, answers.value, title, questionMD], ([, , answersNew], [, , answersOld]) => {
  // assess question readiness
  questionReadiness.value.topic = selectedTopic.value !== "";
  questionReadiness.value.question = questionText.value.length > 10;
  questionReadiness.value.answers = answers.value.length >= 2 && answers.value.every((answer) => answer.a.length > 0);
  questionReadiness.value.correct = answers.value.some((answer) => answer.c);
  questionReadiness.value.explanations = answers.value.every((answer) => answer.e.length > 10);
  questionReadiness.value.title = title.value != undefined && (title.value.length > 10);

  // enable / disable the submit button
  questionReady.value = questionReadiness.value.topic && questionReadiness.value.question && questionReadiness.value.title;

  // reset the required highlight to normal if the user makes changes
  if (questionReady.value) requiredHighlight.value = false;

  // changes are sent to the preview with a debounce
  debouncePostMsg();
}, { deep: true });

/** Resets local and store values to start accepting data for a brand new question
* from a blank form */
function resetValuesForNewQuestion() {
  selectedTopic.value = "";
  questionText.value = "";
  title.value = "";
  answers.value.length = 0;
  answers.value.push({ a: "", e: "", c: false, sel: false });

  // clear all fields from the question in store
  // this is a hack and there should be a more elegant solution
  store.resetQuestionToBlank();
}

/** Restore the question content from the local storage, if any */
function restoreQuestionFromLS() {

  // get the question from the local storage
  const lsQuestion = localStorage.getItem(PREVIEW_QUESTION_LS_KEY);
  if (!lsQuestion) {
    console.log("No question found in the local storage");
    return;
  }

  // load the question from the local storage
  // this is assumed to be infallible because only the form can write to the LS
  // it may fail if the contents is not JSON
  const fetchedQuestion = <Question>JSON.parse(lsQuestion);

  // delete the LS if the qid does not match the current question
  // they have to be both present and match or both absent
  if (fetchedQuestion.qid !== props.qid) {
    console.log("Clear LS version - mismatch with the current question");
    localStorage.removeItem(PREVIEW_QUESTION_LS_KEY);
    return;
  }

  // load the question from the struct into the form and the store
  loadQuestion(fetchedQuestion);
}

/// Loads all local variables with the question data, if present.
function loadQuestion(fetchedQuestion: Question) {
  // copy DDB values to the form models
  selectedTopic.value = fetchedQuestion.topic;
  questionText.value = fetchedQuestion.question;
  title.value = fetchedQuestion.title;

  // copy the array while maintaining a reference to the original object
  // https://stackoverflow.com/questions/71353509/why-would-a-vue3-watcher-of-a-prop-not-be-triggered-composition-api
  answers.value.length = 0;
  fetchedQuestion.answers.forEach((answer: Answer) => {
    answers.value.push(answer);
  });

  // copy the loaded question to the store
  // for sub-components to access the data
  questionMD.value = JSON.parse(JSON.stringify(fetchedQuestion));; // store the question in the store
}

/** Clear the storage and go back to the previous page */
function cancelAndGoBack() {
  localStorage.removeItem(PREVIEW_QUESTION_LS_KEY);
  router.back();
}

watchEffect(async () => {

  // clear the question from the store on load to force a reload on save
  // if the store's question ID matches the URL it skips the reload
  question.value = undefined;

  // if no topic/qid is set, this is a new question
  if (!(props.topic && props.qid)) {
    console.log("Adding a new question");
    resetValuesForNewQuestion();
    restoreQuestionFromLS();
    hydrated.value = LoadingStatus.Loaded; // enable the form
    return;
  }

  // disable the form while fetching the question
  hydrated.value = LoadingStatus.Loading;
  questionMD.value = undefined;

  // fetching an existing question for editing
  console.log(`Fetching MD question for ${props.topic}/${props.qid}`);
  if (!token.value) {
    console.log("No token found. Redirecting to homepage.");
    router.push("/");
    return;
  }

  const fetchedQuestion = await fetchQuestionMD(props.topic, props.qid, token.value);
  if (fetchedQuestion) {
    loadQuestion(fetchedQuestion);
    hydrated.value = LoadingStatus.Loaded; // enable the form
  }
  else {
    hydrated.value = LoadingStatus.Error; // enable the error message
  }
});

/** package the current question data from the input fields into a struct */
function packageQuestion() {
  return <Question>{
    qid: props.qid,
    topic: selectedTopic.value,
    question: questionText.value,
    answers: answers.value,
    correct: 0, // setting this to the correct value will enable checkboxes in the preview
    contributor: questionMD.value?.contributor,
    title: title.value,
  };
}

/// Packages the current question data and sends it to the preview window, if open
function postQuestionPreview() {

  // save the question in the local storage as a backup in case the window was closed or refreshed
  const msg = packageQuestion();
  localStorage.setItem(PREVIEW_QUESTION_LS_KEY, JSON.stringify(packageQuestion()));

  // do not attempt to post it if the window is not open, but this will not help if the window was closed
  // because the reference will still be there
  if (!previewWindow.value) return;

  // delete the window ref if the window was closed and exit
  if (previewWindow.value.closed) {
    console.log("Preview window was closed");
    previewWindow.value = null;
    return;
  };

  console.log("Sending preview update msg");
  previewWindow.value?.postMessage(JSON.stringify(msg));
}

</script>
