<template>
    <div>WIP</div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";

const user = ref({});

onMounted(async () => {
    const res = await fetch("http://localhost:8080/profiles");
    user.value = await res.json();
});
</script>
