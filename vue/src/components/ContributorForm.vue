<template>
  <div>
    <div class="flex flex-wrap gap-4 mb-4">
      <InputText type="text" v-model="contributorName" placeholder="Your name" size="small" class="flex-grow" />
      <InputText type="text" v-model="contributorProfileUrl" placeholder="Link to your profile or a project" size="small" class="flex-grow" />
      <InputText type="text" v-model="contributorImageUrl" placeholder="Link to your logo or avatar" size="small" class="flex-grow" />
    </div>
    <InputText type="text" v-model="contributorAbout" placeholder="Something about you" size="small" class="w-full mb-2" />
    <div v-if="!autosave" class="text-end">
      <p v-if="isDifferentFromLS && contributorInLS" class="text-xs">
        Set to <span class="link" @click.prevent="applyDefaultContributorDetails">{{ contributorInLS?.name }}</span>
        or <span class="link" @click.prevent="saveDefaultContributorDetails">save</span> these contributor details as my default
      </p>
      <p v-else-if="isDifferentFromLS" class="text-xs"><span class="link" @click.prevent="saveDefaultContributorDetails">Save</span> these contributor details as my default</p>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import type { ContributorProfile } from '@/interfaces'
import { CONTRIBUTOR_DETAILS_LS_KEY, } from "@/constants";
import { CompareContributors } from '@/interfaces'
import debounce from "lodash.debounce"

import InputText from 'primevue/inputtext';

const props = defineProps<{
  /** Hides save/apply message and does both automatically */
  autosave?: boolean,
}>();

const store = useMainStore();
const { questionMD } = storeToRefs(store);

// this relies on the component being loaded after the store was updated with the data
// fetched from the server
const contributorName = ref(questionMD.value?.contributor?.name || "");
const contributorProfileUrl = ref(questionMD.value?.contributor?.url);
const contributorImageUrl = ref(questionMD.value?.contributor?.imgUrl);
const contributorAbout = ref(questionMD.value?.contributor?.about);

const contributorInLsOnMount = localStorage.getItem(CONTRIBUTOR_DETAILS_LS_KEY);
const contributorInLS = ref(contributorInLsOnMount ? <ContributorProfile>JSON.parse(contributorInLsOnMount) : undefined);

// true if the current values are different from the what is in the local storage
const isDifferentFromLS = ref(!CompareContributors(questionMD.value?.contributor, contributorInLS.value));

/// Saves the default contributor details to local storage
const saveDefaultContributorDetails = () => {
  const contributorDetails = <ContributorProfile>{
    name: contributorName.value,
    url: contributorProfileUrl.value,
    imgUrl: contributorImageUrl.value,
    about: contributorAbout.value,
  };

  localStorage.setItem(CONTRIBUTOR_DETAILS_LS_KEY, JSON.stringify(contributorDetails));
  contributorInLS.value = contributorDetails;
  isDifferentFromLS.value = false;
}

/// Copies the default contributor from LS into the question
const applyDefaultContributorDetails = () => {
  if (questionMD.value && contributorInLS.value?.name) {
    // setting the local refs here because setting the question in store does not update the local refs
    // the store is updated via watch
    contributorName.value = contributorInLS.value?.name;
    contributorProfileUrl.value = contributorInLS.value?.url;
    contributorImageUrl.value = contributorInLS.value?.imgUrl;
    contributorAbout.value = contributorInLS.value?.about;

    isDifferentFromLS.value = false;
  }
  else {
    console.log('No question in storage');
  }
}

/// Slows down auto-saves of the defaults
const debounceContributorDetails = debounce(() => {
  saveDefaultContributorDetails();
}, 2000);

watch([contributorName, contributorProfileUrl, contributorImageUrl, contributorAbout], ([name, profileUrl, imgUrl, about]) => {
  console.log('Contributor details changed');
  if (!questionMD.value) {
    console.log('No question in storage');
    return;
  }
  if (!questionMD.value?.contributor) {
    // this should not happen, but in case the structure is not there we create it from scratch
    questionMD.value.contributor = <ContributorProfile>{ name, url: profileUrl, imgUrl: imgUrl, about };
  }
  else {
    // update the values since the structure may have other fields we should not overwrite
    questionMD.value.contributor.name = name;
    questionMD.value.contributor.url = profileUrl;
    questionMD.value.contributor.imgUrl = imgUrl;
    questionMD.value.contributor.about = about;
  }

  // enable / disable Save as default button based on whether the current values are different from the saved values
  isDifferentFromLS.value = !CompareContributors(questionMD.value.contributor, contributorInLS.value);

  if (isDifferentFromLS.value && props.autosave) {
    console.log('Autosaving contributor details');
    debounceContributorDetails();
  }
});

</script>