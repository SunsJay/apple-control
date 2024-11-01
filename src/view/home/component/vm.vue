<!--
 * @Date: 2024-11-01 13:02:09
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-01 15:18:11
 * @FilePath: /apple-control/src/view/home/component/vm.vue
 * @Description: 
-->
<template>
    <el-table :data="state.data" style="width: 100%;height: 460px">
    <el-table-column prop="date" label="Date" width="180" />
    <el-table-column prop="name" label="Name" width="180" />
    <el-table-column prop="address" label="Address" />
    </el-table>
  <el-pagination
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      :current-page="state.currentPage"
      :page-sizes="[10, 20, 30, 40]"
      :page-size="state.pageSize"
      layout="total, sizes, prev, pager, next, jumper"
      :total="state.total">
  </el-pagination>
</template>
<script setup lang="ts">
import {reactive, ref} from "vue";
const state = reactive({
  data: [],
  total: 0,
  currentPage: 1,
  pageSize: 10,
})
const formData = reactive({
  page: 1,
  pageSize: 10,
})
const fetchData = () => {
  state.data = [
    {
      date: '2016-05-03',
      name: 'Tom',
      address: 'No. 189, Grove St, Los Angeles',
    },
    {
      date: '2016-05-02',
      name: 'Tom',
      address: 'No. 189, Grove St, Los Angeles',
    },
    {
      date: '2016-05-04',
      name: 'Tom',
      address: 'No. 189, Grove St, Los Angeles',
    },
    {
      date: '2016-05-01',
      name: 'Tom',
      address: 'No. 189, Grove St, Los Angeles',
    },
  ]
}

const handleSizeChange = (val: number) => {
  state.pageSize = val
  fetchData()
}

const handleCurrentChange = (val: number) => {
  state.currentPage = val
  fetchData()
}

fetchData()
</script>
<style></style>