<template>
    <div class="searchbar">
        <i class="fas fa-search"></i>
        <input type="text" placeholder="Search for a user..." v-model="search" @input="$emit('search', search)" />
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const search = ref("");

defineEmits<{
    search: [search: string];
}>();
</script>

<style scoped>
@import "../assets/styles.css";

.searchbar {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
    width: 95%;
    max-width: 800px;
    padding: 0.5rem;
    border: 2px solid var(--green);
    border-radius: 0.5rem;
    outline: none;
    font-size: large;
    color: var(--light-gray);
    background-color: var(--dark-gray);
}

.searchbar input {
    width: 100%;
    padding: 0.5rem;
    outline: none;
    border: none;
    font-size: large;
    color: var(--white);
    background-color: var(--dark-gray);
}

.searchbar i {
    margin-right: 0.5rem;
    color: var(--light-gray);
}

.searchbar:focus-within {
    border: 2px solid var(--light-green);
}
.searchbar input::placeholder {
    color: var(--light-gray);
}
</style>
