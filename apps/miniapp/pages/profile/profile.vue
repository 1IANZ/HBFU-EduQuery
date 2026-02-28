<template>
  <view class="profile-container">
    <!-- Atmosphere Blobs -->
    <view class="blob blob-1"></view>
    <view class="blob blob-2"></view>

    <!-- Hero Header -->
    <view class="profile-hero">
      <view class="avatar-wrapper">
        <uni-icons type="person-filled" size="60" color="#fff" />
      </view>
      <text class="hero-name">{{ userInfo.name || '姓名' }}</text>
      <view class="hero-badges">
        <view class="badge origin-badge">{{ userInfo.department || '未知学院' }}</view>
        <view class="badge major-badge">{{ userInfo.major || '未知专业' }}</view>
      </view>
    </view>

    <!-- Inset Grouped Settings -->
    <view class="settings-list">
      
      <!-- Group 1: Basic Stats -->
      <view class="inset-group">
        <view class="setting-item" @click="copyText(userInfo.gender, '性别')">
          <view class="item-left">
            <view class="icon-box blue-bg"><uni-icons type="info" size="18" color="#3b82f6" /></view>
            <text class="item-label">性别</text>
          </view>
          <view class="item-right">
            <text class="item-value">{{ userInfo.gender || '-' }}</text>
          </view>
        </view>
        <view class="setting-item" @click="copyText(userInfo.class, '班级')">
          <view class="item-left">
            <view class="icon-box green-bg"><uni-icons type="flag-filled" size="18" color="#10b981" /></view>
            <text class="item-label">班级</text>
          </view>
          <view class="item-right">
            <text class="item-value">{{ userInfo.class || '-' }}</text>
          </view>
        </view>
        <view class="setting-item" @click="copyText(userInfo.admissionDate, '入学日期')">
          <view class="item-left">
            <view class="icon-box purple-bg"><uni-icons type="calendar-filled" size="18" color="#8b5cf6" /></view>
            <text class="item-label">入学日期</text>
          </view>
          <view class="item-right">
            <text class="item-value">{{ userInfo.admissionDate || '-' }}</text>
          </view>
        </view>
      </view>

      <!-- Group 2: Identity & Security -->
      <view class="group-title">身份信息</view>
      <view class="inset-group">
        <view class="setting-item" @click="copyText(userInfo.studentId, '学号')">
          <view class="item-left">
            <view class="icon-box indigo-bg"><uni-icons type="vip-filled" size="18" color="#6366f1" /></view>
            <text class="item-label">学号</text>
          </view>
          <view class="item-right">
            <text class="item-value mono-text">{{ userInfo.studentId || '-' }}</text>
          </view>
        </view>
        
        <view class="setting-item" @click="copyText(userInfo.admissionNumber, '考生号')">
          <view class="item-left">
            <view class="icon-box orange-bg"><uni-icons type="star-filled" size="18" color="#f59e0b" /></view>
            <text class="item-label">考生号</text>
          </view>
          <view class="item-right">
            <text class="item-value mono-text">{{ userInfo.admissionNumber || '-' }}</text>
          </view>
        </view>

        <view class="setting-item" @click="copyText(userInfo.idNumber, '身份证号')">
          <view class="item-left">
            <view class="icon-box red-bg"><uni-icons type="auth-filled" size="18" color="#ef4444" /></view>
            <text class="item-label">身份证号</text>
          </view>
          <view class="item-right">
            <text class="item-value mono-text">{{ showIdNumber ? userInfo.idNumber || '-' : maskedIdNumber }}</text>
            <view class="action-icon" @click.stop="toggleIdVisibility">
              <uni-icons
                :type="showIdNumber ? 'eye-filled' : 'eye-slash-filled'"
                size="20"
                color="#94a3b8"
              />
            </view>
          </view>
        </view>
      </view>

    </view>
    <view style="height: 100rpx"></view>
  </view>
</template>

<script setup>
import { computed, onMounted, reactive, ref } from "vue";

