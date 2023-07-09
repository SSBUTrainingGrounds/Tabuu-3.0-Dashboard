<template>
    <div class="grid">
        <SearchbarComponent @search="searchBar" />

        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div class="clickable" @click="sortTable(displayUser, 'id', ascendingColumns)">ID</div>
            <div class="clickable" @click="sortTable(displayUser, 'level', ascendingColumns)">Level</div>
            <div>Level Progress</div>
            <div class="clickable" @click="sortTable(displayUser, 'xp', ascendingColumns)">Total XP</div>
            <div class="clickable" @click="sortTable(displayUser, 'messages', ascendingColumns)">Messages</div>
        </div>
        <div class="content" v-for="(u, i) in displayUser" :key="u" @click="fetchUser(users, u['id'])">
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
import { ref, onMounted, type Ref } from "vue";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";
import { sortTable } from "@/helpers/sortTable";
import { filterTable } from "@/helpers/filterTable";
import SearchbarComponent from "@/components/SearchbarComponent.vue";

const user: Ref<any[]> = ref([]);
const displayUser: Ref<any[]> = ref([]);

const props = defineProps(["users"]);

const ascendingColumns = ref({
    id: false,
    level: false,
    xp: false,
    messages: false
});

function searchBar(search: string) {
    displayUser.value = filterTable(user.value, props.users, search);
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/leaderboard";

    const res = await fetch(url);
    user.value = await res.json();

    displayUser.value = user.value;

    sortTable(displayUser.value, "xp", ascendingColumns.value);
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