<script setup lang="ts">
import { ref } from "vue";
import { open, save } from '@tauri-apps/api/dialog';
import { readBinaryFile } from '@tauri-apps/api/fs';
import tool from "../tool";

const defaultPath = ref("");
const filter = ref("");
const multiple = ref(true);
const directory = ref(true);

function arrayBufferToBase64(buffer: any, callback: any) {
    var blob = new Blob([buffer], {
        type: 'application/octet-binary'
    })
    var reader = new FileReader()
    reader.onload = function (evt) {
        var dataurl: String = evt!.target!.result as String
        callback(dataurl!.substr(dataurl!.indexOf(',') + 1))
    }
    reader.readAsDataURL(blob)
}

function openDialog() {
    open({
        title: 'My wonderful open dialog',
        defaultPath: defaultPath.value,
        filters: filter.value
            ? [
                {
                    name: 'Tauri Example',
                    extensions: filter.value.split(',').map((f) => f.trim())
                }
            ]
            : [],
        multiple: multiple.value,
        directory: directory.value
    })
        .then(function (res) {
            var pathToRead = ""
            if (Array.isArray(res)) {
                pathToRead = res[0]
            } else {
                pathToRead = res!
            }
            var isFile = pathToRead.match(/\S+\.\S+$/g)
            readBinaryFile(pathToRead)
                .then(function (response) {
                    if (isFile) {
                        if (
                            pathToRead.includes('.png') ||
                            pathToRead.includes('.jpg')
                        ) {
                            arrayBufferToBase64(
                                new Uint8Array(response),
                                function (base64: any) {
                                    var src = 'data:image/png;base64,' + base64
                                    tool.onMessage(src)
                                }
                            )
                        } else {
                            tool.onMessage(res)
                        }
                    } else {
                        tool.onMessage(res)
                    }
                })
                .catch(tool.onMessage)

        })
        .catch(tool.onMessage)
}

function saveDialog() {
    save({
        title: 'My wonderful save dialog',
        defaultPath: defaultPath.value,
        filters: filter.value
            ? [
                {
                    name: 'Tauri Example',
                    extensions: filter.value.split(',').map((f) => f.trim())
                }
            ]
            : []
    })
        .then(tool.onMessage)
        .catch(tool.onMessage)
}
</script>

<template>
    <div class="card">
        <div class="flex gap-2 children:grow">
            <input class="input" id="dialog-default-path" placeholder="Default path" v-model="defaultPath" />
            <input class="input" id="dialog-filter" placeholder="Extensions filter, comma-separated" v-model="filter" />
        </div>

        <br />
        <div>
            <input type="checkbox" id="dialog-multiple" v-model="multiple" />
            <label for="dialog-multiple">Multiple</label>
        </div>
        <div>
            <input type="checkbox" id="dialog-directory" v-model="directory" />
            <label for="dialog-directory">Directory</label>
        </div>
        <br />
        <button class="btn" id="open-dialog" @click="openDialog()">Open dialog</button>
        <button class="btn" id="save-dialog" @click="saveDialog()">Open save dialog</button>

        <p>{{ defaultPath }}</p>
        <p>{{ filter }}</p>
    </div>
</template>
