<template>
  <view class="container">
    <!-- Atmosphere Blobs -->
    <view class="blob blob-1"></view>
    <view class="blob blob-2"></view>

    <view class="glass-card">
      <view class="form-content">
        <!-- Name -->
        <view class="input-group">
          <text class="label">姓名</text>
          <view class="value-box" @click="copyText(userInfo.name, '姓名')">
            <text>{{ userInfo.name || "-" }}</text>
          </view>
        </view>

        <!-- Gender -->
        <view class="input-group">
          <text class="label">性别</text>
          <view class="value-box" @click="copyText(userInfo.gender, '性别')">
            <text>{{ userInfo.gender || "-" }}</text>
          </view>
        </view>

        <!-- Student ID -->
        <view class="input-group">
          <text class="label">学号</text>
          <view class="value-box" @click="copyText(userInfo.studentId, '学号')">
            <text>{{ userInfo.studentId || "-" }}</text>
          </view>
        </view>

        <!-- Department -->
        <view class="input-group">
          <text class="label">院系</text>
          <view
            class="value-box"
            @click="copyText(userInfo.department, '院系')"
          >
            <text>{{ userInfo.department || "-" }}</text>
          </view>
        </view>

        <!-- Major -->
        <view class="input-group">
          <text class="label">专业</text>
          <view class="value-box" @click="copyText(userInfo.major, '专业')">
            <text>{{ userInfo.major || "-" }}</text>
          </view>
        </view>

        <!-- Class -->
        <view class="input-group">
          <text class="label">班级</text>
          <view class="value-box" @click="copyText(userInfo.class, '班级')">
            <text>{{ userInfo.class || "-" }}</text>
          </view>
        </view>

        <!-- Admission Date -->
        <view class="input-group">
          <text class="label">入学日期</text>
          <view
            class="value-box"
            @click="copyText(userInfo.admissionDate, '入学日期')"
          >
            <text>{{ userInfo.admissionDate || "-" }}</text>
          </view>
        </view>

        <!-- Admission Number -->
        <view class="input-group">
          <text class="label">考生号</text>
          <view
            class="value-box"
            @click="copyText(userInfo.admissionNumber, '考生号')"
          >
            <text>{{ userInfo.admissionNumber || "-" }}</text>
          </view>
        </view>

        <!-- ID Number (Masked) -->
        <view class="input-group">
          <text class="label">身份证号</text>
          <view
            class="value-box id-box"
            @click="copyText(userInfo.idNumber, '身份证号')"
          >
            <text>{{
              showIdNumber ? userInfo.idNumber || "-" : maskedIdNumber
            }}</text>
            <view class="eye-icon" @click.stop="toggleIdVisibility">
              <uni-icons
                :type="showIdNumber ? 'eye-filled' : 'eye-slash-filled'"
                size="24"
                color="#3b82f6"
              ></uni-icons>
            </view>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup>
import { computed, onMounted, reactive, ref } from "vue";

// 响应式数据
const userInfo = reactive({});
const showIdNumber = ref(false);

// 计算属性
const maskedIdNumber = computed(() => {
  const id = userInfo.idNumber;
  if (!id) return "-";
  if (id.length > 7) {
    return (
      id.substring(0, 3) +
      "*".repeat(id.length - 7) +
      id.substring(id.length - 4)
    );
  }
  return "****************";
});

// 方法
const toggleIdVisibility = () => {
  showIdNumber.value = !showIdNumber.value;
};

const copyText = (text, label) => {
  if (!text || text === "-") return;
  uni.setClipboardData({
    data: text,
    success: () => {
      uni.showToast({
        title: "复制成功",
        icon: "none",
      });
    },
  });
};

// 生命周期
onMounted(() => {
  const storedInfo = uni.getStorageSync("userInfo");
  if (storedInfo) {
    Object.assign(userInfo, storedInfo);
  }
});
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-primary: #1e293b;
$text-secondary: #64748b;

.container {
  min-height: 100vh;
  background-color: #f8fafc;
  padding: 40rpx;
  display: flex;
  justify-content: center;
  padding-top: calc(40rpx + var(--status-bar-height));
  position: relative;
  overflow: hidden;
}

.blob {
  position: absolute;
  border-radius: 50%;
  z-index: 0;
  filter: blur(40rpx);
  opacity: 0.3;
}

.blob-1 {
  width: 600rpx;
  height: 600rpx;
  background: radial-gradient(circle, #e0e7ff 0%, rgba(224, 231, 255, 0) 70%);
  top: -200rpx;
  left: -200rpx;
  animation: float 8s infinite ease-in-out;
}

.blob-2 {
  width: 500rpx;
  height: 500rpx;
  background: radial-gradient(circle, #dbeafe 0%, rgba(219, 234, 254, 0) 70%);
  bottom: -150rpx;
  right: -150rpx;
  animation: float 10s infinite ease-in-out reverse;
}

@keyframes float {
  0%,
  100% {
    transform: translate(0, 0);
  }
  50% {
    transform: translate(20rpx, 20rpx);
  }
}

.glass-card {
  width: 100%;
  max-width: 650rpx;
  background: rgba(255, 255, 255, 0.9);
  /* backdrop-filter: blur(20px); Performance optimization */
  border-radius: 40rpx;
  border: 1px solid rgba(255, 255, 255, 0.8);
  box-shadow: 0 8rpx 20rpx -6rpx rgba(0, 0, 0, 0.05);
  padding: 60rpx 40rpx;
  position: relative;
  z-index: 1;
  margin-bottom: 40rpx; /* Bottom space */
}

.form-content {
  display: flex;
  flex-direction: column;
  gap: 32rpx;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 12rpx;
}

.label {
  font-size: 26rpx;
  color: $text-secondary;
  font-weight: 500;
  padding-left: 8rpx;
}

.value-box {
  background: rgba(255, 255, 255, 0.6);
  padding: 24rpx 32rpx;
  border-radius: 20rpx;
  border: 1px solid rgba(0, 0, 0, 0.05);
  font-size: 30rpx;
  color: $text-primary;
  font-weight: 500;
  min-height: 44rpx; /* Ensure height if empty */
  display: flex;
  align-items: center;

  &:active {
    background: rgba(255, 255, 255, 0.9);
  }

  &.id-box {
    justify-content: space-between;
    padding-right: 20rpx;
  }
}

.eye-icon {
  padding: 10rpx;
  display: flex;
  align-items: center;
  z-index: 10;
  flex-shrink: 0;
}
</style>
