<template>
    <div>
        <nav class="nav">
            <RouterLink class="green-link" to="/ranked/leaderboard"><i class="fa fa-home"></i> Leaderboard</RouterLink>
            <RouterLink class="green-link" to="/ranked/matches"><i class="fa fa-list"></i> Matches</RouterLink>
        </nav>
    </div>
</template>

<script setup lang="ts"></script>

<style scoped>
@import "../assets/styles.css";

.nav {
    display: flex;
    justify-content: space-evenly;
    background-color: var(--black);
    color: var(--white);
    bottom: 0;
    padding-top: 1rem;
    padding-bottom: 1rem;
    border-radius: 1rem;
}

.nav a {
    color: var(--green);
    text-decoration: none;
    font-weight: bold;
    font-size: large;
    margin: 0 0.2rem 0 0.2rem;
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

@media (max-width: 600px) {
    .footer .nav a {
        font-size: small;
    }
}
</style>
