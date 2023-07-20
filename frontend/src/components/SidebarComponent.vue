<template>
    <div class="sidebar">
        <nav class="navbar" @click="$emit('changeVisibility')">
            <ul class="list">
                <RouterLink class="green-link" style="grid-row: 1" to="/">
                    <li><i class="fa fa-home"></i> Home</li>
                </RouterLink>

                <RouterLink class="green-link" style="grid-row: 2" to="/ranked/leaderboard">
                    <li><i class="fa fa-check"></i> Ranked Matchmaking</li>
                </RouterLink>

                <RouterLink class="green-link" style="grid-row: 3" to="/level">
                    <li><i class="fa fa-envelope"></i> Level</li>
                </RouterLink>

                <RouterLink class="green-link" style="grid-row: 4" to="/profiles">
                    <li><i class="fa fa-user"></i> Profiles</li>
                </RouterLink>

                <RouterLink class="green-link" style="grid-row: 5" to="/commands">
                    <li><i class="fa fa-signal"></i> Command Stats</li>
                </RouterLink>

                <RouterLink class="green-link" style="grid-row: 6" to="/macro">
                    <li><i class="fa fa-cog"></i> Macros</li>
                </RouterLink>

                <RouterLink class="green-link" style="grid-row: 7" to="/hardware">
                    <li><i class="fa fa-microchip"></i> Hardware</li>
                </RouterLink>
            </ul>

            <ul class="list">
                <a class="green-link" href="https://github.com/SSBUTrainingGrounds/Tabuu-3.0" target="_blank">
                    <li><i class="fab fa-github"></i> Tabuu 3.0 GitHub</li>
                </a>

                <a class="green-link" href="https://discord.gg/ssbutg" target="_blank">
                    <li><i class="fab fa-discord"></i> Join Our Discord</li>
                </a>

                <a class="green-link" href="https://github.com/SSBUTrainingGrounds/Tabuu-3.0-Dashboard" target="_blank">
                    <li><i class="fab fa-github"></i> Dashboard GitHub</li>
                </a>
            </ul>

            <ul class="footer-list">
                <RouterLink class="green-link small" to="/privacy"> Privacy Policy </RouterLink>
                |
                <a class="green-link small" href="mailto:ssbu.traininggrounds@gmail.com">Contact</a>
            </ul>
        </nav>
    </div>
</template>

<script setup lang="ts">
defineEmits<{
    changeVisibility: [];
}>();
</script>

<style scoped>
.sidebar {
    background-color: var(--gray);
    color: var(--white);
    border: 1px solid var(--black);
    padding: 1rem;
    position: fixed;
    height: calc(100% - 2rem);
    overflow: scroll;
    top: 2rem;
    bottom: 2rem;
    left: 0;
    width: 16rem;
    -ms-overflow-style: none;
    scrollbar-width: none;
}

.sidebar::-webkit-scrollbar {
    display: none;
}

.fa,
.fab {
    margin-right: 0.5rem;
}

.footer-list {
    list-style-type: none;
    margin-right: 0.5rem;
    padding: 1rem;
}

.small {
    font-size: small;
    border: none;
}
</style>
