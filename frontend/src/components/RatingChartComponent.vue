<template>
    <Line :data="chartData" :options="chartOptions" class="chart" />
</template>

<script setup lang="ts">
import { Chart as ChartJS, registerables } from "chart.js";
import { ref, type PropType } from "vue";
import { Line } from "vue-chartjs";

ChartJS.register(...registerables);

const props = defineProps({
    ratings: {
        type: Array as PropType<number[]>,
        required: true
    }
});

const displayRating = JSON.parse(JSON.stringify(props.ratings));
displayRating.reverse();

const labels = Array(displayRating.length).fill("");

const chartData = ref({
    labels: labels,
    datasets: [
        {
            label: "Ratings",
            data: displayRating,
            borderColor: "#53e6a4",
            tension: 0.1,
            pointHoverRadius: 5
        }
    ]
});

const chartOptions = ref({
    plugins: {
        legend: {
            display: false
        }
    }
});
</script>

<style scoped>
.chart {
    margin-top: 1rem;
}
</style>
