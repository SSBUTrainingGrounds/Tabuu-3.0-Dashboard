<template>
    <div v-if="!discordToken" class="wrapper">
        <a
            href="https://discord.com/api/oauth2/authorize?client_id=775675159402905642&redirect_uri=http%3A%2F%2Flocalhost%3A5173%2Fdashboard&response_type=token&scope=identify%20guilds%20guilds.members.read"
            class="log-button"
            ><i class="fab fa-discord"></i> Login With Discord
        </a>
    </div>

    <div v-else class="wrapper">
        <a href="http://localhost:5173/macro" class="log-button" @click="logOut"
            ><i class="fab fa-discord"></i> Log Out</a
        >

        <div class="user-display">
            <img
                v-if="user.id && user.avatar"
                :src="`https://cdn.discordapp.com/avatars/${user.id}/${user.avatar}.png`"
                class="avatar"
                alt="Avatar"
            />
            <div class="user-name">
                <h2>Welcome @{{ user.username }}!</h2>
                <h3 class>({{ user.id }})</h3>
            </div>
        </div>

        <div v-if="isAdmin">
            <MacroComponent @sendMacro="sendMacro" @deleteMacro="deleteMacro" />
        </div>

        <div v-else>
            <h2>You are not authorized to view this page</h2>
            <p>If you believe this is a mistake, please contact the server administrators.</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import MacroComponent from "@/components/MacroComponent.vue";

import { onBeforeMount, ref, type Ref } from "vue";

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
}

function logOut() {
    localStorage.removeItem("discordToken");
    discordToken.value = "";
}

onBeforeMount(() => {
    const fragment = new URLSearchParams(window.location.hash.slice(1));

    const token = fragment.get("access_token");

    if (token) {
        logIn(token);
        // Remove the token from the URL.
        window.location.hash = "";
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
                    logOut();
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

function sendMacro(name: string, payload: string) {
    fetch("http://localhost:8080/macro_new", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name,
            macro: payload,
            uses: 0,
            author: user.value.id
        })
    });
}

function deleteMacro(name: string) {
    fetch("http://localhost:8080/macro_delete", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name
        })
    });
}
</script>

<style scoped>
@import "../assets/styles.css";

.wrapper {
    text-align: center;
}

.log-button {
    background-color: var(--discord-blue);
    color: var(--white);
    padding: 0.8rem 1rem;
    margin: 1rem auto;
    border-radius: 5px;
    text-decoration: none;
    font-size: 1.2rem;
    font-weight: bold;
    display: inline-block;
    transition: background-color 0.2s ease-in-out;
}

.log-button:hover {
    background-color: var(--discord-dark);
}

.user-display {
    color: var(--white);
    text-align: center;
}

.user-name,
.avatar {
    display: inline-block;
    margin: 0 auto;
    vertical-align: top;
    padding: 1rem;
}

.avatar {
    height: 150px;
    width: 150px;
    border-radius: 50%;
}
</style>
