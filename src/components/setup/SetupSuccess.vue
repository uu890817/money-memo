<template>
	<div class="success-wrap">
		<n-result status="success" size="large" title="初始化完成" description="">
			<template #footer>
				<n-button @click="startMemo">開始記帳吧</n-button>
			</template>
		</n-result>
	</div>
</template>

    
<script setup lang='ts'>
import { NResult, NButton } from "naive-ui";
import { useRouter } from "vue-router";

const router = useRouter();
const startMemo = () => {
	router.replace("/home");
};


</script>
    
<style scoped>
.success-wrap {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	height: 100vh;
}

.n-button {
	width: 100%;
	height: 50px;
	margin: 60px 0;
}

.n-button:hover {
	background: #2d323c;
	transition: all 0.2s;
}
</style>