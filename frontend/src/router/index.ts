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
            path: "/trueskill",
            name: "trueskill",
            // route level code-splitting
            // this generates a separate chunk (About.[hash].js) for this route
            // which is lazy-loaded when the route is visited.
            component: () => import("../views/TrueSkillView.vue")
        },
        {
            path: "/leaderboard",
            name: "leaderboard",
            component: () => import("../views/LeaderboardView.vue")
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
