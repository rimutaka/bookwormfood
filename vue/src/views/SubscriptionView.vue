<template>
  <LoadingMessage v-if="!user" msg="Loading your subscription ..." />

  <SubscriptionTopics v-if="user" />
  <SubscriptionForm class="mb-12" />

  <LinkButton v-if="user" :href="`/${PageIDs.QUESTIONS}`" icon="pi pi-sparkles" class="mb-12" label="Browse all questions" />

  <TransitionSlot>
    <SubscriptionCTA v-if="user && !user?.topics.length && !questionsWithHistory?.length" class="mb-12" />
  </TransitionSlot>
  <TransitionSlot>
    <SubscriptionCompleted v-if="user?.topics.length && !questionsWithHistory?.length" class="mb-12" />
  </TransitionSlot>

  <h3 v-if="user && questionsWithHistory?.length" class="mb-8">Your questions</h3>
  <QuestionList v-if="user" :topic="topic" :key="topic" />
  <div class="hidden">
    <!-- Needed to load styles for LinkButton -->
    <Button />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, watchEffect } from 'vue';
import { storeToRefs } from 'pinia'
import { useRoute } from 'vue-router'
import { useMainStore } from '@/store';
import { PageIDs } from '@/router';


import TransitionSlot from "@/components/TransitionSlot.vue";
import SubscriptionCTA from '@/components/SubscriptionCTA.vue';
import SubscriptionCompleted from '@/components/SubscriptionCompleted.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';
import QuestionList from '@/components/QuestionList.vue';
import SubscriptionTopics from '@/components/SubscriptionTopics.vue';
import LinkButton from '@/components/LinkButton.vue';
import { Button } from 'primevue';

const route = useRoute();

const store = useMainStore();
const { user, questionsWithHistory } = storeToRefs(store);

const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics
const topic = ref<string | undefined>();

watch(user, (newUser) => {
  if (newUser && !newUser.topics.length) {
    firstTimeSub.value = true;
  }
});

watchEffect(() => {
  topic.value = route.query.topic ? <string>route.query.topic : undefined;
});

</script>
