<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Show from "./components/Show.vue";
import Cli from "./components/Cli.vue";
import Clipboard from "./components/Clipboard.vue";
import TauriEvent from "./components/TauriEvent.vue";
import Dialog from "./components/Dialog.vue";
import Http from "./components/Http.vue";
</script>

<template>
  <h1>Welcome to Tauri!</h1>
  <div class="container">
    <Greet />
    <Show />
    <Cli />
    <Clipboard />
    <TauriEvent />
    <Dialog />
    <Http />
  </div>
</template>