<template>
    <div v-if="!loggedIn" class="wrapper">
        <a
            href="https://discord.com/api/oauth2/authorize?client_id=775675159402905642&redirect_uri=http%3A%2F%2Flocalhost%3A5173%2Fdashboard&response_type=token&scope=identify"
            class="log-button"
            ><i class="fab fa-discord"></i> Login With Discord
        </a>

        <h2>Please login with Discord to use the dashboard</h2>
    </div>

    <div v-else class="wrapper">
        <a href="http://localhost:5173/dashboard" class="log-button"><i class="fab fa-discord"></i> Log Out</a>

        <div class="user-display">
            <img
                :src="`https://cdn.discordapp.com/avatars/${user.id}/${user.avatar}.png`"
                class="avatar"
                alt="Avatar"
            />
            <div class="user-name">
                <h2>Welcome @{{ user.username }}!</h2>
                <h3 class>({{ user.id }})</h3>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from "vue";

let loggedIn = ref(false);

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

onBeforeMount(() => {
    const fragment = new URLSearchParams(window.location.hash.slice(1));

    const token = fragment.get("access_token");

    if (token) {
        fetch("https://discord.com/api/users/@me", {
            headers: {
                Authorization: `Bearer ${token}`
            }
        })
            .then((res) => res.json())
            .then((data) => {
                // The message property is only present if there is an error, like an invalid token.
                if (data.message) {
                    loggedIn.value = false;
                    console.log(data.message);
                } else {
                    user.value = data;
                    loggedIn.value = true;
                }
            });
    } else {
        loggedIn.value = false;
    }
});
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
