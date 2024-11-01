import { defineStore } from "pinia";

export default defineStore("log", {
   state () {
      return {
         logs: [] as ILog[],
      }
   },
   actions: {
      addLog(message: string, color: string = "#333") {
          const currentDate = new Date();
          // 获取年、月、日、小时、分钟和秒
          const year = currentDate.getFullYear(); // 获取年份
          const month = currentDate.getMonth() + 1; // 获取月份（月份从0开始，所以要加    1）
          const day = currentDate.getDate(); // 获取日
          const hours = currentDate.getHours(); // 获取小时
          const minutes = currentDate.getMinutes(); // 获取分钟
          const seconds = currentDate.getSeconds(); // 获取秒
          const timeStr = `${year}-${month}-${day} ${hours}:${minutes}:${seconds} `;
          this.logs.push({
              time: timeStr,
              message: message,
              color: color,
          })
      },
       clearLog() {
           this.logs = []
       }
   }
});