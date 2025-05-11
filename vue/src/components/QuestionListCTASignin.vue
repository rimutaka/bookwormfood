<template>
  <div>
    <h3 class="mb-4 text-slate-700 dark:text-slate-300 w-full">For registered users only</h3>
    <div class="max-w-md">
      <div class="mb-8 text-slate-700 dark:text-slate-200">
        <p class="mb-4 text-start">Signed-in users can track their progress and avoid getting the same question more than once. </p>
        <div class="text-center mx-auto">
          <Button label="Sign in" size="small" aria-label="Sign in with Google or Github" icon="pi pi-sign-in" severity="secondary" class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="signin()" />
          <p class="mb-8 text-xs text-center md:mb-auto text-slate-500 dark:text-slate-200" aria-hidden="true">
            No registration is required - sign in with <i class="pi pi-github ms-1 me-2  dark:opacity-85"></i><i class="pi pi-google me-2  dark:opacity-85"></i>
          </p>
        </div>
      </div>
    </div>

    <h3 class="mb-4 text-slate-700 dark:text-slate-300 w-full">Get questions by email</h3>
    <div class="max-w-md">
      <div class="mb-8 text-slate-700 dark:text-slate-200">
        <p class="mb-4 text-start max-w-md">You will learn more and remember it better if you answer only one or two questions per day delivered to your inbox for free.</p>
        <div class="text-center mx-auto">
          <LinkButton :href="`/${PageIDs.SUBSCRIPTION}`" label="Subscribe" :primary="true" icon="pi pi-envelope" class="mx-auto" />
        </div>
      </div>
    </div>
    <Button style="display:none" /><!--Needed to load LinkButton styles-->
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useAuth0 } from '@auth0/auth0-vue';
import router from "@/router";
import { PageIDs } from '@/router';

import Button from 'primevue/button';
import LinkButton from './LinkButton.vue';

// Auth0 login - this is a duplication
// TODO: move this to a shared file, but that requires making Auth0 working outside the component 
const { isAuthenticated, loginWithRedirect } = useAuth0();
const signin = () => {
  if (!isAuthenticated.value) {
    console.log("Not authenticated. Will redirect to ", router.currentRoute.value.fullPath);
    loginWithRedirect({
      appState: { target: router.currentRoute.value.fullPath }
    });
  } else {
    console.log("Already authenticated");
  }
}

</script>