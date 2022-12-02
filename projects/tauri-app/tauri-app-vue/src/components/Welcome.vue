<script setup lang="ts">
import { ref } from "vue";
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app';
import { relaunch, exit } from '@tauri-apps/api/process';
import tool from "../tool";

const version = ref("0.0.0");
const tauriVersion = ref("0.0.0");
const appName = ref("Unknown");

getName().then((n) => {
    appName.value = n
})

getVersion().then((v) => {
    version.value = v
})

getTauriVersion().then((v) => {
    tauriVersion.value = v
})

async function closeApp() {
    await exit()
}

async function relaunchApp() {
    await relaunch()
}
</script>

<template>
    <div class="card">

        <p>
            This is a demo of Tauri's API capabilities using the <code>@tauri-apps/api</code> package. It's used as the
            main validation app, serving as the test bed of our
            development process. In the future, this app will be used on Tauri's integration
            tests.
        </p>

        <br />
        <br />
        <pre>
            App name: <code>{{ appName }}</code>
            App version: <code>{{ version }}</code>
            Tauri version: <code>{{ tauriVersion }}</code>
        </pre>
        <br />
        <div class="flex flex-wrap gap-1 shadow-">
            <button class="btn" @click="closeApp()">Close application</button>
            <button class="btn" @click="relaunchApp()">Relaunch application</button>
        </div>

    </div>
</template>
