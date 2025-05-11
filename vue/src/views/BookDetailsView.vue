<template>
  <div>
    <div class="resultModal">
      <div class="result">
        <div v-if="title || isbn">
          <div>
            <h3 v-if="title" class="fade-in font-bold">{{ title }}</h3>
            <p v-if="authors" class="fade-in">by {{ authors }}</p>

            <!-- Description with conditional expanding -->
            <p v-if="description && !description.includes('undefined')" class="fade-in py-2 text-xs max-w-prose">
              <template v-if="description.length > 500">
                {{ description.substring(0, 200) }}
                <span :class="descriptionExpanded ? 'descr-full' : 'descr-collapsed'">
                  <span v-if="!descriptionExpanded" class="descr-expand" @click="descriptionExpanded = true">more</span>
                  <span :class="descriptionExpanded ? '' : 'descr-extra-text'">{{ description.substring(200) }}</span>
                </span>
              </template>
              <template v-else>
                {{ description }}
              </template>
            </p>

            <p class="py-2 text-xs">ISBN: {{ isbn }}</p>
          </div>
          <div class="book-actions">
            <i title="Read later" id="status-later" :class="['icon-alarm', { active: status == ReadStatus[0] }]" @click="onClickStatusToRead"></i>
            <i title="Done reading it" id="status-read" :class="['icon-checkmark', { active: status == ReadStatus[1] }]" @click="onClickStatusRead"></i>
            <i title="Liked it!" id="status-liked" :class="['icon-heart', { active: status == ReadStatus[2] }]" @click="onClickStatusLiked"></i>
            <span class="grow"></span>
            <i title="Bin it" id="status-bin" class="icon-bin text-slate-500" @click="onClickStatusBin"></i>
          </div>
          <div class="result-table">
            <div>
              <h3 class="about">About</h3>
              <p><a :href="`https://www.goodreads.com/search?q=${isbn}`">GoodReads</a></p>
              <p><a :href="`https://app.thestorygraph.com/browse?search_term=${isbn}`">StoryGraph</a></p>
              <p><a :href="`https://www.google.com/search?tbo=p&tbm=bks&q=isbn:${isbn}`">Google books</a></p>
            </div>
            <div>
              <h3 class="buy">Buy</h3>
              <p><a :href="`https://www.thenile.co.nz/search?s.q=${isbn}`">The Nile</a></p>
              <p><a :href="`https://www.amazon.com/s?k=${isbn}`">Amazon</a></p>
              <p><a :href="`https://www.mightyape.co.nz/books?q=${isbn}`">MightyApe</a></p>
            </div>
            <div>
              <h3 class="borrow">Borrow</h3>
              <p><a :href="`https://discover.aucklandlibraries.govt.nz/search?query=${isbn}`">Auckland libraries</a></p>
            </div>
          </div>
        </div>
      </div>
      <div class="scanBtn">
        <button @click="onClickMyBooks">MY BOOKS</button>
        <button @click="onClickBackHandler">SCAN AGAIN</button>
        <button id="copyToClip" @click="onClickCopyToClipboard">SHARE</button>
      </div>
      <div v-if="cover" class="book-cover fade-in">
        <img :src="cover" alt="Book cover" />
      </div>
      <div v-if="photos && photos.length > 0" class="book-cover fade-in">
        <div v-for="photo in photos" :key="photo" class="max-w-32 mb-6">
          <a :href="photo"><img :src="photo" alt="Book photo" /></a>
        </div>
      </div>
      <div class="scanBtn">
        <label for="idPicUploader" class="cursor-pointer border-2 p-2 rounded-md">Upload photo</label>
        <input id="idPicUploader" name="idPicUploader" class="visuallyhidden" type="file" accept="image/jpeg" @change="handleFileChange" />
      </div>
    </div>
  </div>

</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';

// Importing required functions and constants from external files
// Assuming these are available in your project
import { initWasmModule, get_book_data, update_book_status, delete_book, upload_pic, ReadStatus } from '@/wasm-rust/isbn_mod'

// Constants
const LAST_AUTH_TIMESTAMP = "auth"

// Helper function for building book URLs
function build_book_url(title, authors, isbn, readerId) {
  let url = (authors) ?
    (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "-by-" + authors.toLowerCase().replace(/[^a-z0-9]/g, "-").replace(/,/g, "") + "/" + isbn + "/").replace(/-{2,}/g, "-")
    : (title.toLowerCase().replace(/[^a-z0-9]/g, "-") + "/" + isbn + "/").replace(/-{2,}/g, "-");

  // reader id is only present in shared links
  if (readerId) {
    url = url.replace(/\/$/, "") + "/reader-" + readerId;
  }

  return url;
}

