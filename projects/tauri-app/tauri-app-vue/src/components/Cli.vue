<script setup lang="ts">
import { ref } from "vue";
import { getMatches } from '@tauri-apps/api/cli';
import tool from "../tool";

const greetMsg = ref("");

function cliMatches() {
    getMatches().then((r) => {
        greetMsg.value = r.subcommand ? r.subcommand.name : ""
    }).catch(tool.onMessage)
}
</script>

<template>
    <div class="card">
        <button class="btn" id="cli-matches" @click="cliMatches()">Get matches</button>
        <p>{{ greetMsg }}</p>
    </div>
</template>
