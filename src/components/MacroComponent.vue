<template>
    <div class="macro">
        <h2>Macros</h2>
        <div class="macro-input">
            <input type="text" class="macro-name" placeholder="Macro Name" id="macro-name" v-model="name" />
            <input type="text" class="macro-value" placeholder="Macro Value" id="macro-value" v-model="payload" />
            <button
                class="macro-button"
                @click="
                    {
                        $emit('sendMacro', name, payload);
                        name = '';
                        payload = '';
                    }
                "
            >
                Send Macro
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

let name = ref("");
let payload = ref("");

defineEmits(["sendMacro"]);
</script>

<style scoped>
@import "../assets/styles.css";

.macro-name,
.macro-value {
    padding: 0.5rem;
    border-radius: 0.5rem;
    border: 1px solid #ccc;
    font-size: 1rem;
    background-color: var(--light-gray);
    color: var(--white);
}

.macro-name {
    width: 20%;
    margin-right: 1%;
}

.macro-value {
    width: 20%;
    margin-right: 1%;
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
}

.macro-button:hover {
    opacity: 0.8;
}
</style>
