import { ref, computed } from "vue";
import { request } from "@/utils/request.js";
import API from "@/utils/api.js";
import { getConfig } from "@/utils/configCache.js";

export function useExam(studentIdRef) {
  const currentSemester = ref("");
  const examList = ref([]);
  const refreshing = ref(false);

  const loadExams = (showLoading = true) => {
    if (!currentSemester.value) return;

    if (showLoading) {
      uni.showLoading({ title: "加载中..." });
    }

    request({
      url: API.user.exam(studentIdRef.value, currentSemester.value),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0) {
          examList.value = res.data.data || [];
        } else {
          if (showLoading) uni.hideLoading();
          uni.showToast({ title: "获取考试安排失败", icon: "none" });
        }
      },
      fail: () => {
        if (showLoading) uni.hideLoading();
        uni.showToast({ title: "网络连接失败", icon: "none" });
      },
      complete: () => {
        if (showLoading) {
          uni.hideLoading();
        }
        refreshing.value = false;
      },
    });
  };

  const loadSemesterList = () => {
    request({
      url: API.user.semester(studentIdRef.value, false),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0 && res.data.data && res.data.data.length > 0) {
          // 使用第一个学期（通常是当前学期）
          currentSemester.value = res.data.data[0].value;
          loadExams();
        } else {
          uni.showToast({ title: "无法获取学期信息", icon: "none" });
        }
      },
      fail: () => {
        uni.showToast({ title: "获取学期失败", icon: "none" });
      },
    });
  };

  const loadSemesters = () => {
    getConfig((configData) => {
      if (configData?.course?.examSemester) {
        currentSemester.value = configData.course.examSemester;
        loadExams();
      } else {
        request({
          url: API.system.config,
          method: "GET",
          success: (configRes) => {
            if (configRes.data.code === 0 && configRes.data.data) {
              const config = configRes.data.data;
              uni.setStorageSync("systemConfig", config);
              if (config.course?.examSemester) {
                currentSemester.value = config.course.examSemester;
                loadExams();
                return;
              }
            }
            loadSemesterList();
          },
          fail: () => {
            loadSemesterList();
          },
        });
      }
    });
  };

  const onRefresh = () => {
    refreshing.value = true;
    loadExams(false);
  };

  const getExamStatus = (examTime) => {
    if (!examTime) return { text: "未知", class: "status-unknown" };

    try {
      // 解析考试时间 "2025-12-30 10:10~11:50"
      const timeMatch = examTime.match(
        /(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2})~(\d{2}:\d{2})/,
      );
      if (!timeMatch) return { text: "未知", class: "status-unknown" };

      const [, date, startTime, endTime] = timeMatch;
      const iosDate = date.replace(/-/g, "/");
      const startDateTime = new Date(`${iosDate} ${startTime}`);
      const endDateTime = new Date(`${iosDate} ${endTime}`);
      const now = new Date();

      if (now < startDateTime) {
        return { text: "未开始", class: "status-upcoming" };
      } else if (now >= startDateTime && now <= endDateTime) {
        return { text: "进行中", class: "status-ongoing" };
      } else {
        return { text: "已结束", class: "status-finished" };
      }
    } catch (e) {
      return { text: "未知", class: "status-unknown" };
    }
  };

  return {
    currentSemester,
    examList,
    refreshing,
    loadSemesters,
    loadExams,
    onRefresh,
    getExamStatus,
  };
}
