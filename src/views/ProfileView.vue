<template>
    <div class="grid">
        <div class="header">
            <div>Index</div>
            <div>ID</div>
            <div>Tag</div>
            <div>Region</div>
            <div>Mains</div>
            <div>Secondaries</div>
            <div>Pockets</div>
            <div>Note</div>
        </div>
        <div
            class="user"
            v-for="(u, i) in user"
            :key="u"
            :style="{
                // @ts-ignore
                borderLeftColor: '#' + u['colour'].toString(16).padStart(6, '0')
            }"
        >
            <div>#{{ i + 1 }}</div>
            <div>{{ u["user_id"] }}</div>
            <div>{{ u["tag"] }}</div>
            <div>{{ u["region"] }}</div>
            <div>{{ getCharacters(u["mains"]) }}</div>
            <div>{{ getCharacters(u["secondaries"]) }}</div>
            <div>{{ getCharacters(u["pockets"]) }}</div>
            <div>{{ u["note"] }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";

const user = ref({});

function getCharacters(characters: string): string {
    if (characters === "") return "None";

    return characters
        .split(" ")
        .map((c) => c.split(":")[1])
        .join(", ");
}

onMounted(async () => {
    const res = await fetch("http://localhost:8080/profiles");
    user.value = await res.json();
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
    grid-template-columns: 1fr 1.5fr 1fr 1fr 1fr 1fr 1fr 1fr;
    word-break: break-all;
    grid-gap: 1rem;
    padding: 1rem;
    border-radius: 1rem;
}

.user {
    border-left: 0.5rem solid;
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
