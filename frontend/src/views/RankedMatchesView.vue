<template>
    <div class="grid">
        <RankedComponent />

        <div class="table-header">
            <div class="clickable" @click="sortTable(matches, 'match_id', ascendingColumns)">Match ID</div>
            <div class="winner clickable" @click="sortTable(matches, 'winner_id', ascendingColumns)">Winner</div>
            <div class="loser clickable" @click="sortTable(matches, 'loser_id', ascendingColumns)">Loser</div>
            <div class="clickable" @click="sortTable(matches, 'timestamp', ascendingColumns)">Time</div>
        </div>
        <div
            class="content"
            v-for="(m, i) in matches"
            :key="i"
            @click="
                {
                    fetchUser(users, m['winner_id']);
                    fetchUser(users, m['loser_id']);
                }
            "
        >
            <div>{{ m["match_id"] }}</div>
            <div>
                <img :src="getUserAvatar(props.users, m['winner_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, m["winner_id"]) }}
            </div>
            <div class="winner-rating">
                {{ (m["old_winner_display_rating"] as number).toFixed(2) }} →
                {{ (m["new_winner_display_rating"] as number).toFixed(2) }}
                {{ getRatingChangeText(m["winner_display_rating_change"]) }}
            </div>
            <div>
                <img :src="getUserAvatar(props.users, m['loser_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, m["loser_id"]) }}
            </div>
            <div class="loser-rating">
                {{ (m["old_loser_display_rating"] as number).toFixed(2) }} →
                {{ (m["new_loser_display_rating"] as number).toFixed(2) }}
                {{ getRatingChangeText(m["loser_display_rating_change"]) }}
            </div>
            <div>{{ new Date(m["timestamp"] * 1000).toLocaleString() }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import RankedComponent from "@/components/RankedComponent.vue";
import { sortTable } from "@/helpers/sortTable";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";
import { onMounted, ref } from "vue";

const props = defineProps(["users"]);

function getRatingChangeText(ratingChange: number): string {
    if (ratingChange > 0) {
        return `(↑${ratingChange.toFixed(2)})`;
    } else {
        return `(↓${Math.abs(ratingChange).toFixed(2)})`;
    }
}

const matches = ref([]);

const ascendingColumns = ref({
    match_id: false,
    winner_id: false,
    loser_id: false,
    timestamp: false
});

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/matches";

    const res = await fetch(url);
    matches.value = await res.json();

    sortTable(matches.value, "timestamp", ascendingColumns.value);
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1fr 1fr 1.5fr 1fr 1.5fr 1fr;
}

.winner {
    grid-column: 2 / 4;
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
</style>
