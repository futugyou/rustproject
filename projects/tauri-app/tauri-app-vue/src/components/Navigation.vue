<script setup lang="ts">

interface Menu {
    lable: string,
    name: string,
}

const MenuConfigs: Menu[] = [
    {
        lable: "Cli",
        name: "Cli"
    },
    {
        lable: "Clipboard",
        name: "Clipboard"
    },
    {
        lable: "Dialog",
        name: "Dialog"
    },
    {
        lable: "Greet",
        name: "Greet"
    },
    {
        lable: "Http",
        name: "Http"
    },
    {
        lable: "Show",
        name: "Show"
    },
    {
        lable: "TauriEvent",
        name: "TauriEvent"
    },
    {
        lable: "Notification",
        name: "Notification"
    }]
</script>

<template>
    <div class="container-fluid">
        <ul class="navbar-nav pt-lg-3">
            <template v-for="menu in MenuConfigs">
                <li class="nav-item">
                    <router-link class="nav-link" :to="{ name: menu.name }">
                        <span class="nav-link-title">
                            {{ menu.lable }}
                        </span>
                    </router-link>
                </li>
            </template>
        </ul>
    </div>
</template>