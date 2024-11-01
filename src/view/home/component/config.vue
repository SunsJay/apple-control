<template>

    <el-form :model="formData" label-width="auto" style="max-width: 600px">
        <el-form-item label="名称">
            <el-input v-model="formData.name" placeholder="请输入名称" />
            <el-button type="primary" @click="onSubmit">提交</el-button>
        </el-form-item>
    </el-form>

</template>


<script setup lang="ts">
import { ref, reactive } from 'vue'
const formData = reactive({
    name: '', // 名称

})

const onSubmit = () => {
  console.log('submit!')
}
</script>
<style></style>