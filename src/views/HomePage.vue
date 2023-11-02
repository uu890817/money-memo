<template>
	<div class="home-wrap">

		<div class="year-month-wrap">
			<div class="year">
				<n-button strong secondary @click="chooseDate.year = (parseInt(chooseDate.year) - 1).toString()">
					<template #icon>
						<n-icon>
							<ChevronBackCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<n-input type="text" :allow-input="onlyAllowYear" :maxlength="4" placeholder="請輸入年分"
					v-model:value="chooseDate.year" />
				<n-button strong secondary @click="chooseDate.year = (parseInt(chooseDate.year) + 1).toString()">
					<template #icon>
						<n-icon>
							<ChevronForwardCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<span>年</span>
			</div>

			<div class="month">
				<n-button strong secondary
					@click="chooseDate.month = chooseDate.month === '1' ? '12' : (parseInt(chooseDate.month) - 1).toString()">
					<template #icon>
						<n-icon>
							<ChevronBackCircleSharp />
						</n-icon>
					</template>
				</n-button>
				<n-input type="text" :allow-input="onlyAllowMonth" :maxlength="2" placeholder="請輸入月分"
					v-model:value="chooseDate.month" />
				<n-button strong secondary
					@click="chooseDate.month = chooseDate.month === '12' ? '1' : (parseInt(chooseDate.month) + 1).toString()">
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

			<div class="weekday-date" v-for="i in weekDate" :key="i.day">
				<p :style="(i.weekday === 'Sun' || i.weekday === 'Sat') ? 'color:#c25e5e;' : 'color:#bdc9dc;'">{{
					i.weekday }}</p>
				<n-button class="today-button" strong secondary circle type="error"
					v-if="i.isThisMonth && chooseDate.year === today.year && chooseDate.month === today.month && i.day === today.day"
					@click="log(i)">
					{{ i.day }}
				</n-button>
				<n-button strong secondary circle type="success" v-else-if="i.isThisMonth && i.year === today.year"
					@click="log(i)">
					{{ i.day }}
				</n-button>
				<n-button circle tertiary v-else @click="log(i)">
					{{ i.day }}
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
import { Ref, onMounted, ref } from "vue";

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
	date: number;
	category: string | null;
	item: string | null;
	type: string | null;
	cost: number;
	payMethod: string | null;
	note: string | null;
};
type Today = {
	timeStamp: number,
	year: string,
	month: string,
	day: string,
	weekday: string
}
type WeekDate = {
	year: string,
	month: string,
	day: string,
	weekday: string,
	isThisMonth: boolean,
}
//-----------------------------------------------------------------------------------------------------------------------
const log = (i) => {
	console.log(i);
}
const weekday = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const today: Ref<Today> = ref({
	timeStamp: new Date().getTime(),
	year: new Date().getFullYear().toString(),
	month: (new Date().getMonth() + 1).toString(),
	day: new Date().getDate().toString(),
	weekday: new Date().getDay().toString()
});
const weekDate: Ref<WeekDate[]> = ref([]);
const chooseDate: WeekDate = ref(
	{
		year: today.value.year,
		month: today.value.month,
		day: today.value.day,
		weekday: weekday[parseInt(today.value.weekday)],
		isThisMonth: true,
	}
);
const insertData: Ref<InsertData> = ref(
	{
		date: new Date().getTime(),
		category: null,
		item: null,
		type: null,
		cost: 0,
		payMethod: null,
		note: null
	}
);
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
//-----------------------------------------------------------------------------------------------------------------------
const getWeekDate = () => {
	let weekDate = [];


	for (let i = 0; i < 7; i++) {
		let newTimeStamp = today.value.timeStamp - ((1000 * 60 * 60 * 24) * (parseInt(today.value.weekday) - i));
		let newDate: WeekDate = {
			year: new Date(newTimeStamp).getFullYear().toString(),
			month: (new Date(newTimeStamp).getMonth() + 1).toString(),
			day: new Date(newTimeStamp).getDate().toString(),
			weekday: weekday[new Date(newTimeStamp).getDay()],
			isThisMonth: false,
		};
		newDate.isThisMonth = (newDate.year === today.value.year) && (newDate.month === today.value.month);
		weekDate.push(newDate);
	}
	return weekDate;

};


//-----------------------------------------------------------------------------------------------------------------------
const onlyAllowYear = (value: string) => !value || /^\d+$/.test(value);
const onlyAllowMonth = (value: string) => !value || /^(1[0-2]|[1-9])$/.test(value);

onMounted(() => {
	today.value = {
		timeStamp: new Date().getTime(),
		year: new Date().getFullYear().toString(),
		month: (new Date().getMonth() + 1).toString(),
		day: new Date().getDate().toString(),
		weekday: new Date().getDay().toString()
	};
	weekDate.value = getWeekDate();
});

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
	width: 225px;
	margin: 5px;
}

.insert-button {
	width: 100%;
	margin: auto;
	margin-top: 10px;
}
</style>