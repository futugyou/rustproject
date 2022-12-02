<script setup lang="ts">

import tool from "../tool";

// send the notification directly
// the backend is responsible for checking the permission
function _sendNotification() {
    new Notification('Notification title', {
        body: 'This is the notification body'
    })
}

// alternatively, check the permission ourselves
function sendNotification() {
    if (Notification.permission === 'default') {
        Notification.requestPermission()
            .then(function (response) {
                if (response === 'granted') {
                    _sendNotification()
                } else {
                    tool.onMessage('Permission is ' + response)
                }
            })
            .catch(tool.onMessage)
    } else if (Notification.permission === 'granted') {
        _sendNotification()
    } else {
        tool.onMessage('Permission is denied')
    }
}
</script>

<template>
    <div class="card">
        <button class="btn" id="notification" @click="_sendNotification()">
            Send test notification
        </button>
    </div>
</template>
