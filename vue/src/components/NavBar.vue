<template>
  <nav class="mx-auto py-4 border-t mt-12 text-xs">
    <ul class="flex flex-wrap font-small">
      <li class="flex-none"><a href="/about">Privacy policy</a></li>
      <li class="flex-grow text-center"><span class="align-baseline">{{ appVersion }}</span></li>
      <li class="flex-none">
        <div v-if="auth.isAuthenticated" v-once>
          <button class="loginBtn" @click="handleLogout" type="button">Sign out</button>
        </div>
        <div v-else v-once>
          <button class="loginBtn" @click="handleLogin" type="button" title="Login to save your books to in the cloud">
            <svg class="mr-2 -ml-1 w-4 h-4" aria-hidden="true" focusable="false" data-prefix="fab" data-icon="google" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 488 512"><path fill="currentColor" d="M488 261.8C488 403.3 391.1 504 248 504 110.8 504 0 393.2 0 256S110.8 8 248 8c66.8 0 123 24.5 166.3 64.9l-67.5 64.9C258.5 52.6 94.3 116.6 94.3 256c0 86.5 69.1 156.6 153.7 156.6 98.2 0 135-70.4 140.8-106.9H248v-85.3h236.1c2.3 12.7 3.9 24.9 3.9 41.4z"></path></svg>
            Sign in with Google
          </button>
        </div>
      </li>
    </ul>
  </nav>
</template>


<script setup lang="ts">
import { ref, watch, watchEffect, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

const store = useMainStore();

const auth = useAuth()

// Toggle sign in / sign out button
watchEffect(() => {
  // This will run whenever auth.isAuthenticated changes
  // Original effect was empty, keeping it for future implementation
})

// Handle login with redirect
async function handleLogin() {
  await auth.loginWithRedirect({
    appState: {
      returnTo: window.location.pathname,
    },
  })
}

// Handle logout
async function handleLogout() {
  await auth.logout({
    logoutParams: { 
      returnTo: "https://" + window.location.hostname + (window.location.port == 80 ? "" : ":" + window.location.port) + "/logout" 
    }
  })
}

// Calculate app version
const appVersion = computed(() => {
  // Assuming process.env is available or replaced with equivalent in Vue
  let version = import.meta.env.VITE_APP_BUILD_TS || ''
  
  if (!version) return 'v.dev'
  
  version = version.substring(5, 6) == "0"
    ? version.substring(6, 16) // do not show leading zero for 08-25
    : version.substring(5, 16)
    
  // convert it to v.825.1608
  return "v." + version.replace("-", "").replace(":", "").replace(" ", ".")
})

</script>