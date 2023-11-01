<template>
	<InsertCategoryData @gotoItem="gotoItem" v-if="componentsSept === 'InsertCategoryData'" />
	<InsertItemData @gotoPayMethod="gotoPayMethod" v-else-if="componentsSept === 'InsertItemData'" />
	<InsertPayMethodData @gotoSuccess="gotoSuccess" v-else-if="componentsSept === 'InsertPayMethod'" />
	<SetupSuccess v-else-if="componentsSept === 'SetupSuccess'" />

	<div class="setup-wrap animate__animated animate__zoomIn" v-else>
		<img class="setup-logo" src="../assets/MoneyMemo-Long.svg">

		<p class="setup-text">沒有發現資料庫，請問要新增新資料庫還是導入已有資料庫 ?</p>

		<div class="setup-button">
			<n-button class="setup-button__importDataBase" type="info" ghost @click="importDB">
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
import InsertCategoryData from "@/components/setup/InsertCategoryData.vue";
import InsertItemData from "@/components/setup/InsertItemData.vue";
import InsertPayMethodData from "@/components/setup/InsertPayMethodData.vue";
import SetupSuccess from "@/components/setup/SetupSuccess.vue";

import { NButton, NIcon, useMessage } from "naive-ui";
import { ArrowRedoCircleSharp, AddCircleSharp } from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const message = useMessage();
const componentsSept = ref("default");

const gotoItem = () => {
	componentsSept.value = "InsertItemData";
};
const gotoPayMethod = () => {
	componentsSept.value = "InsertPayMethod";
};
const gotoSuccess = () => {
	componentsSept.value = "SetupSuccess";
};

const importDB = async () => {
	message.error("導入功能製作中");
};

const newDB = async () => {
	let connectionState = await invoke("create_connection");
	if (connectionState === true) {
		let tableState = await invoke("create_new_tables");
		if (tableState === true) {
			message.success("資料庫建立成功");
			componentsSept.value = "InsertCategoryData";
		}
	} else {
		message.error("資料庫建立失敗");
	}
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
	max-width: 40%;
}

.setup-text {
	font-size: 20px;
	margin-bottom: 100px;
}

.n-button {
	margin-left: 50px;
	margin-right: 50px;
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
