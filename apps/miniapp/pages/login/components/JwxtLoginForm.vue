<template>
  <view class="form-content">
    <view class="section-title">教务系统登录</view>

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
        <uni-icons type="person" size="20" color="var(--text-sub)" />
      </view>
    </view>

    <view class="input-group">
      <input
        class="input-field"
        type="text"
        :password="!showPassword"
        :value="props.password"
        @input="$emit('update:password', $event.detail.value)"
        placeholder="教务系统密码"
        placeholder-class="placeholder-style"
      />

      <view class="input-icon-wrapper">
        <uni-icons type="locked" size="20" color="var(--text-sub)" />
      </view>
      <view class="eye-icon-wrapper" @click="showPassword = !showPassword">
        <uni-icons
          :type="showPassword ? 'eye-filled' : 'eye-slash-filled'"
          size="20"
          color="var(--text-sub)"
        />
      </view>
    </view>

    <button class="btn-submit active-scale" :loading="loading" @click="handleLogin">
      登录教务系统
    </button>

    <view class="contact-btn" @click="$emit('back-to-vpn')">
      <text class="contact-text" style="color: var(--text-sub); font-size: 24rpx">
        重新登录 VPN
      </text>
    </view>
  </view>
</template>

<script setup>
import { ref } from "vue";
import { useJwxtLogin } from "../hooks/useJwxtLogin";

const props = defineProps({
  username: String,
  password: String,
});

const emit = defineEmits([
  "update:username",
  "update:password",
  "success",
  "back-to-vpn",
]);
const showPassword = ref(false);

const { loading, login } = useJwxtLogin(() => {
  emit("success");
});

const handleLogin = () => {
  login({
    username: props.username,
    password: props.password,
    onVpnInvalid: () => {
      emit("back-to-vpn");
    },
  });
};
</script>
