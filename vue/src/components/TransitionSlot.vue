<template>
  <Transition mode="out-in" name="fade" enter-active-class="duration-800 ease-out" enter-from-class="transform opacity-0" enter-to-class="opacity-100" leave-active-class="duration-800 ease-in" leave-from-class="opacity-100" leave-to-class="transform opacity-0">
    <slot></slot>
  </Transition>
</template>

<script setup lang="ts">
import { Transition } from "vue";
</script>