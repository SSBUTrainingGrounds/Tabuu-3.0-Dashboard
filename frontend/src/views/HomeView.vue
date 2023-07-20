<template>
    <div class="home-div">
        <img class="avatar" src="../assets/avatar.png" alt="Tabuu 3.0 Avatar" width="150" />
        <div class="description">
            <h1>Tabuu 3.0 Dashboard</h1>

            <p>Welcome to the Tabuu 3.0 Dashboard! This is a work in progress.</p>
            <p>
                Please Log In with Discord and choose a page from the navigation bar.
                <br />
                If you are not logged in, or have not joined our server, you will not be able to access any pages.
            </p>
        </div>
    </div>
</template>

<style scoped>
@import "../assets/styles.css";

.home-div {
    color: var(--white);
    text-align: center;
}

.description,
.avatar {
    display: inline-block;
    margin: 0 auto;
    vertical-align: top;
}

.avatar {
    height: 150px;
    width: 150px;
}
</style>
