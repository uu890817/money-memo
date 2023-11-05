<template>
	<div class="year-month-wrap">
		<div class="year">
			<n-button strong secondary @click="changeYear('-')">
				<template #icon>
					<n-icon>
						<ChevronBackCircleSharp />
					</n-icon>
				</template>
			</n-button>
			<n-input type="text" :allow-input="onlyAllowYear" :maxlength="4" placeholder="請輸入年分"
				v-model:value="chooseDate.year" @input="changeYear('keydown')" />
			<n-button strong secondary @click="changeYear('+')">
				<template #icon>
					<n-icon>
						<ChevronForwardCircleSharp />
					</n-icon>
				</template>
			</n-button>
			<span>年</span>
		</div>

		<div class="month">
			<n-button strong secondary @click="changeMonth('-')">
				<template #icon>
					<n-icon>
						<ChevronBackCircleSharp />
					</n-icon>
				</template>
			</n-button>
			<n-input type="text" :allow-input="onlyAllowMonth" :maxlength="2" placeholder="請輸入月分"
				v-model:value="chooseDate.month" @input="changeYear('keydown')" />
			<n-button strong secondary @click=" changeMonth('+')">
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
		<n-button strong secondary @click="changeWeekDates('-')">
			<template #icon>
				<n-icon size="30">
					<ChevronBackCircleSharp />
				</n-icon>
			</template>
		</n-button>

		<div class="weekday-date" v-for=" weekDate  in  weekDates " :key="weekDate.day">
			<p :style="(weekDate.weekday === 'Sun' || weekDate.weekday === 'Sat') ? 'color:#c25e5e;' : 'color:#bdc9dc;'">
				{{
					weekDate.weekday }}</p>
			<n-button class="today-button" strong circle type="error"
				v-if="weekDate.isThisMonth && chooseDate.year === today.year && chooseDate.month === today.month && weekDate.day === today.day"
				@click="changeSelectDate(weekDate)"
				:ghost="weekDate.year === selectDate.year && weekDate.month === selectDate.month && weekDate.day === selectDate.day ? false : true">
				{{ weekDate.day }}
			</n-button>
			<n-button strong circle type="success" v-else-if="weekDate.isThisMonth && weekDate.year === chooseDate.year"
				@click="changeSelectDate(weekDate)"
				:ghost="weekDate.year === selectDate.year && weekDate.month === selectDate.month && weekDate.day === selectDate.day ? false : true">
				{{ weekDate.day }}
			</n-button>
			<!-- <n-button circle tertiary v-else @click="log(weekDate)">
					{{ weekDate.day }}
				</n-button> -->
		</div>

		<n-button strong secondary @click="changeWeekDates('+')">
			<template #icon>
				<n-icon size="30">
					<ChevronForwardCircleSharp />
				</n-icon>
			</template>
		</n-button>
	</div>
</template>
    
