<template>
    <div class="grid">
        <RankedComponent />

        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div class="clickable" @click="sortTable(user, 'user_id', ascendingColumns)">ID</div>
            <div class="clickable" @click="sortTable(user, 'display_rating', ascendingColumns)">TabuuSkill</div>
            <div class="clickable" @click="sortTable(user, 'deviation', ascendingColumns)">Deviation</div>
            <div class="clickable" @click="sortTable(user, 'wins', ascendingColumns)">Wins</div>
            <div class="clickable" @click="sortTable(user, 'losses', ascendingColumns)">Losses</div>
            <div>Winrate</div>
        </div>
        <div class="content" v-for="(u, i) in user" :key="u" @click="fetchUser(users, u['user_id'])">
            <div>
                <i>#{{ i + 1 }}</i>
            </div>
            <div>
                <img :src="getUserAvatar(props.users, u['user_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["user_id"]) }}
            </div>
            <div>{{ u["user_id"] }}</div>
            <div>{{ (u["display_rating"] as number).toFixed(2) }}</div>
            <div>{{ (u["deviation"] * 100).toFixed(2) }}</div>
            <div>{{ u["wins"] }}</div>
            <div>{{ u["losses"] }}</div>
            <div>{{ ((u["wins"] / (u["wins"] + u["losses"])) * 100).toFixed(2) }}%</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import RankedComponent from "@/components/RankedComponent.vue";
import { sortTable } from "@/helpers/sortTable";
import { getUserName, getUserAvatar, fetchUser } from "@/helpers/userDetails";
import { ref, onMounted } from "vue";

const props = defineProps(["users"]);

const user = ref([]);

const ascendingColumns = ref({
    user_id: false,
    display_rating: false,
    deviation: false,
    wins: false,
    losses: false
});

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/trueskill";

    const res = await fetch(url);
    user.value = await res.json();

    sortTable(user.value, "display_rating", ascendingColumns.value);
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr 1fr 1fr 1fr 1fr;
}
</style>
