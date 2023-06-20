<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
    <header>
        <div class="wrapper">
            <nav>
                <RouterLink to="/">Home</RouterLink>
                <RouterLink to="/trueskill">TrueSkill</RouterLink>
                <RouterLink to="/leaderboard">Leaderboard</RouterLink>
                <RouterLink to="/commands">Command Stats</RouterLink>
                <RouterLink to="/profiles">Profiles</RouterLink>
            </nav>
        </div>
    </header>

    <RouterView />
</template>

<style scoped>
@import "./assets/styles.css";

header {
    background-color: var(--black);
    color: var(--white);
    padding: 1rem;
}

.wrapper {
    max-width: 1200px;
    margin: 0 auto;
}

nav {
    display: flex;
    justify-content: space-between;
}

nav a {
    color: var(--green);
    text-decoration: none;
    font-weight: bold;
}

nav a:hover {
    color: var(--light-green);
}

nav a.active {
    color: var(--light-green);
}
</style>
