<template>
	<div class="container">
		<router-view />
	</div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import SetupPage from "@/views/SetupPage.vue";
import { invoke } from "@tauri-apps/api";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const bool = ref(false);

onMounted(() => {
	console.log("onMounted");
	const is_database_exist = async () => {
		bool.value = await invoke("is_database_exist");
		console.log(bool.value);
		if (bool.value) {
			console.log(bool.value);
			router.replace("/home");
		} else {
			console.log(bool.value);
			router.replace("/setup");
		}
	};
	is_database_exist();
});

</script>

<style scoped></style>