// Data
const userInfo = reactive({});
const showIdNumber = ref(false);

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

const toggleIdVisibility = () => {
  showIdNumber.value = !showIdNumber.value;
};

const copyText = (text, label) => {
  if (!text || text === "-") return;
  uni.setClipboardData({
    data: text,
    success: () => {
      uni.showToast({
        title: label + "已复制",
        icon: "none",
      });
    },
  });
};

onMounted(() => {
  const storedInfo = uni.getStorageSync("userInfo");
  if (storedInfo) {
    Object.assign(userInfo, storedInfo);
  }
});
</script>

<style lang="scss" scoped>
.profile-container {
  min-height: 100vh;
  padding: 0 32rpx;
  padding-top: calc(100rpx + var(--status-bar-height));
  position: relative;
  z-index: 10;
  box-sizing: border-box;
}

/* Hero Section */
.profile-hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 60rpx;
}

.avatar-wrapper {
  width: 180rpx;
  height: 180rpx;
  border-radius: 50%;
  background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 16rpx 32rpx rgba(59, 130, 246, 0.25);
  border: 4px solid rgba(255, 255, 255, 0.8);
  margin-bottom: 24rpx;
}

.hero-name {
  font-size: 48rpx;
  font-weight: 800;
  color: #1e293b;
  margin-bottom: 16rpx;
  text-shadow: 0 2rpx 10rpx rgba(255, 255, 255, 0.8);
}

.hero-badges {
  display: flex;
  gap: 16rpx;
}

.badge {
  padding: 8rpx 20rpx;
  border-radius: 30rpx;
  font-size: 24rpx;
  font-weight: 600;
  backdrop-filter: blur(8px);
}

.origin-badge {
  background: rgba(59, 130, 246, 0.15);
  color: #2563eb;
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.major-badge {
  background: rgba(139, 92, 246, 0.15);
  color: #7c3aed;
  border: 1px solid rgba(139, 92, 246, 0.3);
}

/* Inset Groups */
.settings-list {
  display: flex;
  flex-direction: column;
  gap: 32rpx;
}

.group-title {
  margin-left: 16rpx;
  margin-bottom: -16rpx;
  font-size: 26rpx;
  font-weight: 700;
  color: #64748b;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.inset-group {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border-radius: 32rpx;
  border: 1px solid rgba(255, 255, 255, 0.9);
  box-shadow: 0 8rpx 24rpx rgba(148, 163, 184, 0.04);
  overflow: hidden;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 32rpx 32rpx;
  border-bottom: 1px solid rgba(226, 232, 240, 0.6);
  transition: background 0.2s;

  &:last-child {
    border-bottom: none;
  }

  &:active {
    background: rgba(255, 255, 255, 0.9);
  }
}

.item-left {
  display: flex;
  align-items: center;
  gap: 20rpx;
}

.icon-box {
  width: 56rpx;
  height: 56rpx;
  border-radius: 16rpx;
  display: flex;
  align-items: center;
  justify-content: center;
}

.blue-bg { background: rgba(59, 130, 246, 0.15); }
.green-bg { background: rgba(16, 185, 129, 0.15); }
.purple-bg { background: rgba(139, 92, 246, 0.15); }
.indigo-bg { background: rgba(99, 102, 241, 0.15); }
.orange-bg { background: rgba(245, 158, 11, 0.15); }
.red-bg { background: rgba(239, 68, 68, 0.15); }

.item-label {
  font-size: 30rpx;
  font-weight: 600;
  color: #334155;
}

.item-right {
  display: flex;
  align-items: center;
  gap: 16rpx;
}

.item-value {
  font-size: 28rpx;
  color: #64748b;
  font-weight: 500;
}

.mono-text {
  font-family: monospace;
  font-size: 26rpx;
  letter-spacing: 0.5px;
}

.action-icon {
  width: 44rpx;
  height: 44rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: rgba(241, 245, 249, 0.8);
}
</style>
