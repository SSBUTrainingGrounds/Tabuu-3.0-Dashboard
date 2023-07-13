<template>
    <div class="grid">
        <div class="general-header"><i class="fa fa-window-restore"></i> OS Stats</div>
        <div class="general-stats">
            Uptime:
            {{ moment.duration(hwStats["uptime"] || "0", "seconds").humanize() }}
        </div>
        <div class="general-stats">OS: {{ hwStats["os_name"] || "Unknown OS" }}</div>

        <div class="cpu-header"><i class="fa fa-microchip"></i> CPU Stats</div>
        <div class="cpu-stats">Name: {{ hwStats["cpu_name"] || "Unknown CPU" }}</div>
        <div class="cpu-stats">Cores: {{ hwStats["cpu_cores"][0] || 0 }} ({{ hwStats["cpu_cores"][1] || 0 }})</div>
        <div class="cpu-stats">Frequency: {{ getAverage(hwStats["cpu_freq"]) }} MHz</div>
        <div class="cpu-stats">
            Usage:
            {{
                getAverage(hwStats["cpu_usage"]).toLocaleString("en", {
                    maximumFractionDigits: 2,
                    minimumFractionDigits: 2
                })
            }}%
        </div>
        <div class="cpu-stats">Temperature: {{ getAverage(hwStats["cpu_temp"]) }}°C</div>

        <div class="ram-header"><i class="fa fa-memory"> </i> Memory Stats</div>
        <div class="ram-stats">
            RAM Usage: {{ getGBString(hwStats["ram_used"]) || 0 }} / {{ getGBString(hwStats["ram_total"]) }} ({{
                hwStats["ram_percentage"].toLocaleString("en", {
                    style: "percent",
                    maximumFractionDigits: 2,
                    minimumFractionDigits: 2
                })
            }})
        </div>
        <div class="ram-stats">
            Swap Usage: {{ getGBString(hwStats["swap_used"]) || 0 }} / {{ getGBString(hwStats["swap_total"]) }} ({{
                hwStats["swap_percentage"].toLocaleString("en", {
                    style: "percent",
                    maximumFractionDigits: 2,
                    minimumFractionDigits: 2
                })
            }})
        </div>
        <div class="ram-stats" v-for="(disk, i) in hwStats['disks']" :key="i">
            Disk {{ i + 1 }} Usage: {{ getGBString(disk[2]) }} / {{ getGBString(disk[1]) }} ({{
                disk[4].toLocaleString("en", {
                    style: "percent",
                    maximumFractionDigits: 2,
                    minimumFractionDigits: 2
                })
            }})
        </div>
    </div>
</template>

<script setup lang="ts">
import type { HwStats } from "@/helpers/types";
import { onMounted, ref, type Ref } from "vue";
import { getAverage, getGBString } from "@/helpers/hardware";
import moment from "moment";

const hwStats: Ref<HwStats> = ref({
    uptime: 0,
    os_name: "",
    cpu_name: "",
    cpu_cores: [0, 0],
    cpu_freq: [],
    cpu_usage: [],
    cpu_temp: [],
    ram_total: 0,
    ram_used: 0,
    ram_free: 0,
    ram_percentage: 0,
    swap_total: 0,
    swap_used: 0,
    swap_free: 0,
    swap_percentage: 0,
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
}
</style>
