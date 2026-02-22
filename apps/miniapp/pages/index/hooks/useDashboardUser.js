import { ref } from "vue";

export function useDashboardUser() {
  const studentName = ref("同学");
  const studentId = ref("HBFU");

  const checkLogin = () => {
    const userCreds = uni.getStorageSync("user_creds");
    if (!userCreds || !userCreds.username || !userCreds.password) {
      uni.reLaunch({
        url: "/pages/login/login",
      });
      return false;
    }
    return true;
  };

  const loadUserInfo = () => {
    const userInfo = uni.getStorageSync("userInfo");
    if (userInfo) {
      studentName.value = userInfo.name || "同学";
      studentId.value = userInfo.studentId || "HBFU";
    }
  };

  return {
    studentName,
    studentId,
    checkLogin,
    loadUserInfo,
  };
}
