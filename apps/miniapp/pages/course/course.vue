<template>
  <view class="container">
    <view class="blob blob-1"></view>
    <view class="blob blob-2"></view>
    <CourseHeader
      :semester-list="semesterList"
      :current-semester="currentSemester"
      :current-week="currentWeek"
      :season-label="seasonLabel"
      @semester-change="onSemesterChange"
      @prev-week="prevWeek"
      @next-week="nextWeek"
    />
    <CourseTable
      :week-days="weekDays"
      :sections="sections"
      :current-week="currentWeek"
      :actual-week="actualWeek || currentWeek" 
      :get-course="getCourse"
      :get-course-color="getCourseColor"
      @course-click="showCourseDetail"
    />
    <CourseDetailPopup
      :visible="showDetail"
      :course="selectedCourse"
      @close="closeDetail"
    />
  </view>
</template>

<script setup>
import { ref, computed } from "vue";
import { onLoad } from "@dcloudio/uni-app";
import CourseHeader from "./components/CourseHeader.vue";
import CourseTable from "./components/CourseTable.vue";
import CourseDetailPopup from "./components/CourseDetailPopup.vue";
import { useWeek } from "./hooks/useWeek";
import { useSemester } from "./hooks/useSemester";
import { useCourseTable } from "./hooks/useCourseTable";

const studentId = ref("");
const showDetail = ref(false);
const selectedCourse = ref(null);
const defaultSemesterValue = ref(null);

const { currentWeek, actualWeek, initWeek, prevWeek, nextWeek } = useWeek();

const isWinter = computed(() => {
  const month = new Date().getMonth() + 1;
  return month >= 10 || month <= 4;
});

const seasonLabel = computed(() => (isWinter.value ? "冬季" : "夏季"));

const winterSections = [
  { id: 1, name: "第一大节", time: "08:00-09:30" },
  { id: 2, name: "第二大节", time: "09:45-11:15" },
  { id: 3, name: "第三大节", time: "11:25-12:10" },
  { id: 4, name: "第四大节", time: "14:00-15:30" },
  { id: 5, name: "第五大节", time: "15:45-17:15" },
  { id: 6, name: "第六大节", time: "17:25-18:10" },
  { id: 7, name: "第七大节", time: "18:40-20:10" },
  { id: 8, name: "第八大节", time: "20:20-21:05" },
];

const summerSections = [
  { id: 1, name: "第一大节", time: "08:00-09:30" },
  { id: 2, name: "第二大节", time: "09:45-11:15" },
  { id: 3, name: "第三大节", time: "11:25-12:10" },
  { id: 4, name: "第四大节", time: "14:30-16:00" },
  { id: 5, name: "第五大节", time: "16:15-17:45" },
  { id: 6, name: "第六大节", time: "17:55-18:40" },
  { id: 7, name: "第七大节", time: "19:10-20:40" },
  { id: 8, name: "第八大节", time: "20:50-21:35" },
];

const sections = computed(() =>
  isWinter.value ? winterSections : summerSections,
);

const weekDays = [
  { name: "一", value: "星期一" },
  { name: "二", value: "星期二" },
  { name: "三", value: "星期三" },
  { name: "四", value: "星期四" },
  { name: "五", value: "星期五" },
  { name: "六", value: "星期六" },
  { name: "日", value: "星期日" },
];

const { semesterList, currentSemester, loadSemesters } = useSemester(studentId);

const { loadCourses, getCourse, getCourseColor } = useCourseTable(
  studentId,
  currentSemester,
);

const onSemesterChange = (index) => {
  currentSemester.value = semesterList.value[index];
  currentWeek.value = 1;
  loadCourses();
};

const showCourseDetail = (course) => {
  selectedCourse.value = course;
  showDetail.value = true;
};

const closeDetail = () => {
  showDetail.value = false;
  selectedCourse.value = null;
};

onLoad((options) => {
  const userInfo = uni.getStorageSync("userInfo");

  if (!userInfo?.studentId) {
    uni.showToast({
      title: "用户信息缺失，请重新登录",
      icon: "none",
    });
    return;
  }

  studentId.value = userInfo.studentId;

  if (options?.semester) {
    defaultSemesterValue.value = options.semester;
  }

  if (options?.startDate) {
    initWeek(options.startDate);
  }

  loadSemesters(defaultSemesterValue.value, () => {
    loadCourses();
  });
});
</script>

<style lang="scss">
@import "./styles/course.scss";
</style>
