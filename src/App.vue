<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
    <header>
        <div class="wrapper-header">
            <nav class="nav">
                <RouterLink to="/"><i class="fa fa-home"></i> Home</RouterLink>
                <RouterLink to="/trueskill"><i class="fa fa-check"></i> TrueSkill</RouterLink>
                <RouterLink to="/leaderboard"><i class="fa fa-envelope"></i> Leaderboard</RouterLink>
                <RouterLink to="/commands"><i class="fa fa-signal"></i> Command Stats</RouterLink>
                <RouterLink to="/profiles"><i class="fa fa-user"></i> Profiles</RouterLink>
            </nav>
        </div>
    </header>

    <RouterView />

    <footer>
        <div class="wrapper-footer">
            <nav class="nav">
                <a href="https://github.com/SSBUTrainingGrounds/Tabuu-3.0"><i class="fa fa-github"></i> Bot GitHub</a>
                <a href="https://github.com/atomflunder/bot-frontend"><i class="fa fa-github"></i> Dashboard GitHub</a>
            </nav>
        </div>
    </footer>
</template>

<style>
@import "./assets/styles.css";

header {
    background-color: var(--black);
    color: var(--white);
    padding: 1rem;
}

footer {
    background-color: var(--black);
    color: var(--white);
    bottom: 0;
    width: 100%;
    position: absolute;
    padding-top: 1rem;
    padding-bottom: 1rem;
}

header,
footer,
.grid {
    overflow: hidden hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;
}

header::-webkit-scrollbar,
footer::-webkit-scrollbar,
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

@media (max-width: 500px) {
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
