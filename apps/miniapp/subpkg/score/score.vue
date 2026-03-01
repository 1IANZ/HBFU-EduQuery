<template>
  <view class="container">
    <CustomNavBar title="成绩查询" :show-back="true" />
    <ScoreHeader
      :semesterList="semesterList"
      :currentSemester="currentSemester"
      :sortOrder="sortOrder"
      :summary="scoreData.summary"
      :passCount="passCount"
      :failCount="failCount"
      @semesterChange="onSemesterChange"
      @toggleSort="toggleSort"
    />

    <ScoreList
      class="score-list-container"
      :scores="sortedScores"
      :refreshing="refreshing"
      :loading="loading"
      :getScoreClass="getScoreClass"
      @refresh="onRefresh"
      @itemClick="showDetail"
    />

    <ScoreDetailPopup
      :visible="!!selectedScore"
      :score="selectedScore"
      :getScoreClass="getScoreClass"
      @close="closeDetail"
    />
  </view>
</template>

<script setup>
import { ref, onMounted } from "vue";
import ScoreHeader from "./components/ScoreHeader.vue";
import ScoreList from "./components/ScoreList.vue";
import ScoreDetailPopup from "./components/ScoreDetailPopup.vue";
import { useSemester } from "./hooks/useSemester";
import { useScore } from "./hooks/useScore";

const studentId = ref("");
const selectedScore = ref(null);

const { semesterList, currentSemester, loadSemesters } = useSemester(studentId);

const {
  scoreData,
  sortOrder,
  refreshing,
  passCount,
  failCount,
  sortedScores,
  loadScores,
  toggleSort,
  onRefresh,
  getScoreClass,
  loading
} = useScore(studentId, currentSemester);

const onSemesterChange = (e) => {
  currentSemester.value = semesterList.value[e.detail.value];
  loadScores();
};

const showDetail = (item) => {
  uni.vibrateShort({ type: 'light' });
  selectedScore.value = item;
};

const closeDetail = () => {
  selectedScore.value = null;
};

onMounted(() => {
  const userInfo = uni.getStorageSync("userInfo");
  if (userInfo && userInfo.studentId) {
    studentId.value = userInfo.studentId;
    loadSemesters(() => {
      loadScores();
    });
  } else {
    console.error("User info or studentId not found in storage");
    uni.showToast({ title: "用户信息缺失，请重新登录", icon: "none" });
  }
});
</script>

<style lang="scss">
@import "./styles/score.scss";
</style>
