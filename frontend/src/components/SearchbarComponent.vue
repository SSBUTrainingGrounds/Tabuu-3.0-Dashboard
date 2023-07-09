<template>
    <div class="searchbar">
        <input type="text" placeholder="Search for a user..." v-model="search" @input="$emit('search', search)" />
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const search = ref("");

defineEmits(["search"]);

defineProps(["users", "table"]);
</script>

<style scoped>
@import "../assets/styles.css";

.searchbar {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
    width: 100%;
    max-width: 800px;
    padding: 0.5rem;
}

.searchbar input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid var(--green);
    border-radius: 0.5rem;
    outline: none;
    font-size: large;
    color: var(--green);
    background-color: var(--dark-gray);
}

.searchbar input:focus {
    border: 1px solid var(--light-green);
}

.searchbar input::placeholder {
    color: var(--green);
}
</style>
