<script setup lang="ts">
import { ref } from "vue";
import { listen, emit } from '@tauri-apps/api/event';
import tool from "../tool";

const greetMsg = ref("");

document.addEventListener('DOMContentLoaded', async () => {
    await listen('rust-event', tool.onMessage)
})

function emitEvent() {
    emit('js-event', 'this is the payload string')
}
</script>

<template>
    <div class="card">
        <button class="btn" id="event" @click="emitEvent()">
            Send event to Rust
        </button>
        <p>{{ greetMsg }}</p>
    </div>
</template>
