<template>
	<div class="insert-default-wrap">
		<p class="insert-default-title">建立預設資料</p>
		<div class="insert-default-class-wrap">
			<p class="insert-default-class-title">新增交易明細</p>
			<p class="insert-default-class-describe">交易明細是細分後的交易類型，例如:餐費中的『早餐、午餐、晚餐』、娛樂費的『看電影、課金』等等，未來可進行修改</p>

			<n-scrollbar style="max-height: 400px">
				<n-collapse v-for="category in categorys" :key="category.category_id">

					<n-collapse-item :title="category.name" :name="category.category_id">
						<n-dynamic-input style="padding-bottom: 10px;" v-model:value="value[category.category_id - 1]"
							placeholder="請輸入明細" :min="0" :max="200">
							<template #create-button-default>
								<p>按此添加新明細</p>
							</template>
						</n-dynamic-input>
						<template #header>
							<p style="padding: 10px;">{{ category.name }}</p>
						</template>
						<template #header-extra>
							目前明細筆數：{{ value[category.category_id - 1].length }} 筆
						</template>
					</n-collapse-item>
					<n-divider />

				</n-collapse>

			</n-scrollbar>
			<!-- <pre>{{ JSON.stringify(value, null, 2) }}</pre> -->

			<n-button type="success" ghost @click="insertItems">
				下一步
			</n-button>
		</div>

		<!-- <pre>{{ JSON.stringify(value, null, 2) }}</pre> -->
	</div>
</template>
    
<script setup lang='ts'>
import { NDynamicInput, NScrollbar, NButton, NCollapse, NCollapseItem, NDivider, useMessage } from "naive-ui";
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, onMounted, ref } from "vue";

type Categorys = {
	category_id: number;
	name: string;
}
const message = useMessage();
const emits = defineEmits(["gotoPayMethod"]);
const categorys: Ref<Categorys[]> = ref([]);
const value: Ref<string[][]> = ref([]);


const insertItems = async () => {
	let errorFlag = false;
	for (let i = 0; i < value.value.length; i++) {
		if (value.value[i].length !== 0) {
			console.log(value.value[i].length);
			for (let j of value.value[i]) {
				let result = await invoke("insert_data", { to: "Item", data: JSON.stringify({ "CategoryId": categorys.value[i].category_id, "Name": j }) });
				if (result) {
					message.success(`${categorys.value[i].name} : ${j} 新增成功`);
				} else {
					message.error(`${categorys.value[i].name} : ${j} 新增失敗`);
					errorFlag = true;
				}
			}
		}
	}
	if (errorFlag) {
		message.error("新增失敗");
		return;
	} else {
		emits("gotoPayMethod");
	}
};


onMounted(async () => {
	categorys.value = JSON.parse(await invoke("select_all", { from: "Category" }));
	for (let i = 0; i < categorys.value.length; i++) {
		value.value.push([]);
	}
});



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
	/* height: 50px; */
	margin: 30px 0;
}

.n-button:hover {
	background: #2d323c;
	transition: all 0.2s;
}

.n-collapse-item {
	padding: 0px 10px;
	border-radius: 5px;
	transition: all 0.2s;

}


.n-collapse-item:hover {
	background-color: #292d34;
	transition: all 0.2s;
}
</style>