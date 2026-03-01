<template>
  <transition name="fade">
    <view v-if="visible" class="popup-overlay" @click="$emit('close')">
      <transition name="slide-up" appear>
        <view class="popup-card" @click.stop v-if="visible">
          <view class="popup-header">
            <text class="popup-title">课程详情</text>
            <view class="popup-close" @click="$emit('close')">
              <uni-icons type="closeempty" size="22" color="#fff" />
            </view>
          </view>

          <view class="popup-body">
            <view class="detail-item">
              <text class="detail-label">课程名称</text>
              <text class="detail-value">{{ course?.courseName }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">课程编号</text>
              <text class="detail-value">{{ course?.courseId }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">开课单位</text>
              <text class="detail-value">{{ course?.department }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">学时</text>
              <text class="detail-value">{{ course?.hours }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">学分</text>
              <text class="detail-value">{{ course?.credits }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">课程属性</text>
              <text class="detail-value">{{ course?.courseAttribute }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">选课方式</text>
              <text class="detail-value">{{ course?.selectionType }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">选课状态</text>
              <text class="detail-value highlight">{{ course?.selected }}</text>
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
  course: {
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
  background: $accent-color;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.popup-title {
  font-size: 34rpx;
  color: #fff;
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

.detail-label {
  font-size: 26rpx;
  color: var(--text-sub);
  margin-bottom: 12rpx;
}

.detail-value {
  font-size: 32rpx;
  color: var(--text-main);
  font-weight: 600;
  line-height: 1.5;
  word-break: break-all;

  &.highlight {
    color: $accent-color;
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
