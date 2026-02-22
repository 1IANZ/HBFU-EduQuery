import { ref, reactive, computed, onMounted } from "vue";
import API from "@/utils/api";
import { request } from "@/utils/request";

export function usePlan(studentIdRef) {
  const semesterList = ref([]);
  const currentSemester = ref(null);
  const refreshing = ref(false);
  const selectedCourse = ref(null);

  const planData = reactive({
    plans: [],
    semesters: [],
  });

  const loadPlan = () => {
    if (!studentIdRef.value) return;
    uni.showLoading({ title: "加载中..." });
    request({
      url: API.user.plan(studentIdRef.value),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0) {
          Object.assign(planData, res.data.data || {});
          // 初始化学期列表并按升序排序
          semesterList.value = (planData.semesters || []).sort();
          if (semesterList.value.length > 0 && !currentSemester.value) {
            currentSemester.value = semesterList.value[0];
          }
        } else {
          uni.hideLoading();
          uni.showToast({ title: "加载培养方案失败", icon: "none" });
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

  const filteredPlans = computed(() => {
    if (!currentSemester.value) return planData.plans;
    return planData.plans.filter(
      (plan) => plan.semester === currentSemester.value,
    );
  });

  const onRefresh = () => {
    refreshing.value = true;
    loadPlan();
  };

  return {
    semesterList,
    currentSemester,
    refreshing,
    filteredPlans,
    loadPlan,
    onRefresh,
  };
}
