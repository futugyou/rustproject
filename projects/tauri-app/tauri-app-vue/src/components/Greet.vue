<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import tool from "../tool";

const greetMsg = ref("");
const name = ref("");

document.addEventListener('DOMContentLoaded', () => {
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
  invoke('close_splashscreen')
})

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

function log() {
  invoke('log_operation', {
    event: 'tauri-click',
    payload: 'this payload is optional because we used Option in Rust'
  })
}

function performRequest() {
  invoke('perform_request', {
    endpoint: 'dummy endpoint arg',
    body: {
      id: 5,
      name: 'test'
    }
  })
    .then(tool.onMessage)
    .catch(tool.onMessage)
}
</script>

<template>
  <div class="card">
    <button class="btn" id="log" @click="log()">Call Log API</button>
    <button class="btn" id="request" @click="performRequest()"> Call Request (async) API </button>
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
    <p>{{ greetMsg }}</p>
  </div>

</template>
