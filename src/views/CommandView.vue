<template>
    <div class="grid">
        <div class="header">
            <div>Rank</div>
            <div>Command Name</div>
            <div>Uses</div>
            <div>Last Used</div>
        </div>
        <div class="command" v-for="(c, i) in command" :key="c">
            <div>#{{ i + 1 }}</div>
            <div>%{{ c["command"] }}</div>
            <div>{{ c["uses"] }}</div>
            <div>{{ c["last_used"] !== 0 ? new Date(c["last_used"] * 1000).toLocaleString() : "N/A" }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";

const command = ref({});

onMounted(async () => {
    const res = await fetch("http://localhost:8080/commands");
    command.value = await res.json();
    // @ts-ignore
    command.value.sort((a, b) => b["uses"] - a["uses"]);
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

.command,
.header {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    grid-gap: 1rem;
    padding: 1rem;
    border-radius: 0.5rem;
}

.command:nth-child(odd) {
    background-color: var(--black);
}

.command:nth-child(even) {
    background-color: var(--dark-gray);
}

.user:hover {
    background-color: var(--green);
}
</style>
