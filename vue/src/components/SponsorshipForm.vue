<template>
  <div>
    <div>
      <div class="flex flex-wrap gap-4 mb-4 items-center">
        <label for="qty-input">Number of questions:</label>
        <InputText type="text" v-model="qty" size="small" :invalid="qtyNumber < 1 || qtyNumber > MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT" class="w-12" id="qty-input" />
        =<span><span class="text-xs align-text-top">US$</span>{{ price * qtyNumber }}</span>
      </div>
      <div class="flex flex-wrap gap-4 mb-4 items-center">
        <label for="qty-input">Preferred topics:</label>
        <InputText type="text" v-model="topics" placeholder="Topics" size="small" class=" flex-grow" />
      </div>
    </div>
    <div class="flex-shrink text-end w-fit ms-auto mt-8">
      <Button label="Secure payment with Stripe" icon="pi pi-credit-card" raised class="my-auto whitespace-nowrap" @click="get_checkout_url()">
        <span class="p-button-label" id="stripe-logo" data-pc-section="label">Secure payment with </span>
      </Button>
      <div class="w-full h-6" id="secure-payments" aria-label="Supports google pay, apple pay and major cards"></div>
      <p v-if="qtyNumber < 1 || qtyNumber > MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT" class="text-red-500 text-base mt-2">Maximum {{ MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT }} questions per transaction</p>

    </div>
    <LoadingMessage v-if="loadingStatus == LoadingStatus.Loading" />
    <h3 v-if="loadingStatus == LoadingStatus.Error" class="mt-8 mb-8 text-center text-slate-500 dark:text-slate-200">Sorry, something went wrong. Try again.</h3>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, computed, watchEffect } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import type { LoadingStatus as LoadingStatusType, QuestionSponsorship } from '@/interfaces'
import { PAYMENTS_HANDLER_URL, SPONSOR_DETAILS_LS_KEY, CONTRIBUTOR_DETAILS_LS_KEY, MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT, AWS_BODY_HASH_HEADER } from "@/constants";
import { LoadingStatus } from "@/interfaces";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import debounce from "lodash.debounce"

import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import LoadingMessage from "./LoadingMessage.vue";
import { PageIDs } from "@/router";


const store = useMainStore();
const { question, anonymousContributor } = storeToRefs(store);

const price = 50;
const qty = ref("1");
const topics = ref("any topic");
const loadingStatus = ref<LoadingStatusType>(LoadingStatus.Loaded);

const sponsorDetailsInLS = localStorage.getItem(SPONSOR_DETAILS_LS_KEY);

const qtyNumber = computed(() => +qty.value || 0);

/// Saves the default contributor details to local storage
const saveDefaultSponsorDetails = () => {
  console.log("Saving default sponsor details");

  // only what is needed to restore the defaults for the next session
  const sponsorDetails = <QuestionSponsorship>{
    qty: +qty.value,
    topics: topics.value,
  };
  localStorage.setItem(SPONSOR_DETAILS_LS_KEY, JSON.stringify(sponsorDetails));
}

/** Calls a lambda to create a checkout page for this transaction and redirects the user to it */
async function get_checkout_url() {

  // quantity is the only required field and must be less than ...
  // it's a pointless check because it can be changed at the checkout
  if (qtyNumber.value < 1 || qtyNumber.value > MAX_NUMBER_OF_QUESTIONS_PER_PAYMENT) {
    console.log("Invalid quantity: ", qty.value);
    return;
  }

  loadingStatus.value = LoadingStatus.Loading;

  saveDefaultSponsorDetails();

  // this may potentially create problems for someone who write questions and sponsors,
  // but it's unlikely to be an issue for now
  if (anonymousContributor.value) {
    console.log("Anonymous - deleting contributor details from LS");
    localStorage.setItem(CONTRIBUTOR_DETAILS_LS_KEY, JSON.stringify({}));
  }

  // the lambda gets all it needs from the serialized JSON object
  const questionDonation = <QuestionSponsorship>{
    qty: +qty.value,
    topics: topics.value,
    contributor: anonymousContributor.value ? undefined : question.value?.contributor, // this struct is set by a sub-component
    cancelUrl: window.location.href,
    successUrl: `${window.location.origin}/${PageIDs.THANKYOU}`,
  };

  // the lambda gets all it needs from the serialized JSON object
  const questionDonationStr = JSON.stringify(questionDonation);
  console.log(questionDonationStr);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(questionDonationStr);
  const bodyHash = toHex(await hash.digest());

  const response = await fetch(`${PAYMENTS_HANDLER_URL}`, {
    method: "POST",
    body: questionDonationStr,
    headers: {
      [AWS_BODY_HASH_HEADER]: bodyHash,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the error occurred
  console.log("Response status: ", response.status);
  if (response.status === 200) {
    try {
      const checkoutUrl = await response.text();
      if (checkoutUrl?.startsWith("https://")) {
        console.log("Redirecting to checkout page: ", checkoutUrl);
        window.location.href = checkoutUrl;
      }
      else {
        // it is probably a lambda bug if the status = 200 and the URL is not valid
        console.log("Invalid checkout URL: ", checkoutUrl);
        loadingStatus.value = LoadingStatus.Error;
      }
    } catch (error) {
      console.error(error);
      loadingStatus.value = LoadingStatus.Error;
    }
  } else {
    console.error("Failed to get checkout URL: ", response.status);
    loadingStatus.value = LoadingStatus.Error;
  }
}

/// Slows down auto-saved of the defaults
const debounceSponsorshipDetails = debounce(() => {
  saveDefaultSponsorDetails();
}, 2000);

watch([qty, topics], () => {
  debounceSponsorshipDetails();
});

watchEffect(() => {
  // get the list of topics from the default contributor details saved in local storage from the previous time
  const sponsorDetails = sponsorDetailsInLS ? <QuestionSponsorship>JSON.parse(sponsorDetailsInLS) : undefined;
  if (sponsorDetails?.topics) topics.value = sponsorDetails.topics;
  if (sponsorDetails?.qty) qty.value = sponsorDetails.qty.toString();
});

</script>