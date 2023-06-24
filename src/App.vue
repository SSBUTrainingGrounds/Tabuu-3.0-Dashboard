<template>
    <HeaderComponent :discordToken="discordToken" :user="user" :guilds="guilds" @logOut="logOut" />

    <div class="view"><RouterView :isAdmin="isAdmin" :user="user" /></div>

    <FooterComponent />
</template>

<script setup lang="ts">
import { onBeforeMount, ref, type Ref } from "vue";
import { RouterView } from "vue-router";

import HeaderComponent from "./components/HeaderComponent.vue";
import FooterComponent from "./components/FooterComponent.vue";

onBeforeMount(() => {
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
                    user.value = data;
                    logIn(discordToken.value);
                }
            });

        // Find out if the user is an admin on the server.
        fetch("https://discord.com/api/users/@me/guilds", {
            headers: {
                Authorization: `Bearer ${discordToken.value}`
            }
        })
            .then((res) => res.json())
            .then((data) => {
                if (data.message) {
                    console.log(data.message);
                } else {
                    guilds.value = data;

                    for (let i = 0; i < guilds.value.length; i++) {
                        // @ts-ignore
                        if (guilds.value[i].id === "739299507795132486" && guilds.value[i].permissions === 2147483647) {
                            isAdmin.value = true;
                            return;
                        }
                    }

                    isAdmin.value = false;
                }
            });
    }
});

let discordToken = ref(localStorage.getItem("discordToken") || "");
let isAdmin = ref(false);

let user = ref({
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

<style>
@import "./assets/styles.css";

.view {
    padding-top: 4rem;
    padding-bottom: 4rem;
}

@media (max-width: 600px) {
    .view {
        padding-top: 20rem;
    }
}

.grid {
    overflow: hidden hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;

    display: grid;
    grid-gap: 0.2rem;
    max-width: 1200px;
    min-width: 600px;
    justify-content: center;
    margin: 0 auto;
}

.grid::-webkit-scrollbar {
    display: none;
}

.avatar-preview {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 50%;
    margin-right: 0.5rem;
}

.table-header {
    font-weight: bold;
}

.user,
.table-header,
.command {
    display: grid;
    grid-gap: 1rem;
    padding: 1rem;
    border-radius: 0.5rem;
}

.user:nth-child(odd),
.command:nth-child(odd) {
    background-color: var(--black);
}

.user:nth-child(even),
.command:nth-child(even) {
    background-color: var(--dark-gray);
}

.user:hover,
.command:hover {
    background-color: var(--green);
}
</style>
