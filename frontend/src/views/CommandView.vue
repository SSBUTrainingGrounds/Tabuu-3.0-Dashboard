<template>
    <div class="grid">
        <div class="table-header">
            <div>Rank</div>
            <div class="clickable" @click="sortTable(command, 'command', ascendingColumns)">Command Name</div>
            <div class="clickable" @click="sortTable(command, 'uses', ascendingColumns)">Uses</div>
            <div class="clickable" @click="sortTable(command, 'last_used', ascendingColumns)">Last Used</div>
        </div>
        <div class="content" v-for="(c, i) in command" :key="c">
            <div>
                <i>#{{ i + 1 }}</i>
            </div>
            <div>%{{ c["command"] }}</div>
            <div>{{ c["uses"] }}</div>
            <div>{{ c["last_used"] !== 0 ? new Date(c["last_used"] * 1000).toLocaleString() : "N/A" }}</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { sortTable } from "@/helpers/sortTable";
import { ref, onMounted } from "vue";

const command = ref([]);

const ascendingColumns = ref({
    uses: false,
    last_used: false,
    command: false
});

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/commands";

    const res = await fetch(url);
    command.value = await res.json();

    sortTable(command.value, "uses", ascendingColumns.value);
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr;
}
</style>
