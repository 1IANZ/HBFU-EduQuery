<template>
  <view class="container">
    <view class="blob blob-1"></view>
    <view class="blob blob-2"></view>
    <ExamList
      :exam-list="examList"
      :refreshing="refreshing"
      :get-exam-status="getExamStatus"
      @refresh="onRefresh"
      @item-click="showDetail"
    />

    <ExamDetailPopup
      :visible="!!selectedExam"
      :exam="selectedExam"
      @close="closeDetail"
    />
  </view>
</template>

<script setup>
import { ref, onMounted } from "vue";
import ExamList from "./components/ExamList.vue";
import ExamDetailPopup from "./components/ExamDetailPopup.vue";
import { useExam } from "./hooks/useExam";

const studentId = ref("");
const selectedExam = ref(null);

const { examList, refreshing, loadSemesters, onRefresh, getExamStatus } =
  useExam(studentId);

const showDetail = (item) => {
  selectedExam.value = item;
};

const closeDetail = () => {
  selectedExam.value = null;
};

onMounted(() => {
  const userInfo = uni.getStorageSync("userInfo");
  if (userInfo && userInfo.studentId) {
    studentId.value = userInfo.studentId;
    loadSemesters();
  } else {
    console.error("User info or studentId not found in storage");
    uni.showToast({ title: "用户信息缺失，请重新登录", icon: "none" });
  }
});
</script>

<style lang="scss">
@import "./styles/exam.scss";
</style>
