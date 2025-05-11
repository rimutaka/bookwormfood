<template>
  <!-- For .p-button- styles to work the page must contain at least one proper Button from 'primevue/button' -->
  <!-- These two types of links are identical except that one goes via the router.
   Keep their markup identical - changes to one should be copied to the other -->
   <a v-if="isExternalUrl" :href="href" class="p-button p-component whitespace-nowrap p-button-sm" :class="{ 'p-button-secondary': !props.primary }" :aria-label="label">
    <span v-if="icon" class="p-button-icon p-button-icon-left" :class="icon"></span>
    <span class="p-button-label">{{ label }}</span>
  </a>

   <router-link v-else :to="href" class="p-button p-component whitespace-nowrap p-button-sm" :class="{ 'p-button-secondary': !props.primary }" :aria-label="label">
    <span v-if="icon" class="p-button-icon p-button-icon-left" :class="icon"></span>
    <span class="p-button-label">{{ label }}</span>
  </router-link>

</template>


<script setup lang="ts">

const props = defineProps<{
  href: string,
  label: string,
  icon?: string,
  primary?: boolean,
}>()

// true if the link should go via A tag because it is an external link
const isExternalUrl = props.href.startsWith('http');

</script>
