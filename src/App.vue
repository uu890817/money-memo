<template>
	<n-config-provider :theme="darkTheme">
		<n-global-style />
		<n-message-provider>
			<div class="container">
				<router-view />
			</div>
		</n-message-provider>
	</n-config-provider>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import SetupPage from "@/views/SetupPage.vue";
import { invoke } from "@tauri-apps/api";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { NConfigProvider, darkTheme, NGlobalStyle, NMessageProvider } from "naive-ui";



const router = useRouter();
const bool = ref(false);


window.addEventListener("keydown", function (e) {
	const F_KEY = ["F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];
	for (let key of F_KEY) {
		if (e.key === key) {
			e.preventDefault(); // 防止F1-F12按鍵的預設行為
		}

	}

	if ((e.ctrlKey || e.metaKey) && (e.key === "C" || e.key === "c")) {
		e.preventDefault(); // 防止Ctrl+C的預設行為
	} else if ((e.ctrlKey || e.metaKey) && (e.key === "V" || e.key === "v")) {
		e.preventDefault(); // 防止Ctrl+V的預設行為
	}
	if ((e.ctrlKey || e.metaKey) && (e.key === "f" || e.key === "F")) {
		e.preventDefault(); // 防止CTRL+F的預設行為
	}
	if ((e.ctrlKey || e.metaKey) && (e.key === "g" || e.key === "G")) {
		e.preventDefault(); // 防止CTRL+G的預設行為
	}
	if ((e.ctrlKey || e.metaKey) && (e.key === "u" || e.key === "U")) {
		e.preventDefault(); // 防止CTRL+G的預設行為
	}
	if ((e.ctrlKey || e.metaKey) && (e.key === "p" || e.key === "P")) {
		e.preventDefault(); // 防止CTRL+G的預設行為
	}
});
// window.addEventListener("contextmenu", function (e) {
// 	e.preventDefault(); // 防止右鍵菜單的預設顯示
// });

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

<style >
* {
	-moz-user-select: none;
	/* Firefox */
	-webkit-user-select: none;
	/* Chrome , Safari*/
	-ms-user-select: none;
	/* IE10 */
	-khtml-user-select: none;
	/* 古早瀏覽器 */
	user-select: none;

}

.n-config-provider {
	font-family: arial, "Microsoft JhengHei", "微軟正黑體", sans-serif !important;
	background-color: #1d2128;
	color: #bdc9dc;
}
</style>
