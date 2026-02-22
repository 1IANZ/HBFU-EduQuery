import { ref, reactive, onMounted } from "vue";
import API from "@/utils/api";
import { request } from "@/utils/request";

export function useElective(studentIdRef) {
  const semesterList = ref([]);
  const currentSemester = ref(null);
  const refreshing = ref(false);
  const showCredits = ref(false);

  const electiveData = reactive({
    courses: [],
    credits: [],
  });

  const loadElective = () => {
    if (!currentSemester.value) return;
    uni.showLoading({ title: "加载中..." });
    request({
      url: API.user.elective(studentIdRef.value, currentSemester.value.value),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0) {
          Object.assign(electiveData, res.data.data || {});
        } else {
          uni.hideLoading();
          uni.showToast({ title: "加载选课数据失败", icon: "none" });
        }
      },
      fail: () => {
        uni.hideLoading();
        uni.showToast({ title: "网络连接失败", icon: "none" });
      },
      complete: () => {
        uni.hideLoading();
        refreshing.value = false;
      },
    });
  };

  const loadSemesters = () => {
    if (!studentIdRef.value) {
      console.error("studentId is empty, cannot load semesters");
      uni.showToast({ title: "学号信息缺失，请重新登录", icon: "none" });
      return;
    }

    request({
      url: API.user.semester(studentIdRef.value, false),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0) {
          semesterList.value = res.data.data || [];
          currentSemester.value = semesterList.value[0] || null;
          loadElective();
        }
      },
    });
  };

  const onRefresh = () => {
    refreshing.value = true;
    loadElective();
  };

  const toggleView = () => {
    showCredits.value = !showCredits.value;
  };

  return {
    semesterList,
    currentSemester,
    refreshing,
    showCredits,
    electiveData,
    loadSemesters,
    loadElective,
    onRefresh,
    toggleView,
  };
}
