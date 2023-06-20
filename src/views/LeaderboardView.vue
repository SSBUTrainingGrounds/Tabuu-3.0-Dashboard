<template>
    <div class="grid">
        <div class="header">
            <div>Rank</div>
            <div>ID</div>
            <div>Level</div>
            <div>XP</div>
            <div>Messages</div>
        </div>
        <div class="user" v-for="(u, i) in user" :key="u">
            <div>#{{ i + 1 }}</div>
            <div>{{ u["id"] }}</div>
            <div>{{ u["level"] }}</div>
            <div>{{ u["xp"] }}</div>
            <div>{{ u["messages"] }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";

const user = ref({});

onMounted(async () => {
    const res = await fetch("http://localhost:8080/leaderboard");
    user.value = await res.json();
    // @ts-ignore
    user.value.sort((a, b) => b["xp"] - a["xp"]);
});
</script>

<style scoped>
@import "../assets/styles.css";
.grid {
    display: grid;
    grid-gap: 0.2rem;
    max-width: 1200px;
    justify-content: center;
    margin: 0 auto;
}

.user,
.header {
    display: grid;
    grid-template-columns: 1fr 1.5fr 1fr 1fr 1fr;
    grid-gap: 1rem;
    padding: 1rem;
    border-radius: 0.5rem;
}

.user:nth-child(odd) {
    background-color: var(--black);
}

.user:nth-child(even) {
    background-color: var(--dark-gray);
}

.user:hover {
    background-color: var(--green);
}
</style>
