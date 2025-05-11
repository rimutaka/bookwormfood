<template>
  <div class="w-full mb-4 flex gap-2 md:gap-8 justify-center border-t pt-2 mt-12 text-sm md:text-base">
    <template v-for="item in menuItems" :key="item.label">
      <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
        <a :href="href" @click="navigate">
          <span :class="item.icon" />
          <span class="ms-2">{{ item.label }}</span>
        </a>
      </router-link>
    </template>
    <a v-if="isAuthenticated" @click="signout">
      <span class="pi pi-sign-out" />
      <span class="ms-2">Logout</span>
    </a>
    <a v-else @click="signin">
      <span class="pi pi-sign-in" />
      <span class="ms-2">Login</span>
    </a>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import router from "@/router";
import { PageIDs } from '@/router';
import { useMainStore } from '@/store';
import { useAuth0 } from '@auth0/auth0-vue';
import { LAST_AUTH_TIMESTAMP } from "@/constants";

const { isAuthenticated, loginWithRedirect, logout } = useAuth0();
const store = useMainStore();

const menuItems = ref([
  {
    label: 'Home',
    icon: 'pi pi-home',
    route: '/'
  },
  {
    label: 'Questions',
    icon: 'pi pi-list-check',
    route:  `/${PageIDs.QUESTIONS}`
  },
  {
    label: 'Contribute',
    icon: 'pi pi-heart text-red-500',
    route: `/${PageIDs.CONTRIBUTE}`
  },
  {
    label: 'About',
    icon: 'pi pi-info-circle',
    route: `/${PageIDs.ABOUT}`
  }
]);

/// Auth0 login
// TODO: move this to a shared file, but that requires making Auth0 working outside the component
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

/// Auth0 logout
function signout() {
  if (isAuthenticated.value) {
    localStorage.removeItem(LAST_AUTH_TIMESTAMP);
    store.reset();
    logout({
      openUrl(url) {
        console.log("Redirecting to logout: ", url);
        router.push("/");
      }
    });
    console.log("Signed out. LAST_AUTH_TIMESTAMP deleted.");

  } else {
    console.log("Already signed out");
  }
}

</script>