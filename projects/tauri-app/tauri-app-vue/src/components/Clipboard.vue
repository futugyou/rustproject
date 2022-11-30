<script setup lang="ts">
import { ref } from "vue";
import { writeText, readText } from '@tauri-apps/api/clipboard';
import tool from "../tool";

const greetMsg = ref("");
const text = ref("");

function write() {
    writeText(text.value)
      .then(() => {
        tool.onMessage('Wrote to the clipboard')
      })
      .catch(tool.onMessage)
  }

  function read() {
    readText()
      .then((contents) => {
        tool.onMessage(`Clipboard contents: ${contents}`)
        greetMsg.value = contents??""
      })
      .catch(tool.onMessage)
  }
</script>

<template>
    <div class="card">
        <input id="text" v-model="text" placeholder="Enter a text..." />
        <button class="btn" id="write" @click="write()">write</button>
        <button class="btn" id="read" @click="read()">read</button>
        <p>{{ greetMsg }}</p>
    </div>
</template>
