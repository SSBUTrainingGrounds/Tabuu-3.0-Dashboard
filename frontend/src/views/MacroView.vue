<template>
    <div class="grid">
        <div class="macro-input content" v-if="isAdmin">
            <h2 class="new-header">Create New Macro</h2>
            <input type="text" class="macro-input-name" placeholder="Macro Name" id="macro-name" v-model="name" />
            <input type="text" class="macro-input-value" placeholder="Macro Value" id="macro-value" v-model="payload" />
            <button class="macro-button" @click="sendMacro">+</button>
        </div>

        <div
            class="table-header"
            :style="{ gridTemplateColumns: isAdmin ? '1fr 2fr 1.2fr 0.5fr 0.5fr' : '1fr 2fr 1.2fr 0.5fr' }"
        >
            <div class="clickable" @click="sortTable(allMacros, 'name', ascendingColumns)">Name</div>
            <div>Payload</div>
            <div class="clickable" @click="sortTable(allMacros, 'author', ascendingColumns)">Author</div>
            <div class="clickable" @click="sortTable(allMacros, 'uses', ascendingColumns)">Uses</div>
        </div>

        <div
            v-for="(macro, i) in allMacros"
            :key="i"
            class="content"
            :style="{ gridTemplateColumns: isAdmin ? '1fr 2fr 1.2fr 0.5fr 0.5fr' : '1fr 2fr 1.2fr 0.5fr' }"
        >
            <div class="break-text">%{{ macro.name }}</div>
            <div class="break-text">{{ displayMacroPayload(macro.payload) }}</div>
            <div>
                <img :src="getUserAvatar(props.users, macro.author)" alt="User Avatar" class="avatar-preview" />
                {{ getUserName(props.users, macro.author) }}
            </div>
            <div>{{ macro.uses.toLocaleString("en") }}</div>
            <button class="delete-button" v-if="isAdmin" @click="deleteMacro(macro.name, i)">X</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from "vue";
import { getUserName, getUserAvatar } from "@/helpers/userDetails";
import type { GuildUser, Macro } from "@/helpers/types";
import { sortTable } from "@/helpers/sortTable";
import { displayMacroPayload } from "@/helpers/macroDisplay";

const props = defineProps({
    isAdmin: {
        type: Boolean,
        required: true
    },
    userID: {
        type: String,
        required: true
    },
    users: {
        type: Map<string, GuildUser>,
        required: true
    }
});

let name = ref("");
let payload = ref("");

let allMacros: Ref<Macro[]> = ref([]);

const ascendingColumns = ref({
    uses: false,
    name: false,
    author: false
});

function sendMacro() {
    for (let i = 0; allMacros.value.length > i; i++) {
        if (allMacros.value[i].name === name.value) {
            alert("Macro name already exists!");
            return;
        }
    }

    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/macro_new";

    fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name.value,
            payload: payload.value,
            uses: 0,
            author: props.userID,
            discord_token: localStorage.getItem("discordToken")
        })
    }).then((res) => {
        if (res.status !== 200) {
            alert(res.statusText);
            return;
        } else {
            allMacros.value.push({
                name: name.value,
                payload: payload.value,
                author: props.userID,
                uses: 0
            });

            alert(`Macro %${name.value} created!`);

            name.value = "";
            payload.value = "";
        }
    });
}

function deleteMacro(name: string, i: number) {
    if (!confirm(`Are you sure you want to delete %${name}?`)) {
        return;
    }

    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/macro_delete";

    fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name,
            discord_token: localStorage.getItem("discordToken")
        })
    }).then((res) => {
        if (res.status !== 200) {
            alert(res.statusText);
            return;
        } else {
            allMacros.value.splice(i, 1);
        }
    });
}

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/macro_get";

    const res = await fetch(url);
    allMacros.value = await res.json();

    sortTable(allMacros.value, "uses", ascendingColumns.value);
});
</script>

<style scoped>
@import "../assets/styles.css";

.content {
    word-break: normal;
}

.new-header {
    grid-column: 1 / 5;
}

@media (max-width: 1300px) {
    .break-text {
        word-break: break-all;
    }
}
</style>
