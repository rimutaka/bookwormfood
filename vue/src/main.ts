import './assets/main.css'
import 'primeicons/primeicons.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import PrimeVue from "primevue/config";
import Aura from '@primevue/themes/aura';
import { createAuth0 } from '@auth0/auth0-vue';
import { PageIDs } from './router'

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)
app.use(router)

app.use(PrimeVue, {
  theme: {
    preset: Aura
  }
})

app.use(
  createAuth0({
    domain: "login.bitesized.info",
    clientId: "p2xjvyoxb8HoKSt1QNDx7CQ8Ka2lXgUJ",
    authorizationParams: {
      redirect_uri: `${window.location.origin}/${PageIDs.LOGIN}`,
    },
    useRefreshTokens: true,
    cacheLocation: "localstorage", // dangerous if there is an XSS attack
  })
);

app.mount('#app')
