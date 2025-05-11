<template>
  <div v-if="user">
    <div class="mt-4 md:mt-8 mb-12">
      <div v-if="user?.topics.length" class="text-center">
        <div v-if="inUpdateMode" class="update-sub">
          <div class="flex mb-4">
            <div class="mx-auto">
              <ul>
                <li>Topics to add: </li>
                <li v-for="topic in topicsToAdd" :key="topic">{{ findTopicById(topic) }}</li>
                <li v-if="!topicsToAdd?.length">none</li>
              </ul>
              <ul>
                <li>Topics to remove: </li>
                <li v-for="topic in topicsToRemove" :key="topic">{{ findTopicById(topic) }}</li>
                <li v-if="!topicsToRemove?.length">none</li>
              </ul>
            </div>
          </div>
          <Button label="Cancel" icon="pi pi-times" raised severity="secondary" class="font-bold px-8 py-4 me-2 mb-2 whitespace-nowrap" @click="cancelInUpdateMode" />
          <Button label="Confirm" icon="pi pi-check" raised class="font-bold px-8 py-4 ms-2 mb-2 whitespace-nowrap" :disabled="!canUpdateSub" @click="subscribe" />
          <p v-if="saving == LoadingStatus.Loading" class="">Saving ...</p>
          <p v-if="saving == LoadingStatus.Error" class="text-red-600 dark:text-red-400">Failed to update your subscription. Refresh the page and try again.</p>
        </div>
      </div>

      <div v-if="!user.topics.length" class="update-sub">
        <Button label="Subscribe" icon="pi pi-envelope" raised class="" :disabled="saving == LoadingStatus.Loading" @click="subscribe" />
        <p class="my-4">
          <span v-if="topicsReminder && !canSubscribe" class="text-red-600 dark:text-red-400">Select at least one topic</span>
          <span v-else-if="saving == LoadingStatus.Loading" class="">Saving ...</span>
          <span v-else-if="saving == LoadingStatus.Error" class="text-red-600 dark:text-red-400">Failed to update your subscription. Refresh the page and try again.</span>
        </p>
      </div>

    </div>
  </div>
</template>


<script setup lang="ts">
import { computed, ref, watchEffect, watch } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { USER_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPICS, URL_PARAM_LIST_SEPARATOR, findTopicById } from "@/constants";
import { type User, LoadingStatus } from "@/interfaces";

import Button from 'primevue/button';

const store = useMainStore();
const { selectedTopics, token, user } = storeToRefs(store);

const topicsReminder = ref(false); // true if attempted to subscribe without selecting topics to show a prompt
const saving = ref<LoadingStatus>(); // set while updating user subs, only loading and error are used, otherwise undefined

/// A new sub requires at least one topic, an update may have no topics selected
const canSubscribe = computed(() => selectedTopics.value.length || user.value?.topics.length);

// enables the confirm/cancel form part
const inUpdateMode = computed(() => topicsToAdd.value?.length || topicsToRemove.value?.length);

const topicsToRemove = computed(() => {
  return user.value?.topics.filter((t) => !selectedTopics.value.includes(t));
});

const topicsToAdd = computed(() => {
  return selectedTopics.value.filter((t) => !user.value?.topics.includes(t));
});

// enables / disables the confirm button
const canUpdateSub = computed(() => topicsToAdd.value.length || topicsToRemove.value?.length);

// resets the list of selected topics to what the user is subscribed to
function cancelInUpdateMode() {
  selectedTopics.value = user.value?.topics.length ? user.value.topics : [];
}

// updates the user subscription in the DB and the store
async function subscribe() {
  // console.log("Subscribing to topics: ", selectedTopics.value);
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);

  // if no topics are selected, show a prompt and return
  if (!canSubscribe.value) {
    topicsReminder.value = true;
    return;
  }

  if (!token.value) {
    console.log("No token found.");
    return; // unreachable code
  }

  // create a URL with list of topics
  const url = `${USER_HANDLER_URL}${URL_PARAM_TOPICS}=${subTopics}`;
  try {
    saving.value = LoadingStatus.Loading;
    const response = await fetch(url, {
      headers: {
        [TOKEN_HEADER_NAME]: token.value,
      },
    });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      console.log("Subscribed successfully");
      try {
        user.value = <User>await response.json();
        console.log(user.value);

        // set selected topics to user's topics
        selectedTopics.value = user.value.topics;
        saving.value = undefined;
      } catch (error) {
        saving.value = LoadingStatus.Error;
        console.error(error);
      }

    }
    else {
      saving.value = LoadingStatus.Error;
      console.error("Failed to subscribe: ", response.status);
    }
  } catch (error) {
    saving.value = LoadingStatus.Error;
    console.error(error);
  }
}

// enable editing form if there are changes
watch(selectedTopics, (newVal, oldVal) => {
  // reset the reminder to select at least one topic for a new subscription
  if (newVal.length) topicsReminder.value = false;
});

watchEffect(() => {
  if (user.value) {
    // set selected topics to user's topics
    selectedTopics.value = user.value.topics;
  }
});

</script>
