<template>
    <div class="grid">
        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div class="clickable" @click="sortTable(user, 'id', ascendingColumns)">ID</div>
            <div class="clickable" @click="sortTable(user, 'level', ascendingColumns)">Level</div>
            <div>Level Progress</div>
            <div class="clickable" @click="sortTable(user, 'xp', ascendingColumns)">Total XP</div>
            <div class="clickable" @click="sortTable(user, 'messages', ascendingColumns)">Messages</div>
        </div>
        <div class="content" v-for="(u, i) in user" :key="u" @click="fetchUser(users, u['id'])">
            <div>
                <i>#{{ i + 1 }}</i>
            </div>
            <div>
                <img :src="getUserAvatar(props.users, u['id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["id"]) }}
            </div>
            <div>{{ u["id"] }}</div>
            <div>{{ u["level"] }}</div>
            <div>
                <div
                    class="pie-progress"
                    :style="{
                        backgroundImage: `conic-gradient(var(--light-green) ${u['xp_progress'] * 100}%, var(--gray) 0%)`
                    }"
                ></div>
                {{ ((u["xp_progress"] * 100) as number).toFixed(2) }}%
            </div>
            <div>{{ u["xp"] }}</div>
            <div>{{ u["messages"] }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";
import { sortTable } from "@/helpers/sortTable";

const user = ref([]);

const props = defineProps(["users"]);

const ascendingColumns = ref({
    id: false,
    level: false,
    xp: false,
    messages: false
});

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/leaderboard";

    const res = await fetch(url);
    user.value = await res.json();

    sortTable(user.value, "xp", ascendingColumns.value);
});
</script>

<style scoped>
@import "@/assets/styles.css";

.content,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr 1fr 1fr 1fr;
}

.pie-progress {
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
}
</style>
