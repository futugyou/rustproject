<script setup lang="ts">
import { ref } from "vue";
import { getClient, Body, ResponseType, HttpOptions, HttpVerb } from '@tauri-apps/api/http';
import tool from "../tool";

const greetMsg = ref("");
const httpMethod = ref("GET")
const httpBody = ref("")

const foo = ref("")
const bar = ref("")
const multipart = ref(true)

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

async function doPost() {
    const client = await getClient().catch((e) => {
        tool.onMessage(e)
        throw e
    })

    const result = await client.request({
        url: 'http://localhost:3003',
        method: 'POST',
        body: Body.form({
            foo: foo.value,
            bar: bar.value
        }),
        headers: multipart
            ? { 'Content-Type': 'multipart/form-data' }
            : undefined,
        responseType: ResponseType.Text
    })
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

    <div class="card">
        <h3>HTTP Form</h3>
        <div class="flex gap-2 children:grow">
            <input class="input" v-model="foo" />
            <input class="input" v-model="bar" />
        </div>
        <br />
        <label>
            <input type="checkbox" v-model="multipart" />
            Multipart
        </label>
        <br />
        <br />
        <button class="btn" type="button" @click="doPost()"> Post it</button>
    </div>
</template>
