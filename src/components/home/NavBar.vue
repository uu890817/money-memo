<template>
	<div class="navbar-wrap">

		<n-popover trigger="hover" :show-arrow="false">
			<template #trigger>
				<n-button class="reload-button" strong secondary type="info" @click="reload">
					<template #icon>
						<n-icon size="35">
							<ReloadCircleSharp />
						</n-icon>
					</template>
				</n-button>
			</template>
			<span>重新整理</span>
		</n-popover>

		<n-popover trigger="hover" :show-arrow="false">
			<template #trigger>
				<n-button class="setting-button" strong secondary type="info">
					<template #icon>
						<n-icon size="30">
							<Settings />
						</n-icon>
					</template>
				</n-button>
			</template>
			<span>設定</span>
		</n-popover>

		<n-popover trigger="hover" :show-arrow="false">
			<template #trigger>
				<n-button class="info-button" strong secondary type="info">
					<template #icon>
						<n-icon size="35">
							<InformationCircleSharp />
						</n-icon>
					</template>
				</n-button>
			</template>
			<span>軟體資訊</span>
		</n-popover>

	</div>
</template>
    
<script setup lang='ts'>
import { NButton, NIcon, NPopover } from "naive-ui";
import { Settings, InformationCircleSharp, ReloadCircleSharp } from "@vicons/ionicons5";


const reload = () => {
	window.location.reload();
};





</script>
    
<style scoped>
.navbar-wrap {
	width: 100%;
	height: 50px;
	border-top: 2px solid #3c415a;
	position: fixed;
	bottom: 0;
}

.n-space {
	margin-right: 5px;
}

.n-button {
	margin: 5px 0px 5px 0;
	width: 40px;
	height: 40px
}

.reload-button {
	position: fixed;
	left: 0;
	margin-left: 5px
}

.setting-button {
	position: fixed;
	right: 50px;
	margin-right: 5px;

}

.info-button {
	position: fixed;
	right: 0;
	margin-right: 5px;
}
</style>