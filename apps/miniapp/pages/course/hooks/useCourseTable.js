// /pages/course/hooks/useCourseTable.js
import { ref, reactive } from "vue";
import API from "@/utils/api.js";
import { request } from "@/utils/request.js";

export function useCourseTable(studentIdRef, currentSemesterRef) {
  const courseList = ref([]);

  const colorPalette = [
    "#3b82f6",
    "#8b5cf6",
    "#ec4899",
    "#f97316",
    "#10b981",
    "#06b6d4",
    "#6366f1",
    "#f43f5e",
  ];

  const colorMap = reactive({});

  const loadCourses = () => {
    if (!studentIdRef.value || !currentSemesterRef.value) return;

    uni.showLoading({ title: "加载课程..." });

    request({
      url: API.user.course(studentIdRef.value, currentSemesterRef.value.value),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0 && res.data.data) {
          courseList.value = res.data.data;

          courseList.value.forEach((course) => {
            if (!colorMap[course.name]) {
              const index = Object.keys(colorMap).length % colorPalette.length;
              colorMap[course.name] = colorPalette[index];
            }
          });
        } else {
          uni.showToast({ title: "获取课程失败", icon: "none" });
        }
      },
      fail: () => {
        uni.showToast({ title: "网络请求失败", icon: "none" });
      },
      complete: () => {
        uni.hideLoading();
      },
    });
  };

  const getCourse = (dayOfWeek, section, currentWeek) => {
    return courseList.value.find(
      (course) =>
        course.dayOfWeek === dayOfWeek &&
        course.section === section &&
        course.weeksArray &&
        course.weeksArray.includes(currentWeek),
    );
  };

  const getCourseColor = (course) => {
    return colorMap[course.name] || colorPalette[0];
  };

  return {
    courseList,
    loadCourses,
    getCourse,
    getCourseColor,
  };
}
