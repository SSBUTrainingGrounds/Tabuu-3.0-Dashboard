<template>
    <header class="header">
        <div class="wrapper-header">
            <nav class="nav">
                <RouterLink to="/"><i class="fa fa-home"></i> Home</RouterLink>
                <RouterLink to="/trueskill"><i class="fa fa-check"></i> TrueSkill</RouterLink>
                <RouterLink to="/leaderboard"><i class="fa fa-envelope"></i> Leaderboard</RouterLink>
                <RouterLink to="/commands"><i class="fa fa-signal"></i> Command Stats</RouterLink>
                <RouterLink to="/profiles"><i class="fa fa-user"></i> Profiles</RouterLink>
                <RouterLink to="/macro"><i class="fa fa-cog"></i> Macros</RouterLink>

                <a
                    href="https://discord.com/api/oauth2/authorize?client_id=775675159402905642&redirect_uri=http%3A%2F%2Flocalhost%3A5173%2F&response_type=token&scope=guilds%20identify%20guilds.members.read"
                    class="log-button"
                    v-if="!discordToken"
                    ><i class="fab fa-discord"></i> Login With Discord
                </a>

                <a href="http://localhost:5173/" class="log-button" v-if="discordToken" @click="logOut"
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

    <div class="view"><RouterView :isAdmin="isAdmin" :user="user" /></div>

    <footer class="footer">
        <div class="wrapper-footer">
            <nav class="nav">
                <a href="https://github.com/SSBUTrainingGrounds/Tabuu-3.0"><i class="fab fa-github"></i> Bot GitHub</a>
                <a href="https://discord.gg/ssbutg"><i class="fab fa-discord"></i> Join Our Discord</a>
                <a href="https://github.com/atomflunder/dashboard-frontend"
                    ><i class="fab fa-github"></i> Dashboard GitHub</a
                >
            </nav>
        </div>
    </footer>
</template>

<script setup lang="ts">
import { onBeforeMount, ref, type Ref } from "vue";
import { RouterLink, RouterView } from "vue-router";

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

.header {
    background-color: var(--black);
    color: var(--white);
    padding: 1rem;
}

.footer {
    background-color: var(--black);
    color: var(--white);
    bottom: 0;
    padding-top: 1rem;
    padding-bottom: 1rem;
}

.view {
    padding-top: 4rem;
    padding-bottom: 4rem;
}

.header,
.footer {
    position: fixed;
    width: 100%;
    left: 0;
    right: 0;
}

.header,
.footer,
.grid {
    overflow: hidden hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;
}

.header::-webkit-scrollbar,
.footer::-webkit-scrollbar,
.grid::-webkit-scrollbar {
    display: none;
}

.wrapper-header {
    max-width: 1200px;
    margin: 0 auto;
}

.wrapper-footer {
    max-width: 800px;
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

.nav {
    display: flex;
    justify-content: space-between;
}

.nav a {
    color: var(--green);
    text-decoration: none;
    font-weight: bold;
    font-size: large;
    margin: 0 0.2rem 0 0.2rem;
}

.nav a:hover {
    color: var(--light-green);
}

.nav a.active {
    color: var(--light-green);
}

@media (max-width: 600px) {
    .nav a {
        font-size: small;
    }

    .grid {
        font-size: x-small;
    }
}

.grid {
    display: grid;
    grid-gap: 0.2rem;
    max-width: 1200px;
    justify-content: center;
    margin: 0 auto;
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
