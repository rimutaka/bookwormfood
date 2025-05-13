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

  /** Extracted from URL path */
  const isbn = computed(() => {
    const x = route.path.match(/\/\d{13}(\/|$)/)?.[0]?.replace(/\//g, "") || "";
    console.log("isbn", x)
    return x
  })

  /** Extracted from URL path */
  const readerId = computed(() => {
    return route.path.match(/\/reader-\d+(\/|$)/)?.[0]?.replace(/\//g, "")?.replace("reader-", "")
  })


  const reset = () => {
    email.value = undefined;
    token.value = undefined;
  }

  return {
    isbn,
    readerId,
    email,
    token,
    reset,
  }
})