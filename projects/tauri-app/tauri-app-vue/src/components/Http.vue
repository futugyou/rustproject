<script setup lang="ts">
import { ref } from "vue";
import { getClient, Body, HttpOptions, HttpVerb } from '@tauri-apps/api/http';
import tool from "../tool";

const greetMsg = ref("");
const httpMethod = ref("GET")
const httpBody = ref("")

async function makeHttpRequest() {
    const client = await getClient().catch((e) => {
        tool.onMessage(e)
        throw e
    })
    let method = httpMethod.value || 'GET'

    const options: HttpOptions = {
        url: 'http://localhost:3003',
        method: (method || 'GET') as HttpVerb,
        body: Body.json(JSON.parse('{}'))
    }

    if (
        (httpBody.value.startsWith('{') && httpBody.value.endsWith('}')) ||
        (httpBody.value.startsWith('[') && httpBody.value.endsWith(']'))
    ) {
        options.body = Body.json(JSON.parse(httpBody.value))
    } else if (httpBody.value !== '') {
        options.body = Body.text(httpBody.value)
    }
    tool.onMessage(httpBody.value)
    client.request(options).then(tool.onMessage).catch(tool.onMessage)
}
</script>

<template>
    <div class="card">
        <form @submit.prevent="makeHttpRequest">
            <select class="input" id="request-method" v-model="httpMethod">
                <option value="GET">GET</option>
                <option value="POST">POST</option>
                <option value="PUT">PUT</option>
                <option value="PATCH">PATCH</option>
                <option value="DELETE">DELETE</option>
            </select>
            <br />
            <textarea class="input h-auto w-100%" id="request-body" placeholder="Request body" rows="5"
                v-model="httpBody" />
            <br />
            <button class="btn" id="make-request"> Make request </button>
        </form>
    </div>
</template>
