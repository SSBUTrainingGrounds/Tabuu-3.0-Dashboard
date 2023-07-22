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
            component: () => import("../views/RankedLeaderboardView.vue"),
            meta: { restricted: true },
            props: true
        },
        {
            path: "/ranked/matches",
            name: "ranked_matches",
            component: () => import("../views/RankedMatchesView.vue"),
            meta: { restricted: true },
            props: true
        },
        {
            path: "/level",
            name: "level",
            component: () => import("../views/LevelView.vue"),
            meta: { restricted: true },
            props: true
        },
        {
            path: "/commands",
            name: "commands",
            component: () => import("../views/CommandView.vue"),
            meta: { restricted: true }
        },
        {
            path: "/profiles",
            name: "profiles",
            component: () => import("../views/ProfileView.vue"),
            meta: { restricted: true },
            props: true
        },
        {
            path: "/macro",
            name: "macro",
            component: () => import("../views/MacroView.vue"),
            meta: { restricted: true },
            props: true
        },
        {
            path: "/hardware",
            name: "hardware",
            component: () => import("../views/HardwareInfoView.vue"),
            meta: { restricted: true }
        },
        {
            path: "/privacy",
            name: "privacy",
            component: () => import("../views/PrivacyPolicyView.vue")
        }
    ]
});

router.beforeEach(async (to, from, next) => {
    // If there is no token detected, redirect to login page to avoid making unnecessary requests.
    // If the user is not logged in or not on the server, the API will return a 401 error.
    if (to.meta.restricted) {
        if (!localStorage.getItem("discordToken")) {
            return next({ path: "/" });
        }
    }

    next();
});

export default router;
