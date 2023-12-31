<template>
    <header class="header">
        <button class="navigation-button" @click="$emit('forceChangeVisibility')">
            <i class="fa fa-bars"></i>
            Navigation
        </button>

        <div class="login">
            <a :href="url" class="long-login-button" v-if="!discordToken"
                ><i class="fab fa-discord"></i> Log In with Discord
            </a>

            <a :href="url" class="short-login-button" v-if="!discordToken"><i class="fab fa-discord"></i> Log In </a>

            <a href="/" class="logout-button" v-if="discordToken" @click="$emit('logOut')"
                ><i class="fab fa-discord"></i> Log Out</a
            >

            <div class="user-display" v-if="discordToken">
                <img
                    v-if="user.id && user.avatar"
                    :src="`https://cdn.discordapp.com/avatars/${user.id}/${user.avatar}.png`"
                    class="avatar"
                    alt="Avatar"
                />
                <div class="user-login-info">Logged in as @{{ user.username }}!</div>
                <div class="user-login-id">({{ user.id }})</div>
            </div>
        </div>
    </header>
</template>

<script setup lang="ts">
import type { LoggedInUser } from "../helpers/types";

defineProps({
    discordToken: String,
    user: {
        type: Object as () => LoggedInUser,
        required: true
    }
});

defineEmits<{
    logOut: [];
    forceChangeVisibility: [];
}>();

const url = import.meta.env.VITE_DISCORD_LOGIN_URL;
</script>

<style scoped>
.header {
    background-color: var(--black);
    color: var(--white);
    padding: 1rem;
    position: fixed;
    width: 100%;
    min-width: 300px;
    height: 2rem;
    left: 0;
    right: 0;
    display: grid;
    grid-template-columns: 1fr 1.5fr;
}
.login {
    display: flex;
    align-items: center;
    justify-self: right;
    margin-right: 4rem;
}

.navigation-button {
    background-color: var(--black);
    color: var(--green);
    border: none;
    font-size: large;
    font-weight: bold;
    cursor: pointer;
    justify-self: left;
    margin-left: 1rem;
}

.navigation-button:hover {
    color: var(--light-green);
}

.user-display {
    display: grid;
    grid-template-columns: 1.5rem 1fr;
    font-weight: bold;
}

.avatar {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 50%;
    grid-row: 1 / 3;
}

.user-login-info {
    grid-column: 2;
    grid-row: 1;
    font-size: x-small;
    margin-left: 0.5rem;
}

.user-login-id {
    grid-column: 2;
    grid-row: 2;
    font-size: xx-small;
    margin-left: 0.5rem;
}

.short-login-button {
    display: none;
}

.long-login-button,
.short-login-button,
.logout-button {
    text-decoration: none;
    font-weight: bold;
    font-size: large;
    margin: 0 0.5rem 0 0.5rem;
}

.long-login-button,
.short-login-button {
    color: var(--discord-dark);
}

.long-login-button:hover,
.short-login-button:hover {
    color: var(--discord-blue);
}

.logout-button {
    color: var(--red);
}

.logout-button:hover {
    color: var(--light-red);
}

@media (max-width: 600px) {
    .user-display {
        display: none;
    }

    .header {
        grid-template-columns: 1fr 1fr;
    }

    .long-login-button {
        display: none;
    }

    .short-login-button {
        display: block;
    }
}

@media (max-width: 400px) {
    .long-login-button,
    .short-login-button,
    .logout-button,
    .navigation-button {
        font-size: medium;
    }

    .header {
        padding: 0.5rem;
    }
}

@media (max-width: 320px) {
    .long-login-button,
    .short-login-button,
    .logout-button,
    .navigation-button {
        font-size: small;
    }
}
</style>
