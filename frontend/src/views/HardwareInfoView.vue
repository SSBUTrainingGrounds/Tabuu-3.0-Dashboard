<template>
    <div class="grid">
        <div class="general-header"><i class="fa fa-window-restore"></i> OS Stats</div>
        <div class="general-stats">
            <div class="name">OS</div>
            <div class="stat">
                {{ hwStats["os_name"] || "Unknown OS" }}
            </div>
        </div>
        <div class="general-stats">
            <div class="name">Uptime</div>
            <div class="stat">{{ moment.duration(hwStats["uptime"] || "0", "seconds").humanize() }}</div>
        </div>

        <div class="cpu-header"><i class="fa fa-microchip"></i> CPU Stats</div>
        <div class="cpu-stats">
            <div class="name">Name</div>
            <div class="stat">{{ hwStats["cpu_name"] || "Unknown CPU" }}</div>
        </div>
        <div class="cpu-stats">
            <div class="name">Cores / Threads</div>
            <div class="stat">{{ hwStats["cpu_cores"][0] || 0 }} / {{ hwStats["cpu_cores"][1] || 0 }}</div>
        </div>
        <div class="cpu-stats">
            <div class="name">Frequency</div>
            <div class="stat">{{ hwStats["avg_cpu_freq"] || 0 }} MHz</div>
        </div>
        <div class="cpu-stats">
            <div class="name">Usage</div>
            {{
                (hwStats["avg_cpu_usage"] || 0).toLocaleString("en", {
                    minimumFractionDigits: 2,
                    maximumFractionDigits: 2
                })
            }}%
        </div>
        <div class="cpu-stats">
            <div class="name">Temperature</div>
            <div class="stat">
                {{
                    (hwStats["avg_cpu_temp_c"] || 0).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}°C ({{
                    (hwStats["avg_cpu_temp_f"] || 0).toLocaleString("en", {
                        minimumFractionDigits: 2,
                        maximumFractionDigits: 2
                    })
                }}°F)
            </div>
        </div>

        <div class="ram-header"><i class="fa fa-memory"> </i> Memory Stats</div>
        <div class="ram-stats">
            <div class="name">RAM Usage</div>
            <div class="stat">
                {{ hwStats["ram_readable_str"] }}
            </div>
        </div>
        <div class="ram-stats">
            <div class="name">Swap Usage</div>
            <div class="stat">
                {{ hwStats["swap_readable_str"] }}
            </div>
        </div>
        <div class="ram-stats" v-for="(disk, i) in hwStats['disks']" :key="i">
            <div class="name">Disk {{ i + 1 }} Usage:</div>
            <div class="stat">
                {{ disk[5] }}
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { HwStats } from "@/helpers/types";
import { onMounted, ref, type Ref } from "vue";
import moment from "moment";

const hwStats: Ref<HwStats> = ref({
    uptime: 0,
    os_name: "",
    cpu_name: "",
    cpu_cores: [0, 0],
    cpu_freq: [],
    avg_cpu_freq: 0,
    cpu_usage: [],
    avg_cpu_usage: 0,
    cpu_temp_c: [],
    avg_cpu_temp_c: 0,
    cpu_temp_f: [],
    avg_cpu_temp_f: 0,
    ram_total: 0,
    ram_used: 0,
    ram_free: 0,
    ram_percentage: 0,
    ram_readable_str: "",
    swap_total: 0,
    swap_used: 0,
    swap_free: 0,
    swap_percentage: 0,
    swap_readable_str: "",
    disks: []
});

onMounted(async () => {
    let url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/hwinfo";

    const res = await fetch(url);
    hwStats.value = await res.json();
});
</script>

<style scoped>
.general-header,
.cpu-header,
.ram-header {
    font-size: 1.5rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
}

.general-stats,
.cpu-stats,
.ram-stats {
    margin-bottom: 0.5rem;

    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-column-gap: 1rem;
}

.name {
    grid-column: 1;
    font-weight: bold;
}

.stat {
    grid-column: 2;
}
</style>
