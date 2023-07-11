<template>
    <header class="header">
        <button class="navigation-button" @click="headerVisible = !headerVisible">
            <i class="fa fa-bars"></i>
            Navigation
        </button>

        <div class="wrapper-header" v-if="headerVisible">
            <nav class="nav" @click="changeHeaderVisibility">
                <RouterLink class="green-link" to="/"><i class="fa fa-home"></i> Home</RouterLink>
                <RouterLink class="green-link" to="/ranked/leaderboard"
                    ><i class="fa fa-check"></i> Ranked Matchmaking</RouterLink
                >
                <RouterLink class="green-link" to="/level"><i class="fa fa-envelope"></i> Level</RouterLink>
                <RouterLink class="green-link" to="/profiles"><i class="fa fa-user"></i> Profiles</RouterLink>
                <RouterLink class="green-link" to="/commands"><i class="fa fa-signal"></i> Command Stats</RouterLink>
                <RouterLink class="green-link" to="/macro"><i class="fa fa-cog"></i> Macros</RouterLink>
            </nav>
        </div>
        <div class="login">
            <a
                :href="
                    'https://discord.com/api/oauth2/authorize?client_id=785303736582012969&redirect_uri=http%3A%2F%2F' +
                    url +
                    '%3A' +
                    port +
                    '%2F&response_type=token&scope=guilds%20guilds.members.read%20identify'
                "
                class="login-button"
                v-if="!discordToken"
                ><i class="fab fa-discord"></i> Login with Discord
            </a>

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
import { ref } from "vue";

defineProps(["discordToken", "user", "guilds"]);

defineEmits(["logOut"]);

const url = window.location.hostname;
const port = "5173";

// We want the nav bar to be visible by default on desktop, but hidden on mobile.
// You can still hide it, if you want to.
const headerVisible = ref(window.innerWidth > 900 ? true : false);

const changeHeaderVisibility = () => {
    if (window.innerWidth > 900) {
        headerVisible.value = true;
    } else {
        headerVisible.value = false;
    }
};
</script>

<style scoped>
.header {
    background-color: var(--black);
    color: var(--white);
    padding: 1rem;
    position: fixed;
    width: 100%;
    left: 0;
    right: 0;
    overflow: hidden hidden;
    display: grid;
    grid-template-columns: 1fr 3fr 1.5fr;
    justify-items: space-around;
    -ms-overflow-style: none;
    scrollbar-width: none;
}

.header::-webkit-scrollbar {
    display: none;
}

.login {
    display: flex;
    align-items: center;
    grid-column: 3;
}

.wrapper-header {
    max-width: 1200px;
    margin: 0 auto;
    grid-column: 2;
    display: flex;
    justify-content: center;
    align-items: center;
}

.navigation-button {
    background-color: var(--black);
    color: var(--green);
    border: none;
    font-size: large;
    font-weight: bold;
    cursor: pointer;
    grid-column: 1;
}

.navigation-button:hover {
    color: var(--light-green);
}

.user-display {
    display: grid;
    position: relative;
    font-weight: bold;
}

.avatar {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 50%;
    margin-right: 0.5rem;
    grid-row: 1 / 3;
}

.user-login-info {
    grid-column: 2;
    grid-row: 1;
    font-size: x-small;
}

.user-login-id {
    grid-column: 2;
    grid-row: 2;
    font-size: xx-small;
}

.green-link,
.login-button,
.logout-button {
    text-decoration: none;
    font-weight: bold;
    font-size: large;
    margin: 0 0.5rem 0 0.5rem;
}

.login-button {
    color: var(--discord-dark);
}

.login-button:hover {
    color: var(--discord-blue);
}

.logout-button {
    color: var(--red);
}

.logout-button:hover {
    color: var(--light-red);
}

.nav {
    display: flex;
    justify-content: space-around;
}

.nav .green-link {
    color: var(--green);
}

.nav .green-link:hover {
    color: var(--light-green);
}

@media (max-width: 1300px) {
    .nav .green-link,
    .login-button,
    .logout-button,
    .navigation-button {
        font-size: small;
    }
}

@media (max-width: 900px) {
    .nav {
        flex-direction: column;
        align-items: center;
    }

    .user-display {
        display: none;
    }

    .nav .green-link,
    .login-button,
    .logout-button,
    .navigation-button {
        margin: 0.1rem 0 0.1rem 0;
        font-weight: bold;
        font-size: medium;
    }

    .header {
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 0.2fr 0.5fr;
        justify-items: center;
        padding: 0.5rem;
    }

    .navigation-button {
        grid-column: 1;
        grid-row: 1;
    }

    .wrapper-header {
        grid-column: 1 / 3;
        grid-row: 2;
        display: flex;
    }

    .login {
        grid-column: 2;
        grid-row: 1;
    }
}
</style>
