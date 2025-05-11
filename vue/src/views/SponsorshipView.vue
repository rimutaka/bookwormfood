<template>
  <div class="mb-12 md:mb-20">
    <div class="flex mb-12 md:mb-20">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="card flex-grow md:flex-shrink cta-box">
        <SponsorshipCTA />
        <fieldset class=" border rounded-md border-dotted border-slate-500 mb-12 mt-8 py-4">
          <legend class="mx-auto text-sm font-bold text-slate-500 dark:text-slate-300">Your <em>thank you</em> message</legend>
          <ContributorCard class="my-4" />
        </fieldset>
        <div class="text-xs"> Add your details:
          <input type="radio" class="ms-2 me-1" id="addContributorTrue" name="addContributor" :value="false" v-model="anonymousContributor" />
          <label class="me-4" for="addContributorTrue">now</label>
          <input type="radio" class="ms-2 me-1" id="addContributorFalse" name="addContributor" :value="true" v-model="anonymousContributor" />
          <label for="addContributorFalse" class="">later</label>
          <div v-if="!anonymousContributor">
            <ContributorForm class="mt-4" :autosave="true" />
            <p class="text-slate-500 dark:text-slate-300 mt-2">See live questions for <router-link :to="PageIDs.QUESTION">attribution examples</router-link></p>
          </div>
          <p v-else class="text-slate-500 dark:text-slate-300 mt-2">We will contact you to confirm your details (<router-link :to="PageIDs.QUESTION">examples</router-link>)</p>
        </div>
      </div>
      <div class="flex-shrink md:flex-grow">
      </div>
    </div>

    <SponsorshipForm />
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { CONTRIBUTOR_DETAILS_LS_KEY } from "@/constants";
import type { Question, ContributorProfile } from '@/interfaces';
import { CONTRIBUTOR_PLACEHOLDER } from '@/interfaces';
import { PageIDs } from '@/router';

import SponsorshipCTA from '@/components/SponsorshipCTA.vue';
import SponsorshipForm from '@/components/SponsorshipForm.vue';
import ContributorCard from '@/components/ContributorCard.vue';
import ContributorForm from '@/components/ContributorForm.vue';


// contributor form uses questionMD, so the contributor details have to go via that object
const store = useMainStore();
const { questionMD, anonymousContributor } = storeToRefs(store);

// load them once at the start
const contributorDetailsInLS = localStorage.getItem(CONTRIBUTOR_DETAILS_LS_KEY);


watch(anonymousContributor, (anonymousContributorNew, anonymousContributorOld) => {
  console.log("anonymousContributor changed (old/new): ", anonymousContributorOld, anonymousContributorNew);
  if (!questionMD.value) {
    console.log("Can't add contributor details - missing `question` in `store`");
    return
  };
  if (anonymousContributorOld === undefined && anonymousContributorNew === true && !contributorDetailsInLS) {
    console.log("Fist load - display contributor placeholder");
    questionMD.value.contributor = CONTRIBUTOR_PLACEHOLDER;
  }
  else if (anonymousContributorNew) {
    console.log("Hide contributor details");
    questionMD.value.contributor = undefined;
  } else {
    // contributor details will be on auto-save, so we can retrieve them from the local storage
    console.log("Restore contributor details");
    const fromLS = localStorage.getItem(CONTRIBUTOR_DETAILS_LS_KEY);
    questionMD.value.contributor = fromLS ? <ContributorProfile>JSON.parse(fromLS) : CONTRIBUTOR_PLACEHOLDER;
  }
});

watchEffect(() => {

  // answers are needed to show the contributor section in full contrast
  // otherwise it will show in a subdued state
  const answers = [{ e: "lorem" }];

  // show placeholder values if there is no contributor details in the local storage
  // it happens for the first time sponsors
  // the LS get wither contrib details or {} if they chose to be anonymous
  let contributor = CONTRIBUTOR_PLACEHOLDER;
  // by default, hide the contributor form to minimize the screen clutter for first time contributors
  anonymousContributor.value = true;

  if (contributorDetailsInLS) {
    // set the contributor state to what is in the LS
    const contributorParsed = <ContributorProfile>JSON.parse(contributorDetailsInLS);
    contributor = contributorParsed;

    // show the contributor form with the details from the local storage
    if (contributorParsed.name) anonymousContributor.value = false;
  }

  questionMD.value = <Question>{
    answers,
    contributor,
  }
});

</script>
