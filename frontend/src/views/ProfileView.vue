<template>
    <div class="grid">
        <SearchbarComponent @search="searchBar" />

        <div class="table-header">
            <div>User</div>
            <div class="id-column">ID</div>
            <div>Tag</div>
            <div>Region</div>
            <div>Mains</div>
            <div>Secondaries</div>
            <div>Pockets</div>
            <div>Note</div>
        </div>
        <div
            class="content"
            v-for="u in displayUser"
            :key="u"
            :style="{
                // @ts-ignore
                borderLeftColor: '#' + u['colour'].toString(16).padStart(6, '0')
            }"
            :class="props.userID === u['user_id'] ? 'highlighted-user' : ''"
            @click="fetchUser(users, u['user_id'])"
        >
            <div>
                <img :src="getUserAvatar(props.users, u['user_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["user_id"]) }}
            </div>
            <div class="id-column">{{ u["user_id"] }}</div>
            <div>{{ u["tag"] }}</div>
            <div>{{ u["region"] }}</div>
            <div>
                <div v-for="character in getCharacters(u['mains'])" :key="character" class="emoji-grid">
                    <img :src="character" alt="Character" class="avatar-preview" />
                </div>
            </div>
            <div>
                <div v-for="character in getCharacters(u['secondaries'])" :key="character" class="emoji-grid">
                    <img :src="character" alt="Character" class="avatar-preview" />
                </div>
            </div>
            <div>
                <div v-for="character in getCharacters(u['pockets'])" :key="character" class="emoji-grid">
                    <img :src="character" alt="Character" class="avatar-preview" />
                </div>
            </div>
            <div class="note">{{ u["note"] }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, type Ref } from "vue";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";
import { getCharacters } from "@/helpers/characterEmojis";
import SearchbarComponent from "@/components/SearchbarComponent.vue";
import { filterTable } from "@/helpers/filterTable";
import type { GuildUser } from "@/helpers/types";

const user: Ref<any[]> = ref([]);
const displayUser: Ref<any[]> = ref([]);

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

// Not sure if it would make sense to implement sorting for this view.

function searchBar(search: string) {
    displayUser.value = filterTable(user.value, props.users, search);
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/profiles";

    const res = await fetch(url);
    user.value = await res.json();

    displayUser.value = user.value;
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1.5fr 1.3fr 1fr 1fr 0.8fr 0.8fr 0.8fr 1.5fr;
}

.content {
    border-left: 0.5rem solid;
    border-radius: 0.2rem;
}

@media (max-width: 800px) {
    .content,
    .table-header {
        grid-template-columns: 2fr 1.2fr 1fr;
    }

    .note {
        grid-column: 1 / span 3;
    }
}
</style>
