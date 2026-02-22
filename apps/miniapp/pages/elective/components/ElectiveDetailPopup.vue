<template>
  <view v-if="visible" class="popup-overlay" @click="$emit('close')">
    <view class="popup-card" @click.stop>
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
  </view>
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
  width: 85%;
  max-height: 75vh;
  background: #fff;
  border-radius: 32rpx;
  overflow: hidden;
  box-shadow: 0 20rpx 50rpx rgba(0, 0, 0, 0.15);
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
  padding: 32rpx;
  max-height: 55vh;
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
  color: $text-secondary;
  margin-bottom: 8rpx;
}

.detail-value {
  display: block;
  font-size: 30rpx;
  color: $text-primary;
  font-weight: 500;
  background: #f8fafc;
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
</style>
