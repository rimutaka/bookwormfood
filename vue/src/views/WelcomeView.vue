<template>
  <div>
    <div id="welcomeMsg" class="welcome">
      <div>
        <h1>Scan the book's barcode to learn, record or share</h1>
        <ul :class="{ hidden: books.length > 4 }">
          <li>View reviews, book and author details</li>
          <li>Borrow from Auckland Libraries</li>
          <li>Buy new or secondhand</li>
          <li>Save it in your reading list</li>
        </ul>
      </div>
    </div>
    <div class="scanBtn">
      <button @click="onScanBtnClickHandler">SCAN barcode</button>
    </div>
    <ul class="scan-list">
      <li v-for="book in books" :key="book.isbn">
        <i :class="getStatusIcon(book.readStatus)"></i>
        <a :href="buildBookUrl(book.title, book.authors?.[0], book.isbn)" @click.prevent="onBookLinkClickHandler(book)">
          {{ book.title }}
        </a>
        <span v-if="book.authors"> by {{ book.authors[0] }}</span>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, watch, onMounted, onBeforeUnmount } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import router from '@/router';

const store = useMainStore();


// Placeholder for WASM and Auth0 logic
// Replace with your actual imports and logic
// import initWasmModule, { get_scanned_books, ReadStatus } from '../wasm-rust/isbn_mod.js';

const books = ref([])
// Placeholder for token and authentication logic
const token = ref(null)
let withCloudSync = true

// Placeholder for ReadStatus enum
const ReadStatus = ['Unread', 'Read', 'Favorite']

function getStatusIcon(readStatus) {
  switch (readStatus) {
    case ReadStatus[0]:
      return 'icon-alarm'
    case ReadStatus[1]:
      return 'icon-checkmark'
    case ReadStatus[2]:
      return 'icon-heart'
    default:
      return 'blank'
  }
}

// Placeholder for building book URL
function buildBookUrl(title, author, isbn) {
  // Replace with your actual logic
  return `/book/${encodeURIComponent(title)}/${encodeURIComponent(author || '')}/${isbn}`
}

function onScanBtnClickHandler() {
  // Replace with your actual navigation logic
  // For example, using vue-router:
  // router.push({ name: 'scan' })
  alert('Navigate to scan page')
}

function onBookLinkClickHandler(book) {
  // Replace with your actual navigation logic
  // For example, using vue-router:
  // router.push({ path: buildBookUrl(book.title, book.authors?.[0], book.isbn) })
  alert(`Navigate to: ${buildBookUrl(book.title, book.authors?.[0], book.isbn)}`)
}

function handleWasmMessage(event) {
  let data
  try {
    data = JSON.parse(event.data)
  } catch (e) {
    return
  }
  if (data?.localBooks?.Ok?.books) {
    books.value = data.localBooks.Ok.books
  }
}

onMounted(() => {
  window.addEventListener('message', handleWasmMessage)
  document.title = "📖📚📚"

  // Placeholder for async logic to fetch books and token
  // Replace with your actual logic
  // await initWasmModule()
  // get_scanned_books(token.value, withCloudSync)
  // if (token.value) withCloudSync = false
})

onBeforeUnmount(() => {
  window.removeEventListener('message', handleWasmMessage)
})



// const { token, question } = storeToRefs(store);

// /// redirect to subscription page if the user is authenticated
// watchEffect(() => {
//   // this redirect has to be here to redirect from homepage only
//   // any other page should not redirect to sub automatically
//   if (token.value) {
//     console.log("Token obtained - redirecting to subscription page");
//     router.replace({ name: PageIDs.SUBSCRIPTION }); // cleaner history with replace
//   }
// });

</script>
