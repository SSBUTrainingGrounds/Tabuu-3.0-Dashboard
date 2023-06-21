<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
    <header>
        <div class="wrapper">
            <nav class="header-nav">
                <RouterLink to="/"><i class="fa fa-home"></i>Home</RouterLink>
                <RouterLink to="/trueskill"><i class="fa fa-check"></i>TrueSkill</RouterLink>
                <RouterLink to="/leaderboard"><i class="fa fa-envelope"></i>Leaderboard</RouterLink>
                <RouterLink to="/commands"><i class="fa fa-signal"></i>Command Stats</RouterLink>
                <RouterLink to="/profiles"><i class="fa fa-user"></i>Profiles</RouterLink>
            </nav>
        </div>
    </header>

    <RouterView />
</template>

<style>
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

.header-nav {
    display: flex;
    justify-content: space-between;
}

.header-nav a {
    color: var(--green);
    text-decoration: none;
    font-weight: bold;
    font-size: large;
}

.header-nav a:hover {
    color: var(--light-green);
}

.header-nav a.active {
    color: var(--light-green);
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
