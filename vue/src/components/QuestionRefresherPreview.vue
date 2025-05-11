<template>
  <div>
    <h3 class="text-start">Refresher material</h3>
    <div class="q-card">
      <ul>
        <li v-for="link in links" :key="link">
          <a :href="link" target="_blank">{{ link }}</a>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">

const props = defineProps<{
  links?: string[], // a list of URLs to display
}>()

</script>
