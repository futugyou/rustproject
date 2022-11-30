<script setup lang="ts">
import { ref } from "vue";
import { show, hide } from '@tauri-apps/api/app';
import tool from "../tool";

const greetMsg = ref("");

function showApp() {
    hideApp()
        .then(() => {
            setTimeout(() => {
                show()
                    .then(() => greetMsg.value = 'Shown app')
                    .catch(tool.onMessage)
            }, 2000)
        })
        .catch(tool.onMessage)
}

function hideApp() {
    return hide()
        .then(() => greetMsg.value = 'Hide app')
        .catch(tool.onMessage)
}
</script>

<template>
    <div class="card">
        <button class="btn" id="show" title="Hides and shows the app after 2 seconds" @click="showApp()">Show</button>
        <button class="btn" id="hide" @click="hideApp()">Hide</button>
        <p>{{ greetMsg }}</p>
    </div>
</template>
