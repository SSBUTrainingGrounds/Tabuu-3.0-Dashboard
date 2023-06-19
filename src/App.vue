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
