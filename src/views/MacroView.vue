<template>
    <div class="macro-input" v-if="isAdmin">
        <h2 class="new-header">Create New Macro</h2>
        <input type="text" class="macro-input-name" placeholder="Macro Name" id="macro-name" v-model="name" />
        <input type="text" class="macro-input-value" placeholder="Macro Value" id="macro-value" v-model="payload" />
        <button class="macro-button" @click="sendMacro">Send Macro</button>
    </div>

    <div class="grid">
        <div class="table-header" :style="{ gridTemplateColumns: isAdmin ? '1fr 1fr 1fr 1fr 1fr' : '1fr 1fr 1fr 1fr' }">
            <div>Name</div>
            <div>Payload</div>
            <div>Author</div>
            <div>Uses</div>
        </div>

        <div
            v-for="(macro, i) in allMacros"
            :key="i"
            class="content"
            :style="{ gridTemplateColumns: isAdmin ? '1fr 1fr 1fr 1fr 1fr' : '1fr 1fr 1fr 1fr' }"
        >
            <div>%{{ macro.name }}</div>
            <div>{{ macro.payload }}</div>
            <div>{{ macro.author }}</div>
            <div>{{ macro.uses }}</div>
            <button class="delete-button" v-if="isAdmin" @click="deleteMacro(macro.name, i)">X</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from "vue";

const props = defineProps(["userID", "isAdmin"]);

let name = ref("");
let payload = ref("");

interface Macro {
    name: string;
    payload: string;
    author: string;
    uses: number;
}

let allMacros: Ref<Macro[]> = ref([]);

function sendMacro() {
    for (let i = 0; allMacros.value.length > i; i++) {
        if (allMacros.value[i].name === name.value) {
            alert("Macro name already exists!");
            return;
        }
    }
    fetch("http://localhost:8080/macro_new", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name.value,
            macro: payload.value,
            uses: 0,
            author: props.userID,
            discordToken: localStorage.getItem("discordToken")
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

    fetch("http://localhost:8080/macro_delete", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name,
            discordToken: localStorage.getItem("discordToken")
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
    const res = await fetch("http://localhost:8080/macro_get");
    allMacros.value = await res.json();
});
</script>

<style scoped>
@import "../assets/styles.css";

.new-header {
    grid-column: 1 / 6;
}

.macro-input {
    display: grid;
    gap: 0.2rem 0.5rem;
    min-width: 600px;
    width: 1200px;
    max-width: 90%;
    margin: 5px auto;
    word-break: break-all;
    background-color: var(--dark-gray);
    border-radius: 1rem;
    padding: 20px;
}

.macro-input-name,
.macro-input-value {
    padding: 0.5rem;
    border-radius: 0.5rem;
    border: 1px solid #ccc;
    font-size: 1rem;
    background-color: var(--light-gray);
    color: var(--white);
}

.macro-input-name {
    grid-column: 1;
}

.macro-input-value {
    grid-column: 2 / 4;
}

.macro-button {
    background-color: var(--light-gray);
    color: var(--white);
    padding: 0.8rem 1rem;
    margin: 1rem auto;
    border-radius: 5px;
    text-decoration: none;
    font-size: 1rem;
    font-weight: bold;
    transition: opacity 0.2s ease-in-out;
    grid-column: 5;
    align-self: start;
}

.macro-button:hover {
    opacity: 0.8;
}

.delete-button {
    background-color: var(--dark-gray);
    border: solid 1px var(--red);
    border-radius: 50%;
    font-size: 1rem;
    font-weight: bold;
    color: var(--red);
    height: 2.5rem;
    width: 2.5rem;
    margin: auto;
}

.delete-button:hover {
    background-color: var(--red);
    color: var(--light-red);
    border: solid 1px var(--light-red);
}
</style>
