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
    if (to.meta.restricted) {
        if (localStorage.getItem("discordToken")) {
            const server_url = new URL(import.meta.env.VITE_API_URL);
            server_url.port = import.meta.env.VITE_API_PORT;
            server_url.pathname = "/api/is_on_server";

            await fetch(server_url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    discord_token: localStorage.getItem("discordToken")
                })
            }).then((res) => {
                // If the user is not on the server, redirect them to the home page.
                if (res.status !== 202) {
                    return next({ path: "/" });
                }
            });
        } else {
            // If the user is not logged in, redirect them to the home page.
            return next({ path: "/" });
        }
    }

    next();
});

export default router;
