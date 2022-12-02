<script setup lang="ts">
import Navigation from "./components/Navigation.vue";
</script>

<template>
  <div class="page">
    <Navigation></Navigation>
    <div class="container">
      <router-view />
    </div>
  </div>
</template>