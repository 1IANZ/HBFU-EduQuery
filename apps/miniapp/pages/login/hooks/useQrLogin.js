import { ref, onUnmounted } from "vue";
import API from "@/utils/api.js";

export function useQrLogin(onSuccess) {
  const qrCodeImage = ref("");
  const qrSessionId = ref("");
  const qrUuid = ref("");
  const qrStudentId = ref("");
  const qrStatus = ref("");
  const loading = ref(false);

  let pollTimer = null;

  const getQRCode = () => {
    qrStatus.value = "loading";

    uni.request({
      url: API.auth.qrcode.uuid,
      method: "GET",
      success: (res) => {
        if (res.data.code === 0) {
          qrCodeImage.value = res.data.data.qrCodeImage;
          qrSessionId.value = res.data.data.sessionId;
          qrUuid.value = res.data.data.qrUuid;
          qrStatus.value = "waiting";
          startPolling();
        } else {
          qrStatus.value = "";
          uni.showToast({ title: "获取二维码失败", icon: "none" });
        }
      },
      fail: () => {
        qrStatus.value = "";
        uni.showToast({ title: "网络请求失败", icon: "none" });
      },
    });
  };

  const startPolling = () => {
    stopPolling();
    pollTimer = setInterval(pollQRStatus, 2000);
  };

  const stopPolling = () => {
    if (pollTimer) {
      clearInterval(pollTimer);
      pollTimer = null;
    }
  };

  const pollQRStatus = () => {
    if (!qrSessionId.value) return;

    uni.request({
      url: API.auth.qrcode.status(qrSessionId.value),
      method: "GET",
      success: (res) => {
        if (res.data.code !== 0) return;

        const status = res.data.data.status;

        if (status === "not_found") {
          qrStatus.value = "waiting";
        } else if (status === "scanning") {
          qrStatus.value = "scanning";
        } else if (status === "found") {
          qrStatus.value = "found";
          qrStudentId.value = res.data.data.studentId;
          stopPolling();
          loginByQRCode();
        }
      },
    });
  };

  const loginByQRCode = () => {
    loading.value = true;
    uni.showLoading({ title: "正在连接VPN..." });

    uni.request({
      url: API.auth.vpn.qrcode,
      method: "POST",
      data: {
        sessionId: qrSessionId.value,
        oaPassword: "dummy",
      },
      success: (res) => {
        loading.value = false;
        uni.hideLoading();

        if (res.data.code === 0) {
          const cookies = res.data.data.cookies;
          uni.setStorageSync("userCookies", cookies);

          uni.showToast({ title: "VPN连接成功", icon: "success" });

          onSuccess?.(cookies, qrStudentId.value);
        } else {
          uni.showToast({
            title: res.data.message || "VPN登录失败",
            icon: "none",
          });
          getQRCode();
        }
      },
      fail: () => {
        loading.value = false;
        uni.hideLoading();
        uni.showToast({ title: "网络请求失败", icon: "none" });
      },
    });
  };

  onUnmounted(() => {
    stopPolling();
  });

  return {
    qrCodeImage,
    qrStatus,
    qrStudentId,
    loading,
    getQRCode,
  };
}
