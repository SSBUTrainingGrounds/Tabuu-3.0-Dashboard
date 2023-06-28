<template>
    <header class="header">
        <div class="wrapper-header">
            <nav class="nav">
                <RouterLink class="green-link" to="/"><i class="fa fa-home"></i> Home</RouterLink>
                <RouterLink class="green-link" to="/trueskill"><i class="fa fa-check"></i> TrueSkill</RouterLink>
                <RouterLink class="green-link" to="/leaderboard"><i class="fa fa-envelope"></i> Leaderboard</RouterLink>
                <RouterLink class="green-link" to="/profiles"><i class="fa fa-user"></i> Profiles</RouterLink>
                <RouterLink class="green-link" to="/commands"><i class="fa fa-signal"></i> Command Stats</RouterLink>
                <RouterLink class="green-link" to="/macro"><i class="fa fa-cog"></i> Macros</RouterLink>

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
                    ><i class="fab fa-discord"></i> Login With Discord
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
            </nav>
        </div>
    </header>
</template>

<script setup lang="ts">
defineProps(["discordToken", "user", "guilds"]);

defineEmits(["logOut"]);

const url = window.location.hostname;
const port = "5173";
</script>

<style scoped>
.header {
    background-color: var(--black);
    color: var(--white);
    padding: 1rem;
}

.header {
    position: fixed;
    width: 100%;
    left: 0;
    right: 0;
}

.header {
    overflow: hidden hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;
}

.header::-webkit-scrollbar {
    display: none;
}

.wrapper-header {
    max-width: 1200px;
    margin: 0 auto;
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
    margin: 0 0.2rem 0 0.2rem;
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
    justify-content: space-between;
}

.nav .green-link {
    color: var(--green);
}

.nav .green-link:hover {
    color: var(--light-green);
}

.nav .green-link.active {
    color: var(--light-green);
}

@media (max-width: 1000px) {
    .nav .green-link,
    .login-button,
    .logout-button {
        font-size: small;
    }
}

@media (max-width: 600px) {
    .nav {
        flex-direction: column;
        align-items: flex-start;
    }

    .nav .green-link,
    .login-button,
    .logout-button {
        margin: 0.2rem 0 0.2rem 0;
        font-size: large;
    }
}
</style>
