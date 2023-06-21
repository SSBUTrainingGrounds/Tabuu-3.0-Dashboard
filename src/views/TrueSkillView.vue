<template>
    <div class="grid">
        <div class="header">
            <div>Rank</div>
            <div>ID</div>
            <div>TabuuSkill</div>
            <div>Deviation</div>
            <div>Wins</div>
            <div>Losses</div>
            <div>Winrate</div>
        </div>
        <div class="user" v-for="(u, i) in user" :key="u">
            <div>#{{ i + 1 }}</div>
            <div>{{ u["user_id"] }}</div>
            <div>{{ getTabuuSkill(u["rating"], u["deviation"]).toFixed(2) }}</div>
            <div>{{ (u["deviation"] * 100).toFixed(2) }}</div>
            <div>{{ u["wins"] }}</div>
            <div>{{ u["losses"] }}</div>
            <div>{{ ((u["wins"] / (u["wins"] + u["losses"])) * 100).toFixed(2) }}%</div>
        </div>
    </div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";

const user = ref({});

function getTabuuSkill(rating: number, deviation: number) {
    return (rating - 3 * deviation) * 100 + 1000;
}

onMounted(async () => {
    const res = await fetch("http://localhost:8080/trueskill");
    user.value = await res.json();
    // @ts-ignore
    user.value.sort((a, b) => getTabuuSkill(b["rating"], b["deviation"]) - getTabuuSkill(a["rating"], a["deviation"]));
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
    grid-template-columns: 1fr 1.5fr 1fr 1fr 1fr 1fr 1fr;
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
