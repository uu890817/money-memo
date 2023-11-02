<template>
    <div class="navbar-wrap">
        <n-space justify="end">
            <n-button strong secondary type="info">
                <template #icon>
                    <n-icon size="30">
                        <Settings />
                    </n-icon>
                </template>
            </n-button>
            <n-button strong secondary type="info">
                <template #icon>
                    <n-icon size="35">
                        <InformationCircleSharp />
                    </n-icon>
                </template>
            </n-button>
        </n-space>
    </div>
</template>
    
<script setup lang='ts'>
import { NSpace, NButton, NIcon } from "naive-ui";
import { Settings, InformationCircleSharp } from "@vicons/ionicons5";
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
</style>