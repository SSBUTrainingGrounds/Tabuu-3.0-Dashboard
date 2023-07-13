<template>
    <div class="sidebar">
        <nav class="navbar" @click="$emit('change-visibility')">
            <ul>
                <li>
                    <RouterLink class="green-link" style="grid-row: 1" to="/"
                        ><i class="fa fa-home"></i> Home</RouterLink
                    >
                </li>

                <li>
                    <RouterLink class="green-link" style="grid-row: 2" to="/ranked/leaderboard"
                        ><i class="fa fa-check"></i> Ranked Matchmaking</RouterLink
                    >
                </li>
                <li>
                    <RouterLink class="green-link" style="grid-row: 3" to="/level"
                        ><i class="fa fa-envelope"></i> Level</RouterLink
                    >
                </li>
                <li>
                    <RouterLink class="green-link" style="grid-row: 4" to="/profiles"
                        ><i class="fa fa-user"></i> Profiles</RouterLink
                    >
                </li>
                <li>
                    <RouterLink class="green-link" style="grid-row: 5" to="/commands"
                        ><i class="fa fa-signal"></i> Command Stats</RouterLink
                    >
                </li>
                <li>
                    <RouterLink class="green-link" style="grid-row: 6" to="/macro"
                        ><i class="fa fa-cog"></i> Macros</RouterLink
                    >
                </li>
                <li>
                    <RouterLink class="green-link" style="grid-row: 7" to="/hardware"
                        ><i class="fa fa-microchip"></i> Hardware</RouterLink
                    >
                </li>
            </ul>

            <ul>
                <li>
                    <a class="green-link" href="https://github.com/SSBUTrainingGrounds/Tabuu-3.0" target="_blank"
                        ><i class="fab fa-github"></i> Tabuu 3.0 GitHub</a
                    >
                </li>
                <li>
                    <a class="green-link" href="https://discord.gg/ssbutg" target="_blank"
                        ><i class="fab fa-discord"></i> Join Our Discord</a
                    >
                </li>
                <li>
                    <a class="green-link" href="https://github.com/atomflunder/bot-dashboard" target="_blank"
                        ><i class="fab fa-github"></i> Dashboard GitHub</a
                    >
                </li>
            </ul>
        </nav>
    </div>
</template>

<script setup lang="ts">
defineEmits(["change-visibility"]);
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

.green-link {
    color: var(--green);
    text-decoration: none;
    font-weight: bold;
    font-size: large;
    margin: 0 0.2rem 0 0.2rem;
}

.green-link:hover {
    color: var(--light-green);
}

ul {
    list-style-type: none;
    padding: 0.5rem;
}

li {
    padding: 0.5rem;
}

li:hover {
    background-color: var(--black);
    color: var(--green);
    border-radius: 0.5rem;
}

li:hover .fa,
li:hover .fab {
    color: var(--light-green);
}

li:hover a {
    cursor: pointer;
}

.fa,
.fab {
    margin-right: 0.5rem;
}
</style>
