<!--
 * @Date: 2024-11-01 13:00:41
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-01 15:37:32
 * @FilePath: /apple-control/src/view/home/component/control.vue
 * @Description: 
-->
<template>
  <div class="container">
    <div class="row">
      <el-button type="primary" @click="handleStartCommand" plain
        >启动控制台</el-button
      >
      <el-button type="danger" @click="handleStopCommand" plain :disabled="stopBtn"
        >停止控制台</el-button
      >
      <el-button type="warning" @click="handleClearInvalidVm" plain
        >清理失效虚拟机</el-button
      >
      <el-button type="info" @click="handleClearConsole" plain
        >清空控制台日志</el-button
      >
    </div>
    <div class="">
      <el-row :gutter="10">
        <el-col :span="12">
          <el-divider content-position="left">虚拟机数量统计</el-divider>
          <el-row>
            <el-col :span="6">
              <el-statistic title="总数" :value="268500" />
            </el-col>
            <el-col :span="6">
              <el-statistic title="可用" :value="138"> </el-statistic>
            </el-col>
            <el-col :span="6">
              <el-statistic title="静置中" :value="3" />
            </el-col>
            <el-col :span="6">
              <el-statistic title="已失效" :value="562"></el-statistic>
            </el-col>
          </el-row>
        </el-col>
        <el-col :span="12">
          <el-divider content-position="left">运行实例状态</el-divider>
          <el-row>
            <el-col :span="8">
              <el-statistic title="开机实例数" :value="268500" />
            </el-col>

            <el-col :span="8">
              <el-statistic title="执行任务中" :value="1212" />
            </el-col>
            <el-col :span="8">
              <el-statistic title="初始化中" :value="562"> </el-statistic>
            </el-col>
          </el-row>
        </el-col>
      </el-row>
    </div>
    <div class="row">
      <div class="console">
        <p v-for="(item, i) in log.logs" :key="i" :style="{ color: item.color }">
          <span>{{ item.time }}</span
          ><span>{{ item.message }}</span>
        </p>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, getCurrentInstance } from "vue";
import useStore from "@/store";
const { proxy } = getCurrentInstance();
const { log, app } = useStore()

const stopBtn = ref(true);
// addLog 新增日志记录
const addLog = (message: string, color: string = "#333") => {
  log.addLog(message, color);
};

// 启动控制台
const handleStartCommand = () => {
  proxy.$message.success("启动控制台");
  addLog("启动控制台", "green");
};
//handleStopCommand  停止控制台
const handleStopCommand = () => {
  proxy.$message.error("停止控制台");
  addLog("停止控制台", "red");
};

// handleClearInvalidVm 清理失效虚拟机
const handleClearInvalidVm = () => {
  proxy.$message.error("清理失效虚拟机");
  addLog("清理失效虚拟机", "#eaeaea");
};
// handleClearConsole 清空控制台日志
const handleClearConsole = () => {
  proxy.$message.info("清空控制台日志");
  log.clearLog()
};
</script>
<style scoped>
.console {
  width: 100%;
  overflow-y: auto;
  background-color: #eee;
  color: #333;
  flex-direction: column;
  text-align: left;
  height: 300px;
  overflow-y: auto;
  margin-top: 10px;
}
.console p {
  padding-left: 10px;
  line-height: 10px;
}
</style>
