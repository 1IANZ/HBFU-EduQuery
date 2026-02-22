import { ref } from "vue";
import API from "@/utils/api.js";
import { request } from "@/utils/request.js";

export function useJwxtLogin(onSuccess) {
  const loading = ref(false);

  const login = ({ username, password, onVpnInvalid }) => {
    if (!username) return uni.showToast({ title: "请输入学号", icon: "none" });
    if (!password)
      return uni.showToast({ title: "请输入教务系统密码", icon: "none" });

    const cookies = uni.getStorageSync("userCookies");
    if (!cookies) {
      uni.showToast({ title: "VPN 未连接，请重新登录", icon: "none" });
      onVpnInvalid?.();
      return;
    }

    loading.value = true;
    uni.showLoading({ title: "教务系统登录中..." });

    uni.request({
      url: API.auth.login_jwxt,
      method: "POST",
      data: {
        studentId: String(username),
        oaPassword: String(password),
        cookies,
      },
      success: (res) => {
        loading.value = false;
        uni.hideLoading();

        const result = res.data;
        if (result.code === 0) {
          handleLoginSuccess(username, password, result.data.cookies);
        } else {
          const msg = result.message || "";
          if (msg.includes("教务系统登录失败")) {
            uni.showToast({ title: "教务系统密码错误", icon: "none" });
          } else {
            uni.showToast({
              title: "VPN 连接已失效，请重新登录",
              icon: "none",
            });
            onVpnInvalid?.();
          }
        }
      },
      fail: () => {
        loading.value = false;
        uni.hideLoading();
        uni.showToast({ title: "网络请求失败", icon: "none" });
      },
    });
  };

  const handleLoginSuccess = (username, password, cookies) => {
    uni.setStorageSync("userCookies", cookies);
    const savedCreds = uni.getStorageSync("user_creds") || {};
    uni.setStorageSync("user_creds", {
      ...savedCreds,
      username,
      password,
    });

    uni.showToast({ title: "登录成功", icon: "success" });

    request({
      url: API.system.config,
      method: "GET",
      success: (res) => {
        if (res.data.code === 0) {
          uni.setStorageSync("systemConfig", res.data.data);
        }
      },
    });

    request({
      url: API.user.info(username),
      method: "POST",
      data: { cookies },
      success: (res) => {
        if (res.data.code === 0) {
          uni.setStorageSync("userInfo", res.data.data);
        }
      },
      complete: () => {
        onSuccess?.(username);
      },
    });
  };

  return {
    loading,
    login,
  };
}
