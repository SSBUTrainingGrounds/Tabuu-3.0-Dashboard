<template>
    <div class="grid">
        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div>ID</div>
            <div>Level</div>
            <div>XP</div>
            <div>Messages</div>
        </div>
        <div class="user" v-for="(u, i) in user" :key="u">
            <div>#{{ i + 1 }}</div>
            <div>
                <img :src="getRandomUserAvatar()" alt="User Avatar" class="avatar-preview" /> {{ getRandomUserName() }}
            </div>
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
import { getRandomUserAvatar, getRandomUserName } from "../helpers/mockUsers";

const user = ref({});

onMounted(async () => {
    const res = await fetch("http://localhost:8080/leaderboard");
    user.value = await res.json();
    // @ts-ignore
    user.value.sort((a, b) => b["xp"] - a["xp"]);
});
</script>

<style scoped>
.user,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr 1fr 1fr;
}
</style>
