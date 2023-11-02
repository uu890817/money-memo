<template>
	<div class="home-wrap">

		<div class="year-month-wrap">
			<div class="year">
				<n-button strong secondary>
					<template #icon>
						<n-icon>
							<ChevronBackCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<n-input type="text" :allow-input="onlyAllowYear" :maxlength="4" placeholder="請輸入年分" />
				<n-button strong secondary>
					<template #icon>
						<n-icon>
							<ChevronForwardCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<span>年</span>
			</div>

			<div class="month">
				<n-button strong secondary>
					<template #icon>
						<n-icon>
							<ChevronBackCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<n-input type="text" :allow-input="onlyAllowMonth" :maxlength="2" placeholder="請輸入月分" />
				<n-button strong secondary>
					<template #icon>
						<n-icon>
							<ChevronForwardCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<span>月</span>
			</div>
		</div>

		<div class="date-wrap">
			<n-button strong secondary>
				<template #icon>
					<n-icon>
						<ChevronBackCircleSharp />
					</n-icon>
				</template>
			</n-button>

			<div class="weekday-date" v-for="(i, index) in weekday" :key="i">
				<p :style="(i === 'Sun' || i === 'Sat') ? 'color:#c25e5e;' : 'color:#bdc9dc;'">{{ i }}</p>
				<n-button strong secondary circle type="success">
					{{ index + 1 }}
				</n-button>
			</div>
			<n-button strong secondary>
				<template #icon>
					<n-icon>
						<ChevronForwardCircleSharp />
					</n-icon>
				</template>
			</n-button>
		</div>

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

		<div class="data-insert-wrap">
			<n-space justify="center">
				<div class="data-insert-box">
					交易日期:
					<n-date-picker v-model:value="insertData.date" type="date" placeholder="選擇日期" />
				</div>


				<div class="data-insert-box">
					交易類型:
					<n-select v-model:value="insertData.category" :options="options" placeholder="選擇類型">
						<template #empty>
							<n-empty description="無數據" />
						</template>
					</n-select>
				</div>

				<div class="data-insert-box">
					交易明細:
					<n-select v-model:value="insertData.item" :options="options" placeholder="選擇明細">
						<template #empty>
							<n-empty description="無數據" />
						</template>
					</n-select>
				</div>


				<div class="data-insert-box">
					交易金額:
					<n-input-number v-model:value="insertData.cost" placeholder="輸入價錢">
						<template #prefix>
							$
						</template>
					</n-input-number>
				</div>

				<div class="data-insert-box">
					支出/收入:
					<n-select v-model:value="insertData.type" :options="options" placeholder="選擇交易種類">
						<template #empty>
							<n-empty description="無數據" />
						</template>
					</n-select>
				</div>

				<div class="data-insert-box">
					支付方式:
					<n-select v-model:value="insertData.payMethod" :options="options" placeholder="選擇支付方式">
						<template #empty>
							<n-empty description="無數據" />
						</template>
					</n-select>
				</div>

				<div class="data-insert-box">
					備註:
					<n-input v-model:value="insertData.note" type="text" placeholder="輸入備註" />
				</div>


			</n-space>
			<n-button class="insert-button" strong secondary type="success">
				送出
			</n-button>
		</div>

		<NavBar />
	</div>
</template>
    
<script setup lang='ts'>
import { NButton, NInput, NIcon, NDivider, NDataTable, NEmpty, NDatePicker, NSelect, NSpace, NInputNumber } from "naive-ui";
import { ChevronBackCircleSharp, ChevronForwardCircleSharp } from "@vicons/ionicons5";
import NavBar from "@/components/home/NavBar.vue";
import { Ref, ref } from "vue";

type MemoData = {
	date: string,
	category: string,
	item: string,
	type: string,
	cost: string,
	payMethod: string,
	note: string
};
type InsertData = {
	date: number,
	category: string,
	item: string,
	type: string,
	cost: number,
	payMethod: string,
	note: string
};


const insertData: Ref<InsertData> = ref(
	{
		date: 10000,
		category: "",
		item: "",
		type: "",
		cost: 0,
		payMethod: "",
		note: ""
	}
);
const weekday = ref(["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]);
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
	{
		date: "2022/01/01",
		category: "投資",
		item: "乙太幣",
		type: "收入",
		cost: "50",
		payMethod: "轉帳",
		note: "賺爛ㄌ"
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
	{
		date: "2022/01/01",
		category: "投資",
		item: "乙太幣",
		type: "收入",
		cost: "50",
		payMethod: "轉帳",
		note: "賺爛ㄌ"
	},
	{
		date: "2022/01/01",
		category: "投資",
		item: "乙太幣",
		type: "收入",
		cost: "50",
		payMethod: "轉帳",
		note: "賺爛ㄌ"
	}
]);
const onlyAllowYear = (value: string) => !value || /^\d+$/.test(value);
const onlyAllowMonth = (value: string) => !value || /^(1[0-2]|[1-9])$/.test(value);



</script>
    
<style scoped>
.home-wrap {
	width: 100vw;
	height: 100vh;
}

.year-month-wrap {
	display: flex;
	justify-content: center;
}



.year,
.month {
	display: flex;
	align-items: center;
	padding: 10px 100px;
	height: 30px;
	max-width: 50%;
}

.year>*,
.month>* {
	margin: 0px 5px;
	height: 30px;
	font-size: 20px;
}

.date-wrap {
	display: flex;
	justify-content: center;
	height: 60px;
	margin-top: 10px;
}

.date-wrap>.n-button {
	height: 60px;
	margin: 0 30px;

}

.weekday-date {
	margin: 0 30px;
}

.weekday-date>p {
	text-align: center;
}

.weekday-date>.n-button {
	text-align: center;
	width: 40px;
	height: 40px;
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
	width: 200px;
	margin: 5px;
}

.insert-button {
	width: 100%;
	margin: auto;
	margin-top: 10px;
}
</style>