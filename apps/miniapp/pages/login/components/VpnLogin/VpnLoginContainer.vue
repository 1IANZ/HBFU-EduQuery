<template>
  <view>
    <view class="section-title">VPN 登录</view>

    <LoginTabs v-model="loginMode" />

    <VpnPasswordForm
      v-if="loginMode === 'password'"
      :username="username"
      :vpnPassword="vpnPassword"
      @update:username="(val) => emit('update:username', val)"
      @update:vpnPassword="(val) => emit('update:vpnPassword', val)"
      @success="handlePasswordSuccess"
    />

    <VpnQRCodeForm v-if="loginMode === 'qrcode'" @success="handleQrSuccess" />

    <FooterActions />
  </view>
</template>

<script setup>
import { ref, watch } from "vue";

import LoginTabs from "./LoginTabs.vue";
import VpnPasswordForm from "./VpnPasswordForm.vue";
import VpnQRCodeForm from "./VpnQRCodeForm.vue";
import FooterActions from "../FooterActions.vue";

const props = defineProps({
  username: {
    type: String,
    default: "",
  },
  vpnPassword: {
    type: String,
    default: "",
  },
});

const emit = defineEmits([
  "update:username",
  "update:vpnPassword",
  "vpn-success",
]);

const loginMode = ref(
  uni.getStorageSync("login_mode_preference") || "password",
);

watch(loginMode, (mode) => {
  uni.setStorageSync("login_mode_preference", mode);
});

const handlePasswordSuccess = () => {
  emit("vpn-success");
};

const handleQrSuccess = ({ cookies, studentId }) => {
  emit("update:username", String(studentId));
  emit("vpn-success", { cookies, studentId });
};
</script>
