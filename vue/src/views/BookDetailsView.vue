<template>
  <div>
    <div class="resultModal">
      <div class="result">
        <div v-if="book?.title || isbn">
          <div>
            <h3 v-if="book?.title" class="fade-in font-bold">{{ book.title }}</h3>
            <p v-if="book?.authors" class="fade-in">by {{ book.authors[0] }}</p>

            <!-- Description with conditional expanding -->
            <p v-if="book?.volumeInfo?.description && !book?.volumeInfo?.description.includes('undefined')" class="fade-in py-2 text-xs max-w-prose">
              <template v-if="book?.volumeInfo?.description.length > 500">
                {{ book?.volumeInfo?.description.substring(0, 200) }}
                <span :class="descriptionExpanded ? 'descr-full' : 'descr-collapsed'">
                  <span v-if="!descriptionExpanded" class="descr-expand" @click="descriptionExpanded = true">more</span>
                  <span :class="descriptionExpanded ? '' : 'descr-extra-text'">{{ book?.volumeInfo?.description.substring(200) }}</span>
                </span>
              </template>
              <template v-else>
                {{ book?.volumeInfo?.description }}
              </template>
            </p>

            <p class="py-2 text-xs">ISBN: {{ isbn }}</p>
          </div>
          <div class="book-actions">
            <i title="Read later" id="status-later" :class="['icon-alarm', { active: book?.readStatus == ReadStatus[ReadStatus.ToRead] }]" @click.prevent="onClickStatusToRead"></i>
            <i title="Done reading it" id="status-read" :class="['icon-checkmark', { active: book?.readStatus == ReadStatus[ReadStatus.Read] }]" @click.prevent="onClickStatusRead"></i>
            <i title="Liked it!" id="status-liked" :class="['icon-heart', { active: book?.readStatus == ReadStatus[ReadStatus.Liked] }]" @click.prevent="onClickStatusLiked"></i>
            <span class="grow"></span>
            <i title="Bin it" id="status-bin" class="icon-bin text-slate-500" @click.prevent="onClickStatusBin"></i>
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
        <button @click.prevent="onClickMyBooks">MY BOOKS</button>
        <button @click.prevent="onClickBackHandler">SCAN AGAIN</button>
        <button id="copyToClip" @click.prevent="onClickCopyToClipboard">SHARE</button>
      </div>
      <div v-if="book?.cover" class="book-cover fade-in">
        <img :src="book.cover" alt="Book cover" />
      </div>
      <div v-if="book?.photos && book?.photos.length > 0" class="book-cover fade-in">
        <div v-for="photo in book.photos" :key="photo" class="max-w-32 mb-6">
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
import { computed, watchEffect, watch, ref, onBeforeMount, onBeforeUnmount } from 'vue';
import { useRoute } from 'vue-router';
import router from '@/router';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import initWasmModule, { get_book_data, update_book_status, delete_book, upload_pic, ReadStatus } from '@/wasm-rust/isbn_mod'
import type { Book } from '@/interfaces.js';
import { buildBookUrl } from '@/interfaces.js';


const route = useRoute()
const store = useMainStore();
const { token, isbn, readerId } = storeToRefs(store);

// State
const book = ref<Book>()
const selectedFile = ref<FileList>()
const descriptionExpanded = ref(false)

// Handle messages from WASM module
const handleWasmMessage = (msg: MessageEvent) => {
  let data
  try {
    data = JSON.parse(msg.data)
  } catch (e) {
    // Parsing error - likely not a WASM message
    return
  }

  // Process WASM response
  if (data?.localBook?.Ok) {
    book.value = data.localBook.Ok
    if (!book.value) {
      console.log("No book found")
      return
    }
    if (!book.value.title) {
      book.value.title = "No data in Google for this ISBN code"
    }

    book.value.photos = book.value.photos || []

    // Update URL with book title
    const url = buildBookUrl(book.value, readerId.value)
    router.replace(`/${url}`)
  } else if (data?.deleted?.Ok) {
    console.log("Book deletion confirmed")
    router.push("/")
  } else if (data?.uploaded?.Ok) {
    console.log("File uploaded:", data.uploaded.Ok)
  } else {
    book.value = { title: "Cannot get data from Google for this book", authors: [], cover: "", volumeInfo: { description: "" }, isbn: 0, readStatus: "ToRead", photos: [], shareId: undefined }
  }
}
// Initialize WASM and fetch book data
watchEffect(async () => {
  if (isbn.value) {
    console.log(`ISBN: ${isbn.value}, Reader ID: ${readerId.value}, URL: ${route.path}`)

    // Get book details
    await initWasmModule()
    get_book_data(isbn.value, token.value, readerId.value)
  }
})

// Setup event listeners
onBeforeMount(() => {
  window.addEventListener("message", handleWasmMessage)
})

onBeforeUnmount(() => {
  window.removeEventListener("message", handleWasmMessage)
})

// Event handlers
const onClickStatusToRead = () => {
  update_book_status(isbn.value, book.value?.readStatus == ReadStatus[ReadStatus.ToRead] ? undefined : ReadStatus.ToRead, token.value)
}

const onClickStatusRead = () => {
  update_book_status(isbn.value, book.value?.readStatus == ReadStatus[ReadStatus.Read] ? undefined : ReadStatus.Read, token.value)
}

const onClickStatusLiked = () => {
  update_book_status(isbn.value, book.value?.readStatus == ReadStatus[ReadStatus.Liked] ? undefined : ReadStatus.Liked, token.value)
}

const onClickStatusBin = () => {
  delete_book(isbn.value, token.value)
}

const onClickMyBooks = () => {
  router.push("/")
}

const onClickBackHandler = () => {
  router.push("/scan")
}

const onClickCopyToClipboard = async () => {
  const url = book.value?.shareId ?
    (window.location.href.replace(/\/$/, "") + "/reader-" + book.value.shareId) :
    window.location.href

  await navigator.clipboard.writeText(url)
  const btnId = document.getElementById("copyToClip")
  if (!btnId) {
    console.error("Button element not found")
    return
  }
  btnId.innerText = "COPIED TO CLIPBOARD"
  btnId.classList.add("done")

  setTimeout(() => {
    btnId.innerText = "SHARE"
    btnId.classList.remove("done")
  }, 3000)
}

function handleFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files || !input.files.length) {
    console.log("No files selected.")
    return
  }
  selectedFile.value = input.files

  try {
    console.log(`Uploading file: ${input.files[0]?.name}`)
    upload_pic(isbn.value, input.files, token.value)
    console.log("File queued for uploading")
  } catch (error) {
    console.error("Error queuing file for uploading:", error)
  }
}

// Update document title when title changes
watch([book], ([newBook]) => {
  const newTitle = newBook?.title;
  const newAuthors = newBook?.authors?.[0];
  if (newTitle) {
    document.title = `${newTitle}${newAuthors ? ' by ' + newAuthors : ''}`
  } else {
    document.title = "Book not found"
  }
}, { deep: true })

</script>