import { ref } from "vue";
import { request } from "@/utils/request.js";
import API from "@/utils/api.js";
import { getConfig } from "@/utils/configCache.js";

export function useSemester(studentIdRef) {
  const semesterList = ref([]);
  const currentSemester = ref(null);

  const loadSemesters = (callback) => {
    if (!studentIdRef.value) {
      console.error("studentId is empty, cannot load semesters");
      uni.showToast({ title: "学号信息缺失，请重新登录", icon: "none" });
      return;
    }

    request({
      url: API.user.semester(studentIdRef.value, true),
      method: "POST",
      success: (res) => {
        semesterList.value = res.data.data || [];

        getConfig((config) => {
          let targetSemester = null;
          const examSemester =
            config?.data?.course?.examSemester || config?.course?.examSemester;

          if (examSemester) {
            targetSemester = semesterList.value.find(
              (item) => item.value === examSemester,
            );
          }

          currentSemester.value =
            targetSemester || semesterList.value[0] || null;

          if (callback) callback();
        });
      },
      fail: () => {
        uni.showToast({ title: "网络请求失败", icon: "none" });
      },
    });
  };

  return {
    semesterList,
    currentSemester,
    loadSemesters,
  };
}
