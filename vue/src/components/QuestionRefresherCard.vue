<template>
  <div class="q-explainer">
    <p>Check out these resources to refresh your knowledge about this subject:</p>
    <ul>
      <li v-for="(link, index) in question?.refresherLinks" :key="index">
        <a :href="link" target="_blank">{{ link }}</a>
      </li>
    </ul>

    <p class="text-xs">Does the question sound confusing or like something not worth knowing?</p>
    <p class="text-xs mb-6">We'd like to hear your feedback.</p>

    <Tag value="Hide refresher" icon="pi pi-sort-up-fill" severity="secondary" class="explain-link" @click.prevent="refresherLinksToggleFlag = false" />
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import Tag from "primevue/tag";

const store = useMainStore();
const { question, refresherLinksToggleFlag } = storeToRefs(store);

</script>
