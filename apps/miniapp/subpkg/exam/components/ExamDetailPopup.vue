<template>
  <transition name="fade">
    <view v-if="visible" class="popup-overlay" @click="$emit('close')">
      <transition name="slide-up" appear>
        <view class="popup-card" @click.stop v-if="visible">
          <view class="popup-header">
            <text class="popup-title">考试详情</text>
            <view class="popup-close" @click="$emit('close')">
              <uni-icons type="closeempty" size="22" color="var(--text-main)" />
            </view>
          </view>

          <view class="popup-body">
            <view class="detail-item">
              <text class="detail-label">课程名称</text>
              <text class="detail-value">{{ exam?.courseName }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">课程代码</text>
              <text class="detail-value">{{ exam?.courseCode }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">考试时间</text>
              <text class="detail-value">{{ exam?.examTime }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">考试地点</text>
              <text class="detail-value highlight">
                {{ exam?.examLocation }}
              </text>
            </view>
          </view>
        </view>
      </transition>
    </view>
  </transition>
</template>

<script setup>
defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  exam: {
    type: Object,
    default: null,
  },
});

defineEmits(["close"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-primary: #1e293b;
$text-secondary: #64748b;

.popup-overlay {
  position: fixed;
  inset: 0;
  width: 100vw;
  height: 100vh;
  box-sizing: border-box;
  padding-top: var(--custom-navbar-safe-height);
  padding-bottom: env(safe-area-inset-bottom);
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.popup-card {
  width: 640rpx;
  max-width: 92vw;
  max-height: calc(100vh - var(--custom-navbar-safe-height) - env(safe-area-inset-bottom) - 48rpx);
  background: var(--bg-card);
  border-radius: 32rpx;
  overflow: hidden;
  box-shadow: 0 24rpx 64rpx rgba(0, 0, 0, 0.18);
  display: flex;
  flex-direction: column;
}

.popup-header {
  padding: 32rpx;
  background: $accent-color;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.popup-title {
  font-size: 34rpx;
  color: var(--text-main);
  font-weight: 700;
}

.popup-close {
  width: 56rpx;
  height: 56rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 50%;

  &:active {
    background: rgba(255, 255, 255, 0.3);
  }
}

.popup-body {
  flex: 1;
  min-height: 0;
  padding: 40rpx 40rpx 20rpx;
  overflow-y: auto;
}

.detail-item {
  margin-bottom: 28rpx;

  &:last-child {
    margin-bottom: 0;
  }
}

.detail-label {
  display: block;
  font-size: 24rpx;
  color: var(--text-sub);
  margin-bottom: 8rpx;
}

.detail-value {
  display: block;
  font-size: 30rpx;
  color: var(--text-main);
  font-weight: 500;
  background: var(--bg-body);
  padding: 16rpx 20rpx;
  border-radius: 16rpx;

  &.highlight {
    background: linear-gradient(
      135deg,
      rgba(59, 130, 246, 0.1) 0%,
      rgba(59, 130, 246, 0.05) 100%
    );
    color: $accent-color;
    font-weight: 600;
  }
}

/* Animations */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active {
  transition: transform 0.4s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.4s ease;
}
.slide-up-leave-active {
  transition: transform 0.3s cubic-bezier(0.36, 0, 0.66, -0.56), opacity 0.3s ease;
}
.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(40rpx) scale(0.95);
  opacity: 0;
}
</style>
