<template>
  <div>
    <h3 class="">Select your topic of interest</h3>
    <TopicList :as-links="true" :as-radio="true" />
    <div v-if="topic">
      <QuestionList :topic="topic" :key="topic" class="mt-12" />
      <div v-if="!email && questionsWithHistory?.length" class="mb-12 mt-12 cta-box">
        <!-- Show it to logged out users -->
        <QuestionListCTASignin />
      </div>
      <div v-if="email && !user?.topics?.length && questionsWithHistory?.length" class="mb-12 mt-12 cta-box">
        <!-- Show it to logged in users with no subscriptions -->
        <QuestionListCTASubscribe />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watchEffect } from 'vue';
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import QuestionListCTASignin from '@/components/QuestionListCTASignin.vue';
import QuestionList from '@/components/QuestionList.vue';
import QuestionListCTASubscribe from '@/components/QuestionListCTASubscribe.vue';
import TopicList from '@/components/TopicList.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';

const route = useRoute();

const store = useMainStore();
const { email, user, questionsWithHistory } = storeToRefs(store);

const topic = ref();

watchEffect(() => {
  topic.value = route.query.topic ? <string>route.query.topic : "";
});

</script>