<script setup lang='ts'>
import { NButton, NInput, NIcon } from "naive-ui";
import { ChevronBackCircleSharp, ChevronForwardCircleSharp } from "@vicons/ionicons5";
import { Ref, onMounted, ref } from "vue";



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
const emits = defineEmits(["selectDateUpdate"]);
const weekday = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const today: Ref<Today> = ref({
	timeStamp: new Date().getTime(),
	year: new Date().getFullYear().toString(),
	month: (new Date().getMonth() + 1).toString(),
	day: new Date().getDate().toString(),
	weekday: new Date().getDay().toString()
});
const weekDates: Ref<WeekDate[]> = ref([]);
const chooseDate: Ref<WeekDate> = ref(
	{
		year: today.value.year,
		month: today.value.month,
		day: today.value.day,
		weekday: weekday[parseInt(today.value.weekday)],
		isThisMonth: true,
	}
);
const selectDate: Ref<WeekDate> = ref(
	{
		year: today.value.year,
		month: today.value.month,
		day: today.value.day,
		weekday: weekday[parseInt(today.value.weekday)],
		isThisMonth: true,
	}
);




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
const changeSelectDate = (date: WeekDate) => {
	selectDate.value.year = chooseDate.value.year;
	selectDate.value.month = chooseDate.value.month;
	selectDate.value.day = date.day;
	selectDate.value.weekday = date.weekday;
	selectDate.value.isThisMonth = date.isThisMonth;
	emits("selectDateUpdate", selectDate.value);
};
const changeYear = (type: string) => {
	if (type === "-") {
		chooseDate.value.year = (parseInt(chooseDate.value.year) - 1).toString();
		resetWeekDate();
	}
	if (type === "+") {
		chooseDate.value.year = (parseInt(chooseDate.value.year) + 1).toString();
		resetWeekDate();
	}
	if (type === "keydown") {
		resetWeekDate();
	}
};
const changeMonth = (type: string) => {
	if (type === "-") {
		chooseDate.value.month = chooseDate.value.month === "1" ? "12" : (parseInt(chooseDate.value.month) - 1).toString();
		resetWeekDate();
	}
	if (type === "only-") {
		chooseDate.value.month = chooseDate.value.month === "1" ? "12" : (parseInt(chooseDate.value.month) - 1).toString();
	}
	if (type === "+") {
		chooseDate.value.month = chooseDate.value.month === "12" ? "1" : (parseInt(chooseDate.value.month) + 1).toString();
		resetWeekDate();
	}
	if (type === "only+") {
		chooseDate.value.month = chooseDate.value.month === "12" ? "1" : (parseInt(chooseDate.value.month) + 1).toString();
	}
	if (type === "keydown") {
		resetWeekDate();
	}
};
const changeWeekDates = (type: string) => {
	if (type === "-") {
		if (!weekDates.value[0].isThisMonth) {
			changeMonth("only-");
			for (let i = 0; i < weekDates.value.length; i++) {
				weekDates.value[i].isThisMonth = !weekDates.value[i].isThisMonth;

			}
		} else {
			const firstOfWeekDate = new Date(`${weekDates.value[0].year}-${weekDates.value[0].month}-${weekDates.value[0].day}`);

			let weekDate = [];
			for (let i = 0; i < 7; i++) {
				let newTimeStamp = firstOfWeekDate.getTime() - ((1000 * 60 * 60 * 24) * (7 - i));
				let newDate: WeekDate = {
					year: new Date(newTimeStamp).getFullYear().toString(),
					month: (new Date(newTimeStamp).getMonth() + 1).toString(),
					day: new Date(newTimeStamp).getDate().toString(),
					weekday: weekday[new Date(newTimeStamp).getDay()],
					isThisMonth: false,
				};

				newDate.isThisMonth = (newDate.year === chooseDate.value.year) && (newDate.month === chooseDate.value.month);
				weekDate.push(newDate);
			}
			weekDates.value = weekDate;
		}
	}
	if (type === "+") {
		if (!weekDates.value[6].isThisMonth) {
			changeMonth("only+");
			for (let i = 0; i < weekDates.value.length; i++) {
				weekDates.value[i].isThisMonth = !weekDates.value[i].isThisMonth;

			}
		} else {
			const firstOfWeekDate = new Date(`${weekDates.value[6].year}-${weekDates.value[6].month}-${weekDates.value[6].day}`);

			let weekDate = [];
			for (let i = 0; i < 7; i++) {
				let newTimeStamp = firstOfWeekDate.getTime() + ((1000 * 60 * 60 * 24) * (i + 1));
				let newDate: WeekDate = {
					year: new Date(newTimeStamp).getFullYear().toString(),
					month: (new Date(newTimeStamp).getMonth() + 1).toString(),
					day: new Date(newTimeStamp).getDate().toString(),
					weekday: weekday[new Date(newTimeStamp).getDay()],
					isThisMonth: false,
				};

				newDate.isThisMonth = (newDate.year === chooseDate.value.year) && (newDate.month === chooseDate.value.month);
				weekDate.push(newDate);
			}
			weekDates.value = weekDate;
		}
	}
};
const resetWeekDate = () => {
	const firstOfMonth = new Date(`${chooseDate.value.year}-${chooseDate.value.month}-01`);
	let weekDate = [];
	for (let i = 0; i < 7; i++) {
		let newTimeStamp = firstOfMonth.getTime() - ((1000 * 60 * 60 * 24) * (firstOfMonth.getDay() - i));
		let newDate: WeekDate = {
			year: new Date(newTimeStamp).getFullYear().toString(),
			month: (new Date(newTimeStamp).getMonth() + 1).toString(),
			day: new Date(newTimeStamp).getDate().toString(),
			weekday: weekday[new Date(newTimeStamp).getDay()],
			isThisMonth: false,
		};

		newDate.isThisMonth = (newDate.year === firstOfMonth.getFullYear().toString()) && (newDate.month === (firstOfMonth.getMonth() + 1).toString());
		weekDate.push(newDate);
	}
	weekDates.value = weekDate;
};



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
	weekDates.value = getWeekDate();
});
</script>
    
<style>
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

.choose-date {
	border-radius: 5px;
	background-color: #3c3c3c;
}

.weekday-date {
	width: 40px;
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
</style>