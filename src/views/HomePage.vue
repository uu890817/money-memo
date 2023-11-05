<template>
	<div class="home-wrap">

		<SelectDate @selectDateUpdate="selectDateUpdate" />


		<n-divider dashed />

		<div class="data-table-wrap">
			<n-data-table :columns="dataTableColumns" :data="dataTableData" :pagination="{ pageSize: 5 }">
				<template #empty>
					<n-empty description="無數據" />
				</template>

			</n-data-table>
			<n-button type="info" dashed class="data-table-button">
				查看詳細數據
			</n-button>
		</div>

		<n-divider dashed>
			<span>新增資料</span>
		</n-divider>

		<InsertData :selectDate="selectDate" />

		<NavBar />
	</div>
</template>
    
<script setup lang='ts'>
import { NButton, NDivider, NDataTable, NEmpty, useMessage } from "naive-ui";
import NavBar from "@/components/home/NavBar.vue";
import SelectDate from "@/components/home/SelectDate.vue";
import InsertData from "@/components/home/InsertData.vue";

import { Ref, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type MemoData = {
	date: string,
	category: string,
	item: string,
	type: string,
	cost: string,
	payMethod: string,
	note: string
};

type WeekDate = {
	year: string,
	month: string,
	day: string,
	weekday: string,
	isThisMonth: boolean,
}
//-----------------------------------------------------------------------------------------------------------------------

const dataTableColumns = ref([
	{
		title: "日期",
		key: "date",
	},
	{
		title: "類型",
		key: "category",
	},
	{
		title: "交易明細",
		key: "item",
	},
	{
		title: "交易種類",
		key: "type",
	},
	{
		title: "交易金額",
		key: "cost",
	},
	{
		title: "交易方式",
		key: "payMethod",
	},
	{
		title: "備註",
		key: "note",
	}
]);

const dataTableData: Ref<MemoData[]> = ref([
	{
		date: "2022/01/01",
		category: "伙食費",
		item: "午餐",
		type: "支出",
		cost: "200",
		payMethod: "現金",
		note: "咖哩拌飯"
	},
	{
		date: "2022/01/01",
		category: "投資",
		item: "乙太幣",
		type: "收入",
		cost: "50",
		payMethod: "轉帳",
		note: "賺爛ㄌ"
	},
]);

const message = useMessage();
//-----------------------------------------------------------------------------------------------------------------------
const selectDate: Ref<WeekDate> = ref({
	year: new Date().getFullYear().toString(),
	month: new Date().getMonth().toString(),
	day: new Date().getDate().toString(),
	weekday: new Date().getDay().toString(),
	isThisMonth: false,
});
const selectDateUpdate = (data: WeekDate) => {

	selectDate.value = data;
};




onMounted(async () => {
	let connectionState = await invoke("create_connection");
	if (connectionState === true) {
		let tableState = await invoke("create_new_tables");
		if (tableState === true) {
			message.success("資料庫連結成功");
		}
	} else {
		message.error("資料庫連結失敗");
	}
});

</script>
    
<style scoped>
.home-wrap {
	width: 100vw;
	height: 100vh;
}



.data-table-wrap {
	position: relative;
	margin: auto;
	max-width: 90%;
	border-radius: 5px;
}

.data-table-button {
	position: absolute;
	margin-top: -40px;
	height: 28px;
}

.n-divider {
	margin: 15px 0;
}

:deep(.n-data-table th) {
	background: #2b2e3a;
	color: #ced6e3;
	font-size: 16px;
	font-weight: bold;
	text-align: center;
}

:deep(.n-data-table td) {
	background: #2b2b2c;
	color: #cccccc;
	text-align: center;

}

:deep(.n-data-table .n-data-table__pagination) {
	background: #ffffff00;
	color: #cccccc;
	text-align: center;
	padding: 0 10px 10px 0;
}

.data-insert-wrap {
	max-width: 90%;
	margin: auto;
}

.data-insert-box {
	width: 225px;
	margin: 5px;
}

.insert-button {
	width: 100%;
	margin: auto;
	margin-top: 10px;
}
</style>