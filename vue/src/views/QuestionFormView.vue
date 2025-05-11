<template>
  <h3 v-if="route.query.qid" class="mb-8">Edit question about <em>{{ findTopicById(topic) }}</em></h3>
  <h3 v-else class="mb-8">Add new question</h3>
  <QuestionForm :topic="<string>route.query.topic" :qid="<string>route.query.qid" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { findTopicById } from '@/constants';
import QuestionForm from '../components/QuestionForm.vue';

const route = useRoute();

const topic = computed(() => <string>route.query.topic);

</script>