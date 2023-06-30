<template>
    <div class="grid">
        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div>ID</div>
            <div>Level</div>
            <div>Level Progress</div>
            <div>Total XP</div>
            <div>Messages</div>
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
// Get the user info from the express server
import { ref, onMounted } from "vue";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";

const user = ref([]);

const props = defineProps(["users"]);

onMounted(async () => {
    let url = new URL(window.location.href);
    url.port = "8080";
    url.pathname = "/leaderboard";

    const res = await fetch(url);
    user.value = await res.json();
    user.value.sort((a, b) => b["xp"] - a["xp"]);
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
