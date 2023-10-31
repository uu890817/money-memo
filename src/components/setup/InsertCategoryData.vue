<template>
	<div class="insert-default-wrap">
		<p class="insert-default-title">建立預設資料</p>
		<div class="insert-default-class-wrap">
			<p class="insert-default-class-title">新增交易類別</p>
			<p class="insert-default-class-describe">交易類別是交易明細的分類名稱，例如:餐費、娛樂費、水電瓦斯費等等大類別，未來可進行修改</p>
			<n-scrollbar style="max-height: 400px">
				<n-dynamic-input v-model:value="value" placeholder="請輸入" :min="1" :max="200" />
			</n-scrollbar>

			<n-button type="success" ghost @click="insert">
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
const emits = defineEmits(["gotoItem"]);


const value = ref([
	"餐費",
	"娛樂費",
	"水電瓦斯費",
]);

const insert = async () => {
	let errorFlag = false;
	for (let i of value.value) {
		console.log({ Name: i });
		let result = await invoke("insert_data", { to: "Category", data: JSON.stringify({ Name: i }) });
		if (!result) {
			message.error(`類別${i}新增失敗!`, {
				icon: () => h(NIcon, null, { default: () => h(HourglassOutline) })
			});
			errorFlag = true;
		} else {
			message.warning(`類別${i}新增成功!`, {
				icon: () => h(NIcon, null, { default: () => h(HourglassOutline) })
			});
		}
	}
	if (!errorFlag) {
		message.success("類別資料新增完成!!!");
		emits("gotoItem");
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