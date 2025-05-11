<template>
  <h3 class="mb-8">Question source review</h3>
  <QuestionReviewForm :topic="<string>route.query.topic" :qid="<string>route.query.qid" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import QuestionReviewForm from '@/components/QuestionReviewForm.vue';

const route = useRoute();

</script>