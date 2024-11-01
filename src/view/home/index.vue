<template>
    <el-tabs v-model="activeName" class="demo-tabs">
    <el-tab-pane label="控制台" name="control">
        <control></control>
    </el-tab-pane>
    <el-tab-pane label="虚拟机列表" name="vm">
        <vm></vm>
    </el-tab-pane>
    <el-tab-pane label="脚本配置" name="config">
        <config></config>
    </el-tab-pane>
    <el-tab-pane label="系统配置" name="setting">
        <setting></setting>
    </el-tab-pane>
    <el-tab-pane label="ID列表" name="aid">
        <aid></aid>
    </el-tab-pane>
    <el-tab-pane label="序列号列表" name="serial">
        <serial></serial>
    </el-tab-pane>
</el-tabs>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import control from './component/control.vue'
import vm from './component/vm.vue'
import config from './component/config.vue'
import setting from './component/setting.vue'
import aid from './component/aid.vue'
import serial from './component/serial.vue'
import type { TabsPaneContext } from 'element-plus'

const activeName = ref('control')


</script>
<style></style>