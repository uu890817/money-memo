<template>
	<div class="setup-wrap">
		<img class="setup-logo" src="../assets/MoneyMemo-Long.svg">

		<p class="setup-text">沒有發現資料庫，請問要新增新資料庫還是導入已有資料庫 ?</p>

		<div class="setup-button">
			<n-button class="setup-button__importDataBase" type="info" ghost>
				<n-icon>
					<ArrowRedoCircleSharp />
				</n-icon>
				導入已有資料庫
			</n-button>
			<n-button class="setup-button__newDataBase" type="success" ghost @click="newDB">
				<n-icon>
					<AddCircleSharp />
				</n-icon>
				新增新資料庫
			</n-button>
		</div>

	</div>
</template>
  
<script setup lang="ts">
import { NButton, NIcon } from "naive-ui";
import { ArrowRedoCircleSharp, AddCircleSharp } from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/tauri";

const newDB = async () => {
	await invoke("create_connection");
};


</script>
<style scoped>
.setup-wrap {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	height: 100vh;
}

.setup-logo {
	position: absolute;
	top: 10%;
	max-width: 30%;
}

.setup-text {
	font-size: 20px;
}

.n-button {
	margin: 30px;
	width: 300px;
	height: 100px;
	font-size: 20px;
	line-height: 1.5;

}

.n-button:hover {
	background: #2d323c;
}

.n-icon>svg {
	vertical-align: top;
}
</style>
