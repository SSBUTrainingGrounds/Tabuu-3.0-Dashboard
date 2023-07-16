<template>
    <div class="grid" id="user-table">
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
        <div
            class="content"
            v-for="u in displayUser"
            :key="u['id']"
            @click="fetchUser(users, u['id'])"
            :class="props.userID === u['id'] ? 'highlighted-user' : ''"
        >
            <div>
                <i>#{{ u["rank"] }}</i>
            </div>
            <div>
                <img :src="getUserAvatar(props.users, u['id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["id"]) }}
            </div>
            <div>{{ u["id"] }}</div>
            <div>{{ (u["level"] as number).toLocaleString("en") }}</div>
            <div>
                <div
                    class="pie-progress"
                    :style="{
                        backgroundImage: `conic-gradient(var(--light-green) ${u['xp_progress'] * 100}%, var(--gray) 0%)`
                    }"
                ></div>
                {{
                    (u["xp_progress"] as number).toLocaleString("en", {
                        style: "percent",
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
            </div>
            <div>{{ (u["xp"] as number).toLocaleString("en") }}</div>
            <div>{{ (u["messages"] as number).toLocaleString("en") }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, type Ref } from "vue";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";
import { sortTable } from "@/helpers/sortTable";
import { filterTable } from "@/helpers/filterTable";
import { infiniteScroll } from "@/helpers/infiniteScroll";
import SearchbarComponent from "@/components/SearchbarComponent.vue";
import type { GuildUser } from "@/helpers/types";

const user: Ref<any[]> = ref([]);
const displayUser: Ref<any[]> = ref([]);
const usersPerPage = 200;

let page = 2;
let throttle = false;
let currentSearch = "";

const props = defineProps({
    users: {
        type: Map<string, GuildUser>,
        required: true
    },
    userID: {
        type: String,
        required: true
    }
});

const ascendingColumns = ref({
    id: false,
    level: false,
    xp: false,
    messages: false
});

function searchBar(search: string) {
    if (!search) {
        displayUser.value = user.value.slice(0, usersPerPage);
        currentSearch = "";
        return;
    }

    currentSearch = search;
    displayUser.value = filterTable(user.value, props.users, search);
    page = 2;
}

function throttleScroll(time: number) {
    if (throttle) return;

    throttle = true;

    setTimeout(() => {
        page = infiniteScroll(displayUser.value, user.value, page, usersPerPage, currentSearch);
        throttle = false;
    }, time);
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/leaderboard";

    const res = await fetch(url);
    user.value = await res.json();

    displayUser.value = user.value;

    sortTable(displayUser.value, "xp", ascendingColumns.value);
    displayUser.value = displayUser.value.slice(0, usersPerPage);
    window.addEventListener("scroll", () => throttleScroll(1000));
});
</script>

<style scoped>
@import "@/assets/styles.css";

.content,
.table-header {
    grid-template-columns: 0.3fr 1.2fr 1.2fr 1fr 1fr 1fr 1fr;
}

.pie-progress {
    width: 1rem;
    height: 1rem;
    border-radius: 50%;
}
</style>
