<template>
  <view class="container">
    <CustomNavBar title="考试安排" :show-back="true" />
    <ExamList
      :examList="examList"
      :refreshing="refreshing"
      :getExamStatus="getExamStatus"
      @refresh="onRefresh"
      @itemClick="showDetail"
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
  uni.vibrateShort({ type: 'light' });
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
