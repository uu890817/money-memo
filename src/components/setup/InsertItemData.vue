<template>
	<div class="insert-default-wrap">
		<p class="insert-default-title">建立預設資料</p>
		<div class="insert-default-class-wrap">
			<p class="insert-default-class-title">新增交易明細</p>
			<p class="insert-default-class-describe">交易類別是交易明細的分類名稱，例如:餐費、娛樂費、水電瓦斯費等等大類別</p>
			<n-scrollbar style="max-height: 400px">
				<n-dynamic-input v-model:value="value" placeholder="請輸入" :min="1" :max="200" />
			</n-scrollbar>
			<n-space justify="space-between">
				<n-button type="info" ghost>
					上一步
				</n-button>
				<n-button type="success" ghost>
					下一步
				</n-button>
			</n-space>
		</div>

		<!-- <pre>{{ JSON.stringify(value, null, 2) }}</pre> -->
	</div>
</template>
    
<script setup lang='ts'>
import { NDynamicInput, NScrollbar, NButton, NSpace } from "naive-ui";
import { ref } from "vue";

let value = ref([
	"餐費",
	"娛樂費",
	"水電瓦斯費",
]);
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
	width: 200px;
	margin: 20px 40px;
}
</style>