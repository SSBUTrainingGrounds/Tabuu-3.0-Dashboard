<template>
    <div class="grid">
        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div>ID</div>
            <div>TabuuSkill</div>
            <div>Deviation</div>
            <div>Wins</div>
            <div>Losses</div>
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
            <div>{{ getTabuuSkill(u["rating"], u["deviation"]).toFixed(2) }}</div>
            <div>{{ (u["deviation"] * 100).toFixed(2) }}</div>
            <div>{{ u["wins"] }}</div>
            <div>{{ u["losses"] }}</div>
            <div>{{ ((u["wins"] / (u["wins"] + u["losses"])) * 100).toFixed(2) }}%</div>
        </div>
    </div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { getUserName, getUserAvatar, fetchUser } from "@/helpers/userDetails";
import { ref, onMounted } from "vue";

const props = defineProps(["users"]);

const user = ref([]);

function getTabuuSkill(rating: number, deviation: number) {
    return (rating - 3 * deviation) * 100 + 1000;
}

onMounted(async () => {
    let url = new URL(window.location.href);
    url.port = "8080";
    url.pathname = "/trueskill";

    const res = await fetch(url);
    user.value = await res.json();

    user.value.sort((a, b) => getTabuuSkill(b["rating"], b["deviation"]) - getTabuuSkill(a["rating"], a["deviation"]));
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr 1fr 1fr 1fr 1fr;
}
</style>
