<template>
  <view class="container">
    <CustomNavBar title="选课信息" :show-back="true" />
    <ElectiveHeader
      :semesterList="semesterList"
      :currentSemester="currentSemester"
      @semesterChange="onSemesterChange"
    />

    <ElectiveList
      class="course-list-container"
      v-if="!showCredits"
      :courses="electiveData.courses"
      :refreshing="refreshing"
      @refresh="onRefresh"
      @itemClick="showDetail"
    />

    <ElectiveCredits
      class="credit-view-container"
      v-else
      :credits="electiveData.credits"
    />

    <view class="float-button" @click="toggleView">
      <uni-icons :type="showCredits ? 'list' : 'bars'" size="24" color="#fff" />
    </view>

    <ElectiveDetailPopup
      :visible="!!selectedCourse"
      :course="selectedCourse"
      @close="closeDetail"
    />
  </view>
</template>

<script setup>
import { onMounted, ref } from "vue";
import ElectiveHeader from "./components/ElectiveHeader.vue";
import ElectiveList from "./components/ElectiveList.vue";
import ElectiveCredits from "./components/ElectiveCredits.vue";
import ElectiveDetailPopup from "./components/ElectiveDetailPopup.vue";
import { useElective } from "./hooks/useElective";

const studentId = ref("");
const selectedCourse = ref(null);

const {
  semesterList,
  currentSemester,
  refreshing,
  showCredits,
  electiveData,
  loadSemesters,
  loadElective,
  onRefresh,
  toggleView,
} = useElective(studentId);

const onSemesterChange = (e) => {
  currentSemester.value = semesterList.value[e.detail.value];
  loadElective();
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
    loadSemesters();
  } else {
    console.error("User info or studentId not found in storage");
    uni.showToast({ title: "用户信息缺失，请重新登录", icon: "none" });
  }
});
</script>

<style lang="scss">
@import "./styles/elective.scss";
</style>
