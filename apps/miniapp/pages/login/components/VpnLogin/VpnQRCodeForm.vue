<template>
  <view class="form-content">
    <view class="qr-code-section">
      <view class="qr-code-wrapper">
        <view v-if="qrCodeImage" class="qr-code-container">
          <image :src="qrCodeImage" class="qr-code-image" mode="aspectFit" />
        </view>

        <view v-else class="qr-code-placeholder" @click="getQRCode">
          <uni-icons type="scan" size="60" color="#94a3b8" />
          <text class="placeholder-text">点击获取二维码</text>
        </view>
      </view>

      <view v-if="qrStatus" class="qr-status">
        <text class="status-text" :class="qrStatus">
          {{ statusText }}
        </text>
      </view>
    </view>

    <button
      v-if="qrCodeImage && !loading"
      class="btn-submit btn-secondary"
      @click="getQRCode"
    >
      刷新二维码
    </button>
  </view>
</template>

<script setup>
import { computed, onMounted } from "vue";
import { useQrLogin } from "../../hooks/useQrLogin";

const emit = defineEmits(["success"]);

const { qrCodeImage, qrStatus, qrStudentId, loading, getQRCode } = useQrLogin(
  (cookies, studentId) => {
    emit("success", { cookies, studentId });
  },
);

onMounted(() => {
  getQRCode();
});

const statusText = computed(() => {
  switch (qrStatus.value) {
    case "loading":
      return "正在获取二维码...";
    case "waiting":
      return "请使用微信扫描二维码";
    case "scanning":
      return "检测到扫码，请在手机上确认";
    case "found":
      return `扫码成功！学号：${qrStudentId.value}`;
    default:
      return "";
  }
});
</script>
