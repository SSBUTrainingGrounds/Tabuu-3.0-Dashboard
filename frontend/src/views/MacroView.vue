<template>
    <div class="grid">
        <div class="macro-input content" v-if="isAdmin">
            <h2 class="new-header">Create New Macro</h2>
            <input type="text" class="macro-input-name" placeholder="Macro Name" id="macro-name" v-model="name" />
            <input type="text" class="macro-input-value" placeholder="Macro Value" id="macro-value" v-model="payload" />
            <button class="macro-button" @click="sendMacro">+</button>
        </div>

        <div class="table-header" :style="{ gridTemplateColumns: isAdmin ? '1fr 1fr 1fr 1fr 1fr' : '1fr 1fr 1fr 1fr' }">
            <div class="clickable" @click="sortTable(allMacros, 'name', ascendingColumns)">Name</div>
            <div>Payload</div>
            <div class="clickable" @click="sortTable(allMacros, 'author', ascendingColumns)">Author</div>
            <div class="clickable" @click="sortTable(allMacros, 'uses', ascendingColumns)">Uses</div>
        </div>

        <div
            v-for="(macro, i) in allMacros"
            :key="i"
            class="content"
            :style="{ gridTemplateColumns: isAdmin ? '1fr 1fr 1fr 1fr 1fr' : '1fr 1fr 1fr 1fr' }"
        >
            <div>%{{ macro.name }}</div>
            <div>{{ macro.payload }}</div>
            <div>{{ getUserName(props.users, macro.author) }}</div>
            <div>{{ macro.uses }}</div>
            <button class="delete-button" v-if="isAdmin" @click="deleteMacro(macro.name, i)">X</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from "vue";
import { getUserName } from "@/helpers/userDetails";
import type { Macro } from "@/helpers/types";
import { sortTable } from "@/helpers/sortTable";

const props = defineProps(["userID", "isAdmin", "users"]);

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
    url.pathname = "/macro_new";

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
    url.pathname = "/macro_delete";

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
    url.pathname = "/macro_get";

    const res = await fetch(url);
    allMacros.value = await res.json();

    sortTable(allMacros.value, "uses", ascendingColumns.value);
});
</script>

<style scoped>
@import "../assets/styles.css";

.new-header {
    grid-column: 1 / 5;
}

.macro-input {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    background-color: var(--dark-gray);
    border-radius: 1rem;
}

.macro-input-name,
.macro-input-value {
    border-radius: 0.5rem;
    border: 1px solid #ccc;
    font-size: 1rem;
    background-color: var(--light-gray);
    color: var(--white);
}

.macro-input-name {
    grid-column: 1 / 2;
}

.macro-input-value {
    grid-column: 2 / 5;
}

.macro-button,
.delete-button {
    all: unset;
    cursor: pointer;
    margin: auto;
    text-decoration: none;
    font-size: 1rem;
    font-weight: bold;
    transition: opacity 0.2s ease-in-out;
    grid-column: 5;
    align-self: start;
    height: 2.5rem;
    width: 2.5rem;
    border-radius: 50%;
    text-align: center;
    background-color: var(--dark-gray);
}

.macro-button {
    border: 1px solid var(--green);
    color: var(--green);
}

.macro-button:hover {
    background-color: var(--green);
    color: var(--light-green);
    border: solid 1px var(--light-green);
}

.delete-button {
    border: solid 1px var(--red);
    color: var(--red);
}

.delete-button:hover {
    background-color: var(--red);
    color: var(--light-red);
    border: solid 1px var(--light-red);
}
</style>