const router = useRouter()
const route = useRoute()
const auth = useAuth()

// Extract ISBN and readerId from URL path
const isbn = computed(() => {
  return route.path.match(/\/\d{13}(\/|$)/)?.[0]?.replace(/\//g, "") || ""
})

const readerId = computed(() => {
  return route.path.match(/\/reader-\d+(\/|$)/)?.[0]?.replace(/\//g, "")?.replace("reader-", "")
})

// State
const title = ref(null)
const authors = ref(null)
const cover = ref(null)
const status = ref(null)
const description = ref(null)
const token = ref(null)
const selectedFile = ref(null)
const photos = ref([])
const shareId = ref(null)
const descriptionExpanded = ref(false)

// Handle messages from WASM module
const handleWasmMessage = (msg) => {
  let data
  try {
    data = JSON.parse(msg.data)
  } catch (e) {
    // Parsing error - likely not a WASM message
    return
  }

  // Process WASM response
  if (data?.localBook?.Ok) {
    const book = data.localBook.Ok
    title.value = book.title || "No data in Google for this ISBN code"
    authors.value = book.authors?.join(", ")
    cover.value = book.cover
    status.value = book.readStatus
    description.value = book.volumeInfo?.description
    photos.value = book.photos || []
    shareId.value = book.shareId

    // Update URL with book title
    const url = build_book_url(title.value, authors.value, isbn.value, readerId.value)
    router.replace(`/${url}`)
  } else if (data?.deleted?.Ok) {
    console.log("Book deletion confirmed")
    router.push("/")
  } else if (data?.uploaded?.Ok) {
    console.log("File uploaded:", data.uploaded.Ok)
  } else {
    title.value = "Cannot get data from Google for this book"
  }
}

// Initialize WASM and fetch book data
watchEffect(async () => {
  if (isbn.value) {
    console.log(`ISBN: ${isbn.value}, Reader ID: ${readerId.value}, URL: ${route.path}`)
    
    // Try to get the token if authenticated
    let idTokenClaims = null
    if (auth.isAuthenticated) {
      idTokenClaims = await auth.getIdTokenClaims()
      if (idTokenClaims?.__raw) {
        token.value = idTokenClaims.__raw
      } else {
        console.log(`Missing token: ${JSON.stringify(idTokenClaims)}`)
      }
    } else {
      console.log("User is not authenticated")
    }

    // Get book details
    await initWasmModule()
    get_book_data(isbn.value, idTokenClaims?.__raw, readerId.value)
  }
})

// Setup event listeners
onMounted(() => {
  window.addEventListener("message", handleWasmMessage)
})

onUnmounted(() => {
  window.removeEventListener("message", handleWasmMessage)
})

// Event handlers
function onClickStatusToRead() {
  update_book_status(isbn.value, status.value == ReadStatus[0] ? null : ReadStatus.ToRead, token.value)
}

function onClickStatusRead() {
  update_book_status(isbn.value, status.value == ReadStatus[1] ? null : ReadStatus.Read, token.value)
}

function onClickStatusLiked() {
  update_book_status(isbn.value, status.value == ReadStatus[2] ? null : ReadStatus.Liked, token.value)
}

function onClickStatusBin(e) {
  e.preventDefault()
  delete_book(isbn.value, token.value)
}

function onClickMyBooks(e) {
  e.preventDefault()
  router.push("/")
}

function onClickBackHandler(e) {
  e.preventDefault()
  router.push("/scan")
}

async function onClickCopyToClipboard(e) {
  e.preventDefault()
  const url = shareId.value ? 
    (window.location.href.replace(/\/$/, "") + "/reader-" + shareId.value) : 
    window.location.href
    
  await navigator.clipboard.writeText(url)
  const btnId = document.getElementById("copyToClip")
  btnId.innerText = "COPIED TO CLIPBOARD"
  btnId.classList.add("done")
  
  setTimeout(() => {
    btnId.innerText = "SHARE"
    btnId.classList.remove("done")
  }, 3000)
}

function handleFileChange(event) {
  const files = event.target.files
  selectedFile.value = files
  
  if (!files || !files.length) {
    console.log("No file selected.")
    return
  }
  
  try {
    console.log(`Uploading file: ${files[0]?.name}`)
    upload_pic(isbn.value, files, token.value)
    console.log("File queued for uploading")
  } catch (error) {
    console.error("Error queuing file for uploading:", error)
  }
}

// Update document title when title changes
watch([title, authors], ([newTitle, newAuthors]) => {
  if (newTitle) {
    document.title = `${newTitle}${newAuthors ? ' by ' + newAuthors : ''}`
  } else {
    document.title = "Book not found"
  }
})

</script>