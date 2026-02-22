import { ref, reactive, onMounted } from "vue";
import { useJwxtLogin } from "./useJwxtLogin";

export function useLoginFlow({ onLoginSuccess }) {
  const viewState = ref("checking");

  const formData = reactive({
    username: "",
    password: "",
    vpnPassword: "",
  });

  const { login: jwxtLogin } = useJwxtLogin(() => {
    onLoginSuccess?.();
  });

  const loadSavedCredentials = () => {
    const savedCreds = uni.getStorageSync("user_creds");
    if (savedCreds?.username) {
      formData.username = savedCreds.username;
      formData.vpnPassword = savedCreds.vpnPassword || "";
      formData.password = savedCreds.password || "";
    }
  };

  const fillVpnPassword = () => {
    const savedCreds = uni.getStorageSync("user_creds");
    if (savedCreds?.vpnPassword) {
      formData.vpnPassword = savedCreds.vpnPassword;
    }
  };

  const tryAutoJwxtLogin = () => {
    const savedCreds = uni.getStorageSync("user_creds");
    const username = formData.username || savedCreds?.username;
    const password = formData.password || savedCreds?.password;

    if (username && password) {
      formData.username = username;
      formData.password = password;
      jwxtLogin({
        username,
        password,
        onVpnInvalid: () => {
          fillVpnPassword();
          viewState.value = "vpn_login";
        },
      });
    }
  };

  const checkAutoLogin = () => {
    const isLogout = uni.getStorageSync("logout_flag");
    if (isLogout) {
      uni.removeStorageSync("logout_flag");
      viewState.value = "vpn_login";
      return;
    }

    viewState.value = "checking";
    uni.showLoading({ title: "连接检测中..." });

    loadSavedCredentials();

    const savedCookies = uni.getStorageSync("userCookies");
    const savedCreds = uni.getStorageSync("user_creds");

    viewState.value =
      savedCookies && savedCreds?.username && savedCreds?.password
        ? "jwxt_login"
        : "vpn_login";

    uni.hideLoading();
    tryAutoJwxtLogin();
  };

  const handleVpnSuccess = () => {
    viewState.value = "jwxt_login";
    tryAutoJwxtLogin();
  };

  const handleBackToVpn = () => {
    fillVpnPassword();
    viewState.value = "vpn_login";
  };

  onMounted(() => {
    checkAutoLogin();
  });

  return {
    viewState,
    formData,
    handleVpnSuccess,
    handleBackToVpn,
  };
}
