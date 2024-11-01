<!--
 * @Date: 2024-11-01 13:00:41
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-01 15:15:48
 * @FilePath: /apple-control/src/view/home/component/control.vue
 * @Description: 
-->
<template>
    <div class="container">
        <div class="row">
            <el-button type="primary" @click="handleStartCommand" plain>启动控制台</el-button>
            <el-button type="danger" @click="handleStopCommand" plain>停止控制台</el-button> 
            <el-button type="warning" @click="handleClearInvalidVm" plain>清理失效虚拟机</el-button>
            <el-button type="info" @click="handleClearConsole" plain>清空控制台日志</el-button>
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
      <el-statistic title="可用" :value="138">
      </el-statistic>
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
              <el-statistic title="初始化中" :value="562">
                <template #suffix>
                  <el-icon style="vertical-align: -0.125em">
                    <ChatLineRound />
                  </el-icon>
                </template>
              </el-statistic>
            </el-col>
          </el-row>
                </el-col>
                
            </el-row>
        </div>
        <div class="row">
            <div class="console">
                <p v-for="(item, i) in logs" :key="i" :style="{color:item.color}">
                    <span>{{ item.time }}</span><span>{{ item.message }}</span>
                </p>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref, getCurrentInstance } from 'vue'
const { proxy } = getCurrentInstance()
type Log = {
    time: string,
    message: string,
    color: string
}
const logs = ref<Log[]>([]);

// addLog 新增日志记录
const addLog = (message: string, color:string = "#333") => {
    const currentDate = new Date();
    
    // 获取年、月、日、小时、分钟和秒
    const year = currentDate.getFullYear(); // 获取年份
    const month = currentDate.getMonth() + 1; // 获取月份（月份从0开始，所以要加    1）
    const day = currentDate.getDate(); // 获取日
    const hours = currentDate.getHours(); // 获取小时
    const minutes = currentDate.getMinutes(); // 获取分钟
    const seconds = currentDate.getSeconds(); // 获取秒
    const timeStr = `${year}-${month}-${day} ${hours}:${minutes}:${seconds} `
    logs.value.push({
        time: timeStr,
        message: message,
        color:color
    });
}

// 启动控制台
const handleStartCommand = () => {
    proxy.$message.success("启动控制台")
    addLog("启动控制台","green")
}
//handleStopCommand  停止控制台
const handleStopCommand = () => {
    proxy.$message.error("停止控制台")
    addLog("停止控制台","red")
}

// handleClearInvalidVm 清理失效虚拟机
const handleClearInvalidVm = () => {
    proxy.$message.error("清理失效虚拟机")
    addLog("清理失效虚拟机", "#eaeaea")
}
// handleClearConsole 清空控制台日志
const handleClearConsole = () => {
    proxy.$message.info("清空控制台日志")
    logs.value = []
}
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
    }
    .console p {
        padding-left: 10px;
        line-height: 10px;
    }
</style>