<template>
  <view class="container">
    <CustomNavBar title="执行计划" :show-back="true" />
    <PlanHeader
      :semesterList="semesterList"
      :currentSemester="currentSemester"
      @semesterChange="onSemesterChange"
    />

    <PlanList
      class="course-list-container"
      :plans="filteredPlans"
      :refreshing="refreshing"
      @refresh="onRefresh"
      @itemClick="showDetail"
    />

    <PlanDetailPopup
      :visible="!!selectedCourse"
      :course="selectedCourse"
      @close="closeDetail"
    />
  </view>
</template>

<script setup>
import { onMounted, ref } from "vue";
import PlanHeader from "./components/PlanHeader.vue";
import PlanList from "./components/PlanList.vue";
import PlanDetailPopup from "./components/PlanDetailPopup.vue";
import { usePlan } from "./hooks/usePlan";

const studentId = ref("");
const selectedCourse = ref(null);

const {
  semesterList,
  currentSemester,
  refreshing,
  filteredPlans,
  loadPlan,
  onRefresh,
} = usePlan(studentId);

const onSemesterChange = (e) => {
  currentSemester.value = semesterList.value[e.detail.value];
};

const showDetail = (item) => {
  uni.vibrateShort({ type: 'light' });
  selectedCourse.value = item;
};

const closeDetail = () => {
  selectedCourse.value = null;
};

onMounted(() => {
  const userInfo = uni.getStorageSync("userInfo");
  if (userInfo && userInfo.studentId) {
    studentId.value = userInfo.studentId;
    loadPlan();
  } else {
    console.error("User info or studentId not found in storage");
    uni.showToast({ title: "用户信息缺失，请重新登录", icon: "none" });
  }
});
</script>

<style lang="scss">
@import "./styles/plan.scss";
</style>
