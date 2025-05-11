<template>
  <div>
    <h3 v-if="user?.topics.length">Your subscribed topics</h3>
    <h3 v-else-if="user && !user.topics.length">Select topics to subscribe</h3>
    <div v-if="user" :key="user?.updated" class="flex flex-wrap items-center gap-4 justify-center my-4">
      <div class="flex" v-for="topic in TOPICS" :key="topic.id">
        <Checkbox v-model="selectedTopics" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>

    <p v-if="user?.topics.length" class="mb-4 text-xs text-slate-500 dark:text-slate-200">Last updated: {{ updateDate }} </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia'
import { useRouter } from 'vue-router'
import { useMainStore } from '@/store';
import { TOPICS, URL_PARAM_TOPIC } from "@/constants";

import Checkbox from 'primevue/checkbox';

const router = useRouter();

const store = useMainStore();
const { user, selectedTopics } = storeToRefs(store);


/// Format RFC3339 date string to a human-readable date
const updateDate = computed(() => {
  if (!user.value?.updated) return "";
  return new Date(user.value.updated).toLocaleString();
});

</script>
