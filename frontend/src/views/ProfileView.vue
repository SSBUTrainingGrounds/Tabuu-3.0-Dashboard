<template>
    <div class="grid">
        <div class="table-header">
            <div>User</div>
            <div>ID</div>
            <div>Tag</div>
            <div>Region</div>
            <div>Mains</div>
            <div>Secondaries</div>
            <div>Pockets</div>
            <div>Note</div>
        </div>
        <div
            class="content"
            v-for="u in user"
            :key="u"
            :style="{
                // @ts-ignore
                borderLeftColor: '#' + u['colour'].toString(16).padStart(6, '0')
            }"
            @click="fetchUser(users, u['user_id'])"
        >
            <div>
                <img :src="getUserAvatar(props.users, u['user_id'])" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, u["user_id"]) }}
            </div>
            <div>{{ u["user_id"] }}</div>
            <div>{{ u["tag"] }}</div>
            <div>{{ u["region"] }}</div>
            <div>{{ getCharacters(u["mains"]) }}</div>
            <div>{{ getCharacters(u["secondaries"]) }}</div>
            <div>{{ getCharacters(u["pockets"]) }}</div>
            <div>{{ u["note"] }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
// Get the user info from the express server
import { ref, onMounted } from "vue";
import { fetchUser, getUserAvatar, getUserName } from "@/helpers/userDetails";

const user = ref([]);

const props = defineProps(["users"]);

function getCharacters(characters: string): string {
    if (characters === "") return "None";

    return characters
        .split(" ")
        .map((c) => c.split(":")[1])
        .join(", ");
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/profiles";

    const res = await fetch(url);
    user.value = await res.json();
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1.2fr 1.5fr 1fr 1fr 1fr 1fr 1fr 1fr;
    word-break: break-all;
}

.content {
    border-left: 0.5rem solid;
    border-radius: 0.2rem;
}
</style>
