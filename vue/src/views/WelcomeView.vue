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
        <span v-if="!book.title" class=" text-sm">No title. ISBN: </span>
        <a :href="buildBookUrl(book)" @click.prevent="onBookLinkClickHandler(book)">
          {{ book.title || book.isbn }}
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
import { PageIDs } from '@/router'
import initWasmModule, { get_scanned_books, ReadStatus } from '../wasm-rust/isbn_mod.js';
import { buildBookUrl } from '@/interfaces.js';
import type { Book, ReadStatusStrings } from '@/interfaces.js';
import { useAuth0 } from '@auth0/auth0-vue';

const store = useMainStore();
const { isAuthenticated, loginWithRedirect, logout, getAccessTokenSilently } = useAuth0();
const { token } = storeToRefs(store);


// Placeholder for WASM and Auth0 logic
// Replace with your actual imports and logic
// import initWasmModule, { get_scanned_books, ReadStatus } from '../wasm-rust/isbn_mod.js';

const books = ref<Array<Book>>([])

// Books should be fetched from the cloud only once.
// The local storage is expected to be in sync while the app is active.
// A reload resets the flag.
// true: fetch books from the cloud, false: already fetched
let withCloudSync = true

function getStatusIcon(readStatus: ReadStatusStrings | undefined) {
  if (!readStatus) {
    return 'blank'
  }
  switch (readStatus) {
    case ReadStatus[ReadStatus.ToRead]:
      return 'icon-alarm'
    case ReadStatus[ReadStatus.Read]:
      return 'icon-checkmark'
    case ReadStatus[ReadStatus.Liked]:
      return 'icon-heart'
    default:
      return 'blank'
  }
}

function onScanBtnClickHandler() {
  // Replace with your actual navigation logic
  // For example, using vue-router:
  router.push({ name: PageIDs.SCAN });
}

function onBookLinkClickHandler(book: Book) {
  router.push({ path: buildBookUrl(book) })
}

const handleWasmMessage = (msg: MessageEvent) => {
  // console.log(`WASM msg: ${msg.data} / ${msg.origin} / ${msg.source}`);
  // WASM messages should be JSON objects
  let data;
  try {
    data = JSON.parse(msg.data);
  }
  catch (e) {
    // use this log for debugging, but this mostly logs messages sent from React tooling
    // in development mode, not sure it's worth logging this in production
    // console.log(`Error parsing JSON data: ${e}`);
    return;
  }

  // see `WasmResult` and `WasmResponse` in the WASM code for the structure of the data
  if (data?.localBooks?.Ok?.books) {
    books.value = data.localBooks.Ok?.books;
  }
  else {
    console.log("Welcome screen received a message that is not a list of books");
    console.log(data);
  }
};


onMounted(() => {
  // handles messages with book data sent back by the WASM module
  window.addEventListener('message', handleWasmMessage);

  // these values are used to set the meta tags in index.html
  // and have to be reset when the component is mounted from
  // a scan that sets them to the book details
  // make sure the values are synchronized with index.html
  // TODO: change ids to constants
  document.title = "📖📚📚";

  // get the list of books from the localStorage
  (async () => {

    await initWasmModule(); // run the wasm initializer before calling wasm methods
    // console.log("Requesting scanned books");
    // request book data from WASM module
    // the responses are sent back as messages to the window object 
    // console.log(`Read token: ${idTokenClaims?.__raw}, sync: ${withCloudSync}`);
    get_scanned_books(token.value, withCloudSync);
    // prevent future list syncs until the page is refreshed
    if (token.value) withCloudSync = false;
    // console.log("Requested scanned books (inside async)");
  })();

  // console.log("Requested scanned books (outside async)");

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
