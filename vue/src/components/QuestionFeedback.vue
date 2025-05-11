<template>
  <div v-if="question?.qid" class="">
    <div class="mb-2">
      <div class="w-full">
        <p class="mb-4">Please share any suggestions how to make this question <i>correct</i>, <i>concise</i> and <i>unambiguous</i>.</p>
        <Textarea v-model="feedbackText" class="w-full" rows="3" />

      </div>
    </div>
    <div class="flex-shrink text-end flex-row space-x-4 space-y-4">
      <Button label="Close" size="small" icon="pi pi-times" severity="secondary" class="whitespace-nowrap" @click="feedbackStatus = undefined" />
      <Button label="Send feedback" size="small" icon="pi pi-send" class="my-auto whitespace-nowrap" @click="submitFeedback()" />
      <p v-if="feedbackStatus == LoadingStatus.Error" class="text-xs">
        <span class="text-red-500">Failed to send your feedback.</span>
        <br />
        <span class="">
          Can you try again or <a target="_blank" :href="`mailto:max@bitesized.info?subject=Feedback for ${question.topic} / ${question.qid}&body=${encodeURIComponent(feedbackText)}`">email the maintainer</a> directly?
        </span>
      </p>
      <p v-else-if="feedbackStatus == LoadingStatus.Loading" class="text-xs">Sending ...</p>
      <p v-else-if="textTooShort" class="text-xs text-red-500">Can you give more detail?</p>
      <p v-else-if="textTooLong" class="text-xs text-red-500">Sorry, can you make it shorter?</p>
      <p v-else-if="feedbackStatus == LoadingStatus.Loaded" class="text-xs">Feedback received. Thank you!</p>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, watch } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";

import { QUESTION_FEEDBACK_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID, TOKEN_HEADER_NAME, AWS_BODY_HASH_HEADER } from "@/constants";
import { LoadingStatus } from "@/interfaces"

import Button from 'primevue/button';
import Textarea from 'primevue/textarea';

const store = useMainStore();
const { token, question, feedbackStatus } = storeToRefs(store);

const feedbackText = ref("");
const textTooShort = ref(false);
const textTooLong = ref(false);

// to prevent multiple submissions of the same feedback
let lastSubmissionHash = "";

// reset validation flags when the text changes
// do not set them here until the user tries to submit
watch(feedbackText, () => {
  if (feedbackText.value.length >= 10) textTooShort.value = false;

  if (feedbackText.value.length > 3000) { textTooLong.value = true } else { textTooLong.value = false };
});

/** Save question in the cloud */
async function submitFeedback() {

  if (feedbackStatus.value === LoadingStatus.Loading) {
    console.log("Waiting for response");
    return;
  }

  // validate input
  const text = feedbackText.value.trim();

  if (text.length < 10) {
    console.log("Feedback is too short");
    textTooShort.value = true;
    return;
  } else {
    textTooShort.value = false;
  }

  if (text.length > 3000) {
    console.log("Feedback is too long");
    textTooLong.value = true;
    return;
  } else {
    textTooLong.value = false;
  }

  if (!question.value?.qid || !question.value?.topic) {
    console.log("No question ID or topic found");
    return;
  }

  feedbackStatus.value = LoadingStatus.Loading;

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(text);
  const bodyHash = toHex(await hash.digest());

  if (lastSubmissionHash === bodyHash) {
    console.log("Duplicate submission");
    feedbackStatus.value = LoadingStatus.Loaded;
    return;
  }

  // user token is optional, but it's nice to know who submitted the feedback
  const headers = new Headers();
  headers.append(AWS_BODY_HASH_HEADER, bodyHash);
  if (token.value) headers.append(TOKEN_HEADER_NAME, token.value);

  fetch(`${QUESTION_FEEDBACK_HANDLER_URL}${URL_PARAM_TOPIC}=${question.value.topic}&${URL_PARAM_QID}=${question.value.qid}`, {
    method: "POST",
    body: text,
    headers,
  }).then((response) => {
    // 204 on success, anything else is an error
    console.log("Response status: ", response.status);
    if (response.status === 204) {
      console.log("Feedback submitted OK");
      feedbackStatus.value = LoadingStatus.Loaded;
      lastSubmissionHash = bodyHash;
    }
    else {
      console.log("Failed to submit feedback: ", response.status);
      feedbackStatus.value = LoadingStatus.Error;
    }
  })
    .catch((error) => {
      console.log(error);
      feedbackStatus.value = LoadingStatus.Error;
    }
    );
}

</script>
