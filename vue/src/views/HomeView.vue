<template>
  <HomeCTA />
  <div class="mt-4">
    <h3 class="">Select your topics of interest</h3>
    <TopicList :as-radio="true" />
  </div>
  <HomeCard />
</template>

<script setup lang="ts">
import { ref, watchEffect, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import router from '@/router';
import { PageIDs } from '@/router';

import HomeCard from '@/components/HomeCard.vue';
import HomeCTA from '@/components/HomeCTA.vue';
import TopicList from '@/components/TopicList.vue';

const store = useMainStore();
const { token, question } = storeToRefs(store);

/// redirect to subscription page if the user is authenticated
watchEffect(() => {
  // this redirect has to be here to redirect from homepage only
  // any other page should not redirect to sub automatically
  if (token.value) {
    console.log("Token obtained - redirecting to subscription page");
    router.replace({ name: PageIDs.SUBSCRIPTION }); // cleaner history with replace
  }
});

</script>
