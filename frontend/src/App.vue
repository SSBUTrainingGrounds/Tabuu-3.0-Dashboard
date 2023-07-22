<template>
    <SidebarComponent v-if="sidebarVisible" @change-visibility="changeSidebarVisibility" />

    <HeaderComponent
        :class="sidebarVisible ? 'shifted' : ''"
        :discordToken="discordToken"
        :user="loggedInUser"
        :guilds="guilds"
        @logOut="logOut"
        @force-change-visibility="changeSidebarVisibility(true)"
    />

    <div class="view" :class="sidebarVisible ? 'shifted' : ''">
        <RouterView :userID="loggedInUser.id" :users="allGuildUsers" />
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, ref, type Ref } from "vue";
import { RouterView } from "vue-router";

import HeaderComponent from "./components/HeaderComponent.vue";
import SidebarComponent from "./components/SidebarComponent.vue";

import type { GuildUser, LoggedInUser } from "./helpers/types";

const sidebarVisible = ref(window.innerWidth > 900 ? true : false);

function changeSidebarVisibility(force: boolean = false) {
    if (force) {
        sidebarVisible.value = !sidebarVisible.value;
    } else {
        if (window.innerWidth > 900) {
            sidebarVisible.value = true;
        } else {
            sidebarVisible.value = !sidebarVisible.value;
        }
    }
}

onBeforeMount(async () => {
    const fragment = new URLSearchParams(window.location.hash.slice(1));

    const token = fragment.get("access_token");

    if (token) {
        logIn(token);
    }

    if (discordToken.value) {
        fetch("https://discord.com/api/users/@me", {
            headers: {
                Authorization: `Bearer ${discordToken.value}`
            }
        })
            .then((res) => res.json())
            .then((data) => {
                // The message property is only present if there is an error, like an invalid token.
                if (data.message) {
                    discordToken.value = "";
                    console.log(data.message);
                } else {
                    loggedInUser.value = data;
                    logIn(discordToken.value);
                }
            });
    }

    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/users";

    await fetch(url, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${discordToken.value}`
        }
    })
        .then((res) => res.json())
        .then((data) => {
            data.forEach((user: any) => {
                allGuildUsers.value.set(user.user.id, {
                    name: user.user.username,
                    avatar: user.user.avatar
                });
            });
        });
});

let discordToken = ref(localStorage.getItem("discordToken") || "");

let loggedInUser: Ref<LoggedInUser> = ref({
    id: "",
    username: "",
    discriminator: "",
    avatar: "",
    bot: false,
    system: false,
    mfa_enabled: false,
    locale: "",
    verified: false,
    email: "",
    flags: 0,
    premium_type: 0,
    public_flags: 0
});

let guilds: Ref<Object[]> = ref([]);

let allGuildUsers: Ref<Map<string, GuildUser>> = ref(new Map());

function logIn(token: string) {
    discordToken.value = token;
    localStorage.setItem("discordToken", discordToken.value);

    // Remove the token from the URL.
    window.location.hash = "";
}

function logOut() {
    discordToken.value = "";
    localStorage.removeItem("discordToken");
}
</script>
