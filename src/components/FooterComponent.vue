<template>
    <footer class="footer">
        <div class="wrapper-footer">
            <nav class="nav">
                <a href="https://github.com/SSBUTrainingGrounds/Tabuu-3.0"><i class="fab fa-github"></i> Bot GitHub</a>
                <a href="https://discord.gg/ssbutg"><i class="fab fa-discord"></i> Join Our Discord</a>
                <a href="https://github.com/atomflunder/dashboard-frontend"
                    ><i class="fab fa-github"></i> Dashboard GitHub</a
                >
            </nav>
        </div>
    </footer>
</template>

<script setup lang="ts"></script>

<style scoped>
.footer {
    background-color: var(--black);
    color: var(--white);
    bottom: 0;
    padding-top: 1rem;
    padding-bottom: 1rem;
    position: fixed;
    width: 100%;
    left: 0;
    right: 0;
    overflow: hidden hidden;
    -ms-overflow-style: none;
    scrollbar-width: none;
}

.footer::-webkit-scrollbar {
    display: none;
}

@media (max-width: 600px) {
    .footer .nav a {
        font-size: small;
    }
}

.wrapper-footer {
    max-width: 800px;
    margin: 0 auto;
}

.nav {
    display: flex;
    justify-content: space-between;
}

.nav a {
    color: var(--green);
    text-decoration: none;
    font-weight: bold;
    font-size: large;
    margin: 0 0.2rem 0 0.2rem;
}

.nav a:hover {
    color: var(--light-green);
}
</style>
