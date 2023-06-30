import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: HomeView
        },
        {
            path: "/ranked/leaderboard",
            name: "ranked_leaderboard",
            // route level code-splitting
            // this generates a separate chunk (About.[hash].js) for this route
            // which is lazy-loaded when the route is visited.
            component: () => import("../views/RankedLeaderboardView.vue")
        },
        {
            path: "/ranked/matches",
            name: "ranked_matches",
            component: () => import("../views/RankedMatchesView.vue")
        },
        {
            path: "/level",
            name: "level",
            component: () => import("../views/LevelView.vue")
        },
        {
            path: "/commands",
            name: "commands",
            component: () => import("../views/CommandView.vue")
        },
        {
            path: "/profiles",
            name: "profiles",
            component: () => import("../views/ProfileView.vue")
        },
        {
            path: "/macro",
            name: "macro",
            component: () => import("../views/MacroView.vue"),
            props: true
        }
    ]
});

export default router;
