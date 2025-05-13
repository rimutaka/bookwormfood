import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'


/// The main store for the application
export const useMainStore = defineStore('main', () => {

  const route = useRoute();

  /// Email from the token from the ID provider, e.g. Google or Github
  const email = ref<string | undefined>()

  /// Raw token from the ID provider, e.g. Google or Github
  const token = ref<string | undefined>()

  const reset = () => {
    email.value = undefined;
    token.value = undefined;
  }

  return {
    email,
    token,
    reset,
  }
})