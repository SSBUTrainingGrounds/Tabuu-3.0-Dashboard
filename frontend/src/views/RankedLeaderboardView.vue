<template>
    <div class="grid" id="user-table" v-if="user.length !== 0">
        <RankedComponent />

        <SearchbarComponent @search="searchBar" :user-row="getUserRow(props.userID, user)" />

        <div class="table-header" @click="userDetails = Array(user.length).fill(false)">
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
            v-for="(u, i) in displayUser"
            :key="u['user_id']"
            @click="
                {
                    fetchUser(users, u['user_id']);
                    userDetails[i] = !userDetails[i];
                }
            "
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
            <div class="skill" v-if="!userDetails[i]">
                {{
                    (u["display_rating"] as number).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
            </div>
            <div v-if="!userDetails[i]">
                {{
                    (u["deviation"] * 100).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}
            </div>
            <div class="wins" v-if="!userDetails[i]">{{ (u["wins"] as number).toLocaleString("en") }}</div>
            <div class="losses" v-if="!userDetails[i]">{{ (u["losses"] as number).toLocaleString("en") }}</div>
            <div v-if="!userDetails[i]">
                {{
                    (u["win_percentage"] || 0).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}%
            </div>
            <div class="user-details" v-if="userDetails[i]">
                <div class="description">TabuuSkill:</div>
                <div class="value">
                    {{
                        (u["display_rating"] as number).toLocaleString("en", {
                            minimumFractionDigits: 2,
                            maximumFractionDigits: 2
                        })
                    }}
                </div>

                <div class="description">Deviation:</div>
                <div class="value">
                    {{
                        (u["deviation"] * 100).toLocaleString("en", {
                            minimumFractionDigits: 2,
                            maximumFractionDigits: 2
                        })
                    }}
                </div>

                <div class="description">Wins:</div>
                <div class="value">{{ (u["wins"] as number).toLocaleString("en") }}</div>

                <div class="description">Losses:</div>
                <div class="value">{{ (u["losses"] as number).toLocaleString("en") }}</div>

                <div class="description">Winrate:</div>
                <div class="value">
                    {{
                        (u["win_percentage"] || 0).toLocaleString("en", {
                            minimumFractionDigits: 2,
                            maximumFractionDigits: 2
                        })
                    }}%
                </div>

                <div class="description">Total Matches:</div>
                <div class="value">{{ u["wins"] + u["losses"] || 0 }}</div>

                <div class="description">
                    Recent Performance:<br />
                    (Last 10 Matches)
                </div>
                <div class="value">
                    {{ getRatingChangeText(u["recent_performance"]) }} ({{ u["recent_matches"] }})
                    <RatingChartComponent :ratings="u['last_ratings']" />
                </div>

                <div class="description">Longest Winning Streak:</div>
                <div class="value">
                    {{ (u["longest_win_streak"] as number).toLocaleString("en") }} (Current:
                    {{ u["current_win_streak"] }})
                </div>

                <div class="description">Longest Losing Streak:</div>
                <div class="value">
                    {{ (u["longest_loss_streak"] as number).toLocaleString("en") }} (Current:
                    {{ u["current_loss_streak"] }})
                </div>

                <div class="description">All-Time Highest Rating:</div>
                <div class="value">
                    {{
                        (u["all_time_highest_rating"][0] as number).toLocaleString("en", {
                            minimumFractionDigits: 2,
                            maximumFractionDigits: 2
                        })
                    }}
                    ({{
                        u["all_time_highest_rating"][1]
                            ? new Date(u["all_time_highest_rating"][1] * 1000).toLocaleString()
                            : "Unknown Date"
                    }})
                </div>

                <div class="description">Highest Win:</div>
                <div class="value">
                    {{
                        (u["highest_win"]["old_loser_display_rating"] || 0).toLocaleString("en", {
                            minimumFractionDigits: 2,
                            maximumFractionDigits: 2
                        })
                    }}
                    <br />
                    (vs. {{ getUserName(props.users, u["highest_win"]["loser_id"]) || "Unknown User" }},
                    {{
                        u["highest_win"]["timestamp"]
                            ? new Date(u["highest_win"]["timestamp"] * 1000).toLocaleString()
                            : "Unknown Date"
                    }})
                </div>

                <div class="description">Last Match:</div>
                <div class="value">
                    {{ u["last_match"]["winner_id"] === u["user_id"] ? "Win" : "Loss" }} vs.
                    {{
                        (u["last_match"]["winner_id"] === u["user_id"]
                            ? getUserName(props.users, u["last_match"]["loser_id"])
                            : getUserName(props.users, u["last_match"]["winner_id"])) || "Unknown User"
                    }},
                    {{
                        u["last_match"]["timestamp"]
                            ? new Date(u["last_match"]["timestamp"] * 1000).toLocaleString()
                            : "Unknown Date"
                    }}
                </div>

                <div class="description">Average Opponent:</div>
                <div class="value">
                    ~{{
                        (u["avg_opponent_rating"] || 0).toLocaleString("en", {
                            minimumFractionDigits: 2,
                            maximumFractionDigits: 2
                        })
                    }}
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import RankedComponent from "@/components/RankedComponent.vue";
import SearchbarComponent from "@/components/SearchbarComponent.vue";
import RatingChartComponent from "@/components/RatingChartComponent.vue";

import { filterTable } from "@/helpers/filterTable";
import { sortDisplayTable } from "@/helpers/sortTable";
import type { GuildUser } from "@/helpers/types";
import { getUserName, getUserAvatar, fetchUser } from "@/helpers/userDetails";
import { infiniteScroll } from "@/helpers/infiniteScroll";
import { ref, onMounted, type Ref } from "vue";
import router from "@/router";
import { getRatingChangeText } from "@/helpers/rating";
import { getUserRow } from "@/helpers/userRow";

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
const userDetails: Ref<any[]> = ref([]);
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

    const res = await fetch(url, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${localStorage.getItem("discordToken")}`
        }
    });

    if (res.ok) {
        user.value = await res.json();
        userDetails.value = Array(user.value.length).fill(false);

        displayUser.value = user.value;
        displayUser.value = sortDisplayTable(displayUser.value, user.value, "display_rating", ascendingColumns.value);
        displayUser.value = displayUser.value.slice(0, usersPerPage);
        window.addEventListener("scroll", () => throttleScroll(1000));
    } else {
        router.push({ path: "/" });
    }
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
