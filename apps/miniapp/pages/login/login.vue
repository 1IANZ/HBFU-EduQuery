<template>
  <view class="login-container">
    <view class="glass-card login-glass-wrap">
      <LoginHeader />

      <!-- 检测状态 -->
      <view v-if="viewState === 'checking'" class="loading-state">
        <text class="loading-text">正在检测连接状态...</text>
      </view>

      <!-- VPN 登录 -->
      <VpnLoginContainer
        v-if="viewState === 'vpn_login'"
        v-model:username="formData.username"
        v-model:vpnPassword="formData.vpnPassword"
        @vpn-success="handleVpnSuccess"
      />

      <!-- 教务系统登录 -->
      <JwxtLoginForm
        v-if="viewState === 'jwxt_login'"
        v-model:username="formData.username"
        v-model:password="formData.password"
        @success="handleJwxtSuccess"
        @back-to-vpn="handleBackToVpn"
      />
    </view>
  </view>
</template>

<script setup>
import LoginHeader from "./components/LoginHeader.vue";
import VpnLoginContainer from "./components/VpnLogin/VpnLoginContainer.vue";
import JwxtLoginForm from "./components/JwxtLoginForm.vue";
import { useLoginFlow } from "./hooks/useLoginFlow";

const handleJwxtSuccess = () => {
  uni.reLaunch({ url: "/pages/index/index" });
};

const { viewState, formData, handleVpnSuccess, handleBackToVpn } = useLoginFlow(
  {
    onLoginSuccess: handleJwxtSuccess,
  },
);
</script>

<style lang="scss">
@import "./styles/login.scss";
</style>
