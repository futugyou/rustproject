<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { show, hide } from '@tauri-apps/api/app';

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

function showApp() {
  hideApp()
    .then(() => {
      setTimeout(() => {
        show()
          .then(() => greetMsg.value = 'Shown app')
      }, 2000)
    })
}

function hideApp() {
  return hide()
    .then(() => greetMsg.value = 'Hide app')
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>


  <div class="card">
    <button class="btn" id="show" title="Hides and shows the app after 2 seconds" @click="showApp()">Show</button>
    <button class="btn" id="hide" @click="hideApp()">Hide</button>
  </div>

</template>
