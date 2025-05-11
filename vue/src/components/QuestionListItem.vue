<template>
  <div :class="status.cssClass">
    <p>
      <i v-if="isAuthor" class="author"></i>
      <i v-else></i>
      <router-link :to="questionTopicAndPageUrl" :aria-label="status.aria + title">{{ title }}</router-link>
      <span v-if="showTopic" class="tag">{{ topicName }}</span>
      <span v-if="isDraft" class="tag draft">draft</span>
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import router, { PageIDs } from "@/router";

import type { QuestionWithHistory } from "@/interfaces";
import { PublishStage } from "@/interfaces"
import * as constants from "@/constants";

const props = defineProps<{
  /** The question to display */
  question: QuestionWithHistory,
  /** Display the topic name if True */
  showTopic?: boolean,
  /** Taken from store.user to match with the question author hash */
  user_email_hash?: string,
}>()

const title = computed(() => props.question.question?.title ? props.question.question?.title : "Untitled question");

const topicName = computed(() => constants.findTopicById(props.question.question.topic));

const isAuthor = computed(() => props.showTopic || (props.user_email_hash != undefined && props.user_email_hash === props.question.question.author));

const isDraft = computed(() => props.question.question.stage == PublishStage.Draft);

// contains CSS class and aria-label for the question
const status = computed((): { cssClass: string, aria: string } => {
  if (props.question?.history?.[0].correct) return { cssClass: `correct`, aria: "Correctly answered before, " };
  if (props.question?.history?.[0].incorrect) return { cssClass: "incorrect", aria: "Incorrect answer last time, " };
  if (props.question?.history?.[0]) return { cssClass: "viewed", aria: "Viewed before, " };
  return { cssClass: "", aria: "" }; // not viewed
});

/// A URL to the page with the question on its own
/// for sharing or opening separately
const questionTopicAndPageUrl = computed(() => `/${PageIDs.QUESTION}?${constants.URL_PARAM_TOPIC}=${props.question.question.topic}&${constants.URL_PARAM_QID}=${props.question.question.qid}`);

</script>
