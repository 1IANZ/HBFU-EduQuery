<template>
  <transition name="fade">
    <view v-if="visible" class="popup-overlay" @click="$emit('close')">
      <transition name="slide-up" appear>
        <view class="popup-card" @click.stop v-if="visible">
          <view class="popup-header">
            <text class="popup-title">课程详情</text>
            <view class="popup-close" @click="$emit('close')">
              <uni-icons type="closeempty" size="22" color="#64748b" />
            </view>
          </view>

          <view class="popup-body">
            <view class="info-item">
              <text class="info-label">课程名称</text>
              <text class="info-value">{{ course?.name }}</text>
            </view>
            <view class="info-item">
              <text class="info-label">授课教师</text>
              <text class="info-value">{{ course?.teacher }}</text>
            </view>
            <view class="info-item">
              <text class="info-label">上课地点</text>
              <text class="info-value">{{ course?.classroom }}</text>
            </view>
            <view class="info-item">
              <text class="info-label">上课时间</text>
              <text class="info-value">{{ course?.timeRange }}</text>
            </view>
            <view class="info-item">
              <text class="info-label">课程节次</text>
              <text class="info-value">{{ course?.duration }}</text>
            </view>
            <view class="info-item">
              <text class="info-label">上课周次</text>
              <text class="info-value">{{ course?.weeks }}</text>
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
    default: false,
  },
  course: {
    type: Object,
    default: null,
  },
});

defineEmits(["close"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;

.popup-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.popup-card {
  width: 640rpx;
  max-width: 92vw;
  max-height: 78vh;
  background: var(--bg-card);
  border-radius: 32rpx;
  overflow: hidden;
  box-shadow: 0 24rpx 64rpx rgba(0, 0, 0, 0.18);
  display: flex;
  flex-direction: column;
}

.popup-header {
  padding: 32rpx;
  background: var(--accent-color);
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
  background: var(--border-card);
  border-radius: 50%;

  &:active {
    background: var(--bg-body);
  }
}

.popup-body {
  flex: 1;
  min-height: 0;
  padding: 40rpx 40rpx 20rpx;
  overflow-y: auto;
}

.info-item {
  display: flex;
  flex-direction: column;
  padding-bottom: 24rpx;
  margin-bottom: 24rpx;
  border-bottom: 1px solid var(--border-card);

  &:last-child {
    border-bottom: none;
    margin-bottom: 20rpx;
  }
}

.info-label {
  font-size: 26rpx;
  color: var(--text-sub);
  margin-bottom: 12rpx;
}

.info-value {
  font-size: 32rpx;
  color: var(--text-main);
  font-weight: 600;
  line-height: 1.5;
  word-break: break-all;
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

