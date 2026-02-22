import { ref } from "vue";
import API from "@/utils/api.js";

export function useVpnLogin(onSuccess) {
  const loading = ref(false);
  const captchaBase64 = ref("");
  const sessionId = ref("");

  const getCaptcha = () => {
    uni.request({
      url: API.auth.captcha,
      method: "GET",
      success: (res) => {
        if (res.data.code === 0) {
          captchaBase64.value = res.data.data.captchaBase64;
          sessionId.value = res.data.data.sessionId;
        } else {
          uni.showToast({ title: "验证码获取失败", icon: "none" });
        }
      },
      fail: () => {
        uni.showToast({ title: "网络请求失败", icon: "none" });
      },
    });
  };

  const login = ({ username, vpnPassword, captcha }) => {
    if (!username) return uni.showToast({ title: "请输入学号", icon: "none" });
    if (!vpnPassword)
      return uni.showToast({ title: "请输入VPN密码", icon: "none" });
    if (!captcha) return uni.showToast({ title: "请输入验证码", icon: "none" });

    loading.value = true;
    uni.showLoading({ title: "VPN登录中..." });

    uni.request({
      url: API.auth.vpn.manual,
      method: "POST",
      data: {
        studentId: String(username),
        vpnPassword: String(vpnPassword),
        captcha: String(captcha),
        sessionId: String(sessionId.value),
      },
      success: (res) => {
        loading.value = false;
        uni.hideLoading();

        const result = res.data;
        if (result.code === 0) {
          uni.setStorageSync("userCookies", result.data.cookies);
          const savedCreds = uni.getStorageSync("user_creds") || {};
          uni.setStorageSync("user_creds", {
            ...savedCreds,
            username,
            vpnPassword,
          });
          uni.showToast({ title: "VPN连接成功", icon: "success" });
          onSuccess?.(result.data.cookies);
        } else {
          uni.showToast({
            title: result.message || "VPN登录失败",
            icon: "none",
          });
          getCaptcha();
        }
      },
      fail: () => {
        loading.value = false;
        uni.hideLoading();
        uni.showToast({ title: "网络请求失败", icon: "none" });
        getCaptcha();
      },
    });
  };

  return {
    loading,
    captchaBase64,
    sessionId,
    getCaptcha,
    login,
  };
}
