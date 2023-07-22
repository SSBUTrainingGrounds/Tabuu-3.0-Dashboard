<template>
    <div class="grid" id="user-table" v-if="matches.length !== 0">
        <RankedComponent />

        <SearchbarComponent @search="searchBar" />

        <div class="table-header">
            <div
                class="clickable"
                @click="displayMatches = sortDisplayTable(displayMatches, matches, 'match_id', ascendingColumns)"
            >
                Match ID
            </div>
            <div
                class="clickable"
                @click="displayMatches = sortDisplayTable(displayMatches, matches, 'timestamp', ascendingColumns)"
            >
                Time
            </div>
            <div
                class="winner clickable"
                @click="displayMatches = sortDisplayTable(displayMatches, matches, 'winner_id', ascendingColumns)"
            >
                Winner
            </div>
            <div
                class="loser clickable"
                @click="displayMatches = sortDisplayTable(displayMatches, matches, 'loser_id', ascendingColumns)"
            >
                Loser
            </div>
        </div>
        <div
            class="content"
            v-for="(m, i) in displayMatches"
            :key="i"
            @click="
                {
                    fetchUser(users, m['winner_id']);
                    fetchUser(users, m['loser_id']);
                }
            "
            :class="props.userID === m['winner_id'] || props.userID === m['loser_id'] ? 'highlighted-user' : ''"
        >
            <div>{{ m["match_id"] }}</div>
            <div>{{ new Date(m["timestamp"] * 1000).toLocaleString() }}</div>
            <div>
                <img :src="getUserAvatar(props.users, m['winner_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, m["winner_id"]) }}
            </div>
            <div class="winner-rating">
                {{
                    (m["old_winner_display_rating"] as number).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
                →
                {{
                    (m["new_winner_display_rating"] as number).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
                {{ getRatingChangeText(m["winner_display_rating_change"]) }}
            </div>
            <div>
                <img :src="getUserAvatar(props.users, m['loser_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, m["loser_id"]) }}
            </div>
            <div class="loser-rating">
                {{
                    (m["old_loser_display_rating"] as number).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
                →
                {{
                    (m["new_loser_display_rating"] as number).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
                {{ getRatingChangeText(m["loser_display_rating_change"]) }}
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
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";
import { getRatingChangeText } from "@/helpers/rating";
import { infiniteScroll } from "@/helpers/infiniteScroll";
import { onMounted, ref, type Ref } from "vue";
import router from "@/router";

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

const matches: Ref<any[]> = ref([]);
const displayMatches: Ref<any[]> = ref([]);
let matchesPerPage = 200;

let page = 2;
let throttle = false;
let currentSearch = "";

const ascendingColumns = ref({
    match_id: false,
    winner_id: false,
    loser_id: false,
    timestamp: false
});

function searchBar(search: string) {
    if (!search) {
        displayMatches.value = matches.value.slice(0, matchesPerPage);
        currentSearch = "";
        return;
    }

    currentSearch = search;
    displayMatches.value = filterTable(matches.value, props.users, search);
    page = 2;
}

function throttleScroll(time: number) {
    if (throttle) return;
    throttle = true;
    setTimeout(() => {
        page = infiniteScroll(displayMatches.value, matches.value, page, matchesPerPage, currentSearch);
        throttle = false;
    }, time);
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/matches";

    const res = await fetch(url, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${localStorage.getItem("discordToken")}`
        }
    });

    if (res.ok) {
        matches.value = await res.json();

        displayMatches.value = matches.value;
        displayMatches.value = sortDisplayTable(
            displayMatches.value,
            matches.value,
            "timestamp",
            ascendingColumns.value
        );
        displayMatches.value = displayMatches.value.slice(0, matchesPerPage);
        window.addEventListener("scroll", () => throttleScroll(1000));
    } else {
        router.push({ path: "/" });
    }
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 0.6fr 1fr 1.5fr 1fr 1.5fr 1fr;
}

.winner {
    grid-column: 3 / 4;
}

.loser {
    grid-column: 4 / 6;
}

.winner-rating {
    color: var(--light-green);
}

.loser-rating {
    color: var(--light-red);
}

@media (max-width: 800px) {
    .content,
    .table-header {
        grid-template-columns: 1fr 1fr;
    }

    .winner {
        grid-column: 1;
    }

    .loser {
        grid-column: 2;
    }
}
</style>
