import {createRouter, createWebHistory  } from "vue-router";

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{
			path: "/",
			name: "MainPage",
			component: () => import("@/views/MainPage.vue")
		},
		{
			path: "/setup",
			name: "SetupPage",
			component: () => import("@/views/SetupPage.vue")
		},
		{
			path: "/home",
			name: "HomePage",
			component: () => import("@/views/HomePage.vue")
		}
	]
});



export default router;