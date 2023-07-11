<template>
    <div class="grid">
        <RankedComponent />

        <SearchbarComponent @search="searchBar" />

        <div class="table-header">
            <div>Rank</div>
            <div>User</div>
            <div class="clickable" @click="sortTable(displayUser, 'user_id', ascendingColumns)">ID</div>
            <div class="clickable" @click="sortTable(displayUser, 'display_rating', ascendingColumns)">TabuuSkill</div>
            <div class="clickable" @click="sortTable(displayUser, 'deviation', ascendingColumns)">Deviation</div>
            <div class="clickable" @click="sortTable(displayUser, 'wins', ascendingColumns)">Wins</div>
            <div class="clickable" @click="sortTable(displayUser, 'losses', ascendingColumns)">Losses</div>
            <div>Winrate</div>
        </div>
        <div class="content" v-for="(u, i) in displayUser" :key="u" @click="fetchUser(users, u['user_id'])">
            <div>
                <i>#{{ i + 1 }}</i>
            </div>
            <div>
                <img :src="getUserAvatar(props.users, u['user_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["user_id"]) }}
            </div>
            <div>{{ u["user_id"] }}</div>
            <div>
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
            <div>{{ (u["wins"] as number).toLocaleString("en") }}</div>
            <div>{{ (u["losses"] as number).toLocaleString("en") }}</div>
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
import { sortTable } from "@/helpers/sortTable";
import { getUserName, getUserAvatar, fetchUser } from "@/helpers/userDetails";
import { ref, onMounted, type Ref } from "vue";

const props = defineProps(["users"]);

const user: Ref<any[]> = ref([]);
const displayUser: Ref<any[]> = ref([]);

const ascendingColumns = ref({
    user_id: false,
    display_rating: false,
    deviation: false,
    wins: false,
    losses: false
});

function searchBar(search: string) {
    displayUser.value = filterTable(user.value, props.users, search);
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/trueskill";

    const res = await fetch(url);
    user.value = await res.json();

    displayUser.value = user.value;

    sortTable(displayUser.value, "display_rating", ascendingColumns.value);
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr 1fr 1fr 1fr 1fr;
}
</style>
