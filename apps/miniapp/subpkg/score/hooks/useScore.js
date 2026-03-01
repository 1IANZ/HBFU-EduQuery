import { ref, reactive, computed } from "vue";
import { request } from "@/utils/request.js";
import API from "@/utils/api.js";

const isPassScore = (score) => {
  if (score === null || score === undefined || score === "") return false;

  const numScore = parseFloat(score);
  if (!isNaN(numScore)) {
    return numScore >= 60;
  }
  const scoreStr = String(score).trim();
  return scoreStr === "通过";
};

const isTextScore = (score) => {
  if (score === null || score === undefined || score === "") return false;
  return isNaN(parseFloat(score));
};

export function useScore(studentIdRef, currentSemesterRef) {
  const scoreData = reactive({
    info: [],
    summary: null,
  });
  const sortOrder = ref("default");
  const refreshing = ref(false);
  const loading = ref(false);

  const passCount = computed(() => {
    if (!scoreData.info) return 0;
    return scoreData.info.filter((item) => isPassScore(item.score)).length;
  });

  const failCount = computed(() => {
    if (!scoreData.info) return 0;
    return scoreData.info.filter((item) => !isPassScore(item.score)).length;
  });

  const sortedScores = computed(() => {
    if (!scoreData.info || scoreData.info.length === 0) return [];
    if (sortOrder.value === "default") {
      return scoreData.info;
    }
    const sorted = [...scoreData.info];
    sorted.sort((a, b) => {
      const aIsText = isTextScore(a.score);
      const bIsText = isTextScore(b.score);

      if (aIsText && bIsText) return 0;
      if (aIsText) return sortOrder.value === "desc" ? 1 : -1;
      if (bIsText) return sortOrder.value === "desc" ? -1 : 1;
      if (sortOrder.value === "desc") {
        return parseFloat(b.score) - parseFloat(a.score);
      } else {
        return parseFloat(a.score) - parseFloat(b.score);
      }
    });
    return sorted;
  });

  const loadScores = (showLoading = true) => {
    if (!currentSemesterRef.value) return;

    const semester =
      currentSemesterRef.value.value === "all"
        ? null
        : currentSemesterRef.value.value;

    if (showLoading) {
      loading.value = true;
    }

    request({
      url: API.user.score(studentIdRef.value, semester),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0) {
          Object.assign(scoreData, res.data.data);
          if (scoreData.info && scoreData.summary) {
            const total = scoreData.info.reduce((sum, item) => {
              const credit = parseFloat(item.credit);
              if (isPassScore(item.score) && !isNaN(credit)) {
                return sum + credit;
              }
              return sum;
            }, 0);
            scoreData.summary.creditTotal = total;
          }
        } else {
          uni.showToast({
            title: "获取成绩失败",
            icon: "none",
          });
        }
      },
      complete: () => {
        if (showLoading) {
          loading.value = false;
        }
        if (refreshing.value) {
          refreshing.value = false;
        }
      },
    });
  };

  const toggleSort = () => {
    if (sortOrder.value === "default") {
      sortOrder.value = "desc";
    } else if (sortOrder.value === "desc") {
      sortOrder.value = "asc";
    } else {
      sortOrder.value = "default";
    }
  };

  const onRefresh = () => {
    refreshing.value = true;
    loadScores(false);
  };

  const getScoreClass = (score) => {
    if (isTextScore(score)) {
      return isPassScore(score) ? "pass" : "fail";
    }
    const numScore = parseFloat(score);
    if (numScore >= 90) return "excellent";
    if (numScore >= 80) return "good";
    if (numScore >= 70) return "medium";
    if (numScore >= 60) return "pass";
    return "fail";
  };

  return {
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
  };
}
