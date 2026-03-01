<template>
  <view class="form-content">
    <view class="input-group">
      <input
        class="input-field"
        type="number"
        maxlength="14"
        :value="username"
        @input="$emit('update:username', $event.detail.value)"
        placeholder="学号"
        placeholder-class="placeholder-style"
      />
      <view class="input-icon-wrapper">
        <uni-icons type="person" size="20" color="var(--text-sub)"></uni-icons>
      </view>
    </view>

    <view class="input-group">
      <input
        class="input-field"
        type="text"
        :password="!showVpnPassword"
        :value="vpnPassword"
        @input="$emit('update:vpnPassword', $event.detail.value)"
        placeholder="VPN密码"
        placeholder-class="placeholder-style"
      />
      <view class="input-icon-wrapper">
        <uni-icons type="locked" size="20" color="var(--text-sub)"></uni-icons>
      </view>
      <view
        class="eye-icon-wrapper"
        @click="showVpnPassword = !showVpnPassword"
      >
        <uni-icons
          :type="showVpnPassword ? 'eye-filled' : 'eye-slash-filled'"
          size="20"
          color="var(--text-sub)"
        ></uni-icons>
      </view>
    </view>

    <!-- 验证码 -->
    <view class="input-group captcha-box" v-if="captchaBase64">
      <view class="captcha-input-wrapper">
        <input
          class="input-field"
          type="text"
          v-model="captcha"
          placeholder="验证码"
          placeholder-class="placeholder-style"
        />
        <view class="input-icon-wrapper">
          <uni-icons type="info" size="20" color="var(--text-sub)"></uni-icons>
        </view>
      </view>

      <image
        :src="captchaBase64"
        class="captcha-img"
        mode="scaleToFill"
        @click="refreshCaptcha"
      ></image>
    </view>

    <button class="btn-submit active-scale" :loading="loading" @click="handleSubmit">
      连接VPN
    </button>
  </view>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useVpnLogin } from "../../hooks/useVpnLogin";

const props = defineProps({
  username: { type: String, default: "" },
  vpnPassword: { type: String, default: "" },
});

const emit = defineEmits(["update:username", "update:vpnPassword", "success"]);

const showVpnPassword = ref(false);
const captcha = ref("");

const { loading, captchaBase64, getCaptcha, login } = useVpnLogin((cookies) => {
  emit("success", { cookies });
});

onMounted(() => {
  getCaptcha();
});

const refreshCaptcha = () => {
  captcha.value = "";
  getCaptcha();
};

const handleSubmit = () => {
  login({
    username: props.username,
    vpnPassword: props.vpnPassword,
    captcha: captcha.value,
  });
};
</script>

<style lang="scss">
@import "../../styles/login.scss";
</style>
