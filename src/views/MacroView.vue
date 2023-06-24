<template>
    <div class="macro">
        <div class="macro-input" v-if="isAdmin">
            <h2 class="new-header">Create New Macro</h2>
            <input type="text" class="macro-input-name" placeholder="Macro Name" id="macro-name" v-model="name" />
            <input type="text" class="macro-input-value" placeholder="Macro Value" id="macro-value" v-model="payload" />
            <button class="macro-button" @click="sendMacro">Send Macro</button>
        </div>

        <div class="macro-grid" :style="{ gridTemplateColumns: isAdmin ? '1fr 1fr 1fr 1fr 1fr' : '1fr 1fr 1fr 1fr' }">
            <h2 class="macro-column-1">Name</h2>
            <h2 class="macro-column-2">Value</h2>
            <h2 class="macro-column-3">Author</h2>
            <h2 class="macro-column-4">Uses</h2>
            <h2 class="macro-column-5" v-if="isAdmin">Delete</h2>
        </div>

        <div
            v-for="(macro, i) in allMacros"
            :key="i"
            class="macro-grid"
            :style="{ gridTemplateColumns: isAdmin ? '1fr 1fr 1fr 1fr 1fr' : '1fr 1fr 1fr 1fr' }"
        >
            <div class="macro-name macro-column-1">%{{ macro.name }}</div>
            <div class="macro-value macro-column-2">{{ macro.payload }}</div>
            <div class="macro-author macro-column-3">{{ macro.author }}</div>
            <div class="macro-uses macro-column-4">{{ macro.uses }}</div>
            <button
                class="macro-button macro-column-5"
                v-if="isAdmin"
                @click="
                    {
                        deleteMacro(macro.name);

                        allMacros.splice(i, 1);
                    }
                "
            >
                Delete
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, type Ref } from "vue";

const props = defineProps(["user", "isAdmin"]);

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
            author: props.user.value.id
        })
    });
    name.value = "";
    payload.value = "";
}

function deleteMacro(name: string) {
    fetch("http://localhost:8080/macro_delete", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            name: name
        })
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

.macro-grid,
.macro-input {
    display: grid;
    gap: 0.2rem 0.5rem;
    min-width: 600px;
    width: 1200px;
    max-width: 90%;
    margin: 5px auto;
    word-break: break-all;
}

.macro-column-1 {
    grid-column: 1;
}

.macro-column-2 {
    grid-column: 2;
}

.macro-column-3 {
    grid-column: 3;
}

.macro-column-4 {
    grid-column: 4;
}

.macro-column-5 {
    grid-column: 5;
}

.macro-name,
.macro-value,
.macro-input-name,
.macro-input-value {
    padding: 0.5rem;
    border-radius: 0.5rem;
    border: 1px solid #ccc;
    font-size: 1rem;
    background-color: var(--light-gray);
    color: var(--white);
}

.macro-name {
    justify-content: center;
    text-align: center;
    font-weight: bold;
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
</style>
