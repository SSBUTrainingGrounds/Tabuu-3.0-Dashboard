<template>
    <div v-for="u in user" :key="u">{{ u }}</div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";

const user = ref({});

onMounted(async () => {
    const res = await fetch("http://localhost:8080/trueskill");
    user.value = await res.json();

    console.log(user.value);
});
</script>
