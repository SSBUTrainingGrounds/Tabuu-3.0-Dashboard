<template>
    <div class="grid" id="user-table">
        <RankedComponent />

        <SearchbarComponent @search="searchBar" />

        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div
                class="clickable id-column"
                @click="displayUser = sortDisplayTable(displayUser, user, 'user_id', ascendingColumns)"
            >
                ID
            </div>
            <div
                class="clickable skill"
                @click="displayUser = sortDisplayTable(displayUser, user, 'display_rating', ascendingColumns)"
            >
                TabuuSkill
            </div>
            <div
                class="clickable"
                @click="displayUser = sortDisplayTable(displayUser, user, 'deviation', ascendingColumns)"
            >
                Deviation
            </div>
            <div
                class="clickable wins"
                @click="displayUser = sortDisplayTable(displayUser, user, 'wins', ascendingColumns)"
            >
                Wins
            </div>
            <div
                class="clickable losses"
                @click="displayUser = sortDisplayTable(displayUser, user, 'losses', ascendingColumns)"
            >
                Losses
            </div>
            <div>Winrate</div>
        </div>
        <div
            class="content"
            v-for="u in displayUser"
            :key="u['user_id']"
            @click="fetchUser(users, u['user_id'])"
            :class="props.userID === u['user_id'] ? 'highlighted-user' : ''"
        >
            <div>
                <i>#{{ u["rank"] }}</i>
            </div>
            <div>
                <img :src="getUserAvatar(props.users, u['user_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["user_id"]) }}
            </div>
            <div class="id-column">{{ u["user_id"] }}</div>
            <div class="skill">
                {{
                    (u["display_rating"] as number).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
            </div>
            <div>
                {{
                    (u["deviation"] * 100).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
            </div>
            <div class="wins">{{ (u["wins"] as number).toLocaleString("en") }}</div>
            <div class="losses">{{ (u["losses"] as number).toLocaleString("en") }}</div>
            <div>
                {{
                    (u["wins"] / (u["wins"] + u["losses"])).toLocaleString("en", {
                        style: "percent",
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import RankedComponent from "@/components/RankedComponent.vue";
import SearchbarComponent from "@/components/SearchbarComponent.vue";
import { filterTable } from "@/helpers/filterTable";
import { sortDisplayTable } from "@/helpers/sortTable";
import type { GuildUser } from "@/helpers/types";
import { getUserName, getUserAvatar, fetchUser } from "@/helpers/userDetails";
import { infiniteScroll } from "@/helpers/infiniteScroll";
import { ref, onMounted, type Ref } from "vue";

const props = defineProps({
    userID: {
        type: String,
        required: true
    },
    users: {
        type: Map<string, GuildUser>,
        required: true
    }
});

const user: Ref<any[]> = ref([]);
const displayUser: Ref<any[]> = ref([]);
let usersPerPage = 200;

let page = 2;
let throttle = false;
let currentSearch = "";

const ascendingColumns = ref({
    user_id: false,
    display_rating: false,
    deviation: false,
    wins: false,
    losses: false
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
    url.pathname = "/api/trueskill";

    const res = await fetch(url);
    user.value = await res.json();

    displayUser.value = user.value;
    displayUser.value = sortDisplayTable(displayUser.value, user.value, "display_rating", ascendingColumns.value);
    displayUser.value = displayUser.value.slice(0, usersPerPage);
    window.addEventListener("scroll", () => throttleScroll(1000));
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 0.3fr 1.2fr 1.2fr 0.5fr 0.5fr 1fr 1fr 1fr;
}

@media (max-width: 800px) {
    .content,
    .table-header {
        grid-template-columns: 0.5fr 3fr 0.7fr 0.7fr 1fr;
    }

    .skill {
        grid-column: 3 / 5;
    }

    .wins {
        grid-column: 3;
    }

    .losses {
        grid-column: 4;
    }
}
</style>
