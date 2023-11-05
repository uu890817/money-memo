<template>
	<div class="data-insert-wrap">
		<n-space justify="center">
			<div class="data-insert-box">
				交易日期:
				<n-date-picker v-model:value="insertData.date" type="date" placeholder="選擇日期" />
			</div>


			<div class="data-insert-box">
				交易類型:
				<n-select v-model:value="insertData.category" filterable :options="categoryOptions" placeholder="選擇類型"
					@click="getCategoryFromDB">
					<template #empty>
						<n-empty description="無數據" />
					</template>
				</n-select>
			</div>

			<div class="data-insert-box">
				交易明細:
				<n-select v-model:value="insertData.item" filterable :options="itemOptions" placeholder="選擇明細"
					@click="getItemFromDB">
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
				<n-select v-model:value="insertData.type" :options="payTypeOptions" placeholder="選擇交易種類">
					<template #empty>
						<n-empty description="無數據" />
					</template>
				</n-select>
			</div>

			<div class="data-insert-box">
				支付方式:
				<n-select v-model:value="insertData.payMethod" :options="paymentMethodOptions" placeholder="選擇支付方式"
					@click="getPaymentMethodFromDB">
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
</template>
    
<script setup lang='ts'>
import { NButton, NInput, NEmpty, NDatePicker, NSelect, NSpace, NInputNumber } from "naive-ui";
import { invoke } from "@tauri-apps/api/tauri";
import { PropType, Ref, onMounted, ref, watch } from "vue";


type InsertData = {
	date: number;
	category: string | null;
	item: string | null;
	type: string | null;
	cost: number;
	payMethod: string | null;
	note: string | null;
};

type WeekDate = {
	year: string,
	month: string,
	day: string,
	weekday: string,
	isThisMonth: boolean,
}
type Options = {
	label: string,
	value: string,
}
type CategoryItem = {
	label: string,
	value: string,
	category_id: string,
	category_name: string,
}
//-----------------------------------------------------------------------------------------------------------------------
const props = defineProps({
	selectDate: Object as PropType<WeekDate>,
});
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

const payTypeOptions: Ref<Options[]> = ref([
	{
		label: "🟢收入",
		value: "收入",
	},
	{
		label: "🟠支出",
		value: "支出",
	}
]);
const categoryOptions: Ref<Options[]> = ref([]);
const itemOptions: Ref<CategoryItem[]> = ref([]);
const paymentMethodOptions: Ref<Options[]> = ref([]);


const getCategoryFromDB = async () => {
	const result = JSON.parse(await invoke("select_all", { from: "Category" }));
	categoryOptions.value = [];
	for (let i of result) {
		categoryOptions.value.push({
			label: i.name,
			value: i.category_id,
		});
	}
};

const getItemFromDB = async () => {
	const result = JSON.parse(await invoke("select_all", { from: "Item" }));
	console.log(result);

	itemOptions.value = [];

	for (let i of result) {
		itemOptions.value.push({
			label: i.item_name,
			value: i.item_id,
			category_id: i.category_id,
			category_name: i.category_name,
		});
	}
};
const getPaymentMethodFromDB = async () => {
	const result = JSON.parse(await invoke("select_all", { from: "PaymentMethod" }));
	console.log(result);

	paymentMethodOptions.value = [];

	for (let i of result) {
		paymentMethodOptions.value.push({
			label: i.name,
			value: i.payment_method_id,
		});
	}
};


//-----------------------------------------------------------------------------------------------------------------------
onMounted(() => {
	getCategoryFromDB();
	getItemFromDB();
	getPaymentMethodFromDB();
});
watch(() => props.selectDate, (data) => {
	insertData.value.date = new Date(`${data?.year} ${data?.month} ${data?.day}`).getTime();
}, { deep: true });
watch(() => insertData.value.item, (data) => {
	let item = itemOptions.value.find((item) => item.value === data);
	insertData.value.category = item!.category_id;

});

</script>
    
<style>
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