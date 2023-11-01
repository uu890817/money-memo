<template>
	<div class="insert-default-wrap">
		<p class="insert-default-title">建立預設資料</p>
		<div class="insert-default-class-wrap">
			<p class="insert-default-class-title">新增交易方式</p>
			<p class="insert-default-class-describe">交易方式可以是現金交易、電子支付、轉帳等方式</p>
			<n-scrollbar style="max-height: 400px">
				<n-dynamic-input v-model:value="value" placeholder="請輸入交易方式" :min="0" :max="200" />
			</n-scrollbar>

			<n-button type="success" ghost @click="insertPayMethod">
				下一步
			</n-button>
		</div>

		<!-- <pre>{{ JSON.stringify(value, null, 2) }}</pre> -->
	</div>
</template>
    
<script setup lang='ts'>
import { NDynamicInput, NScrollbar, NButton, useMessage, NIcon } from "naive-ui";
import { h, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { HourglassOutline } from "@vicons/ionicons5";
const message = useMessage();
const emits = defineEmits(["gotoSuccess"]);

const value = ref([
	"現金",
	"Line Pay",

]);

const insertPayMethod = async () => {
	let errorFlag = false;
	for (let i of value.value) {
		console.log({ Name: i });
		let result = await invoke("insert_data", { to: "PaymentMethod", data: JSON.stringify({ Name: i }) });
		if (!result) {
			message.error(`交易方式"${i}"新增失敗!`, {
				icon: () => h(NIcon, null, { default: () => h(HourglassOutline) })
			});
			errorFlag = true;
		} else {
			message.warning(`交易方式"${i}"新增成功!`, {
				icon: () => h(NIcon, null, { default: () => h(HourglassOutline) })
			});
		}
	}
	if (!errorFlag) {
		message.success("類別資料新增完成!!!");
		emits("gotoSuccess");
	} else {
		message.error("類別資料新增失敗!");
	}


};



</script>
    
<style scoped>
.insert-default-wrap {
	padding: 0 20%;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	height: 100vh;
	max-height: 100vh;

}

.insert-default-title {
	position: fixed;
	top: 50px;
	font-size: 40px;
	text-align: center;
	padding-bottom: 50px;
}

.insert-default-class-wrap {
	position: fixed;
	top: 200px;
}

.insert-default-class-title {
	font-size: 30px;
	padding-bottom: 10px;
}

.insert-default-class-describe {
	font-size: 16px;
	padding-bottom: 20px;
}

.n-button {
	width: 100%;
	margin: 60px 0;
}

.n-button:hover {
	background: #2d323c;
	transition: all 0.2s;
}
</style>