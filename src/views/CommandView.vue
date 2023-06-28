<template>
    <div class="grid">
        <div class="table-header">
            <div>Rank</div>
            <div>Command Name</div>
            <div>Uses</div>
            <div>Last Used</div>
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
// Get the user info from the express server
import { ref, onMounted } from "vue";

const command = ref([]);

onMounted(async () => {
    let url = new URL(window.location.href);
    url.port = "8080";
    url.pathname = "/commands";

    const res = await fetch(url);
    command.value = await res.json();
    command.value.sort((a, b) => b["uses"] - a["uses"]);
});
</script>

<style scoped>
.content,
.table-header {
    grid-template-columns: 1fr 1.2fr 1.5fr 1fr;
}
</style>
