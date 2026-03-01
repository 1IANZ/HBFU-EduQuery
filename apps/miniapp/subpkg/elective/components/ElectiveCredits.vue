<template>
  <view class="credit-view-container">
    <scroll-view scroll-y class="credit-view">
      <view class="credit-content">
        <view v-for="(item, index) in credits" :key="index" class="credit-card">
          <view class="credit-card-header">
            <text class="category-name">{{ item.category }}</text>
          </view>
          <view class="credit-card-body">
            <view class="credit-item">
              <text class="credit-label">必修</text>
              <text class="credit-value">{{ item.required }}</text>
            </view>
            <view class="credit-item">
              <text class="credit-label">限选</text>
              <text class="credit-value">{{ item.limited }}</text>
            </view>
            <view class="credit-item">
              <text class="credit-label">任选</text>
              <text class="credit-value">{{ item.elective }}</text>
            </view>
            <view class="credit-item">
              <text class="credit-label">公选</text>
              <text class="credit-value">{{ item.public }}</text>
            </view>
            <view class="credit-item total-item">
              <text class="credit-label">总计</text>
              <text class="credit-value total-value">{{ item.total }}</text>
            </view>
          </view>
        </view>
      </view>
    </scroll-view>
  </view>
</template>

<script setup>
defineProps({
  credits: {
    type: Array,
    default: () => [],
  },
});
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-primary: #1e293b;
$text-secondary: #64748b;

.credit-view-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.credit-view {
  flex: 1;
  height: 100%;
}

.credit-content {
  padding: 0 0 120rpx;
}

.credit-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 20rpx;
  margin-bottom: 16rpx;
  overflow: hidden;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.credit-card-header {
  background: linear-gradient(135deg, $accent-color 0%, #2563eb 100%);
  padding: 20rpx 24rpx;
}

.category-name {
  font-size: 28rpx;
  font-weight: 600;
  color: #fff;
}

.credit-card-body {
  padding: 20rpx 24rpx;
  display: flex;
  flex-direction: column;
  gap: 16rpx;
}

.credit-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12rpx 16rpx;
  background: #f8fafc;
  border-radius: 12rpx;

  &.total-item {
    background: rgba(59, 130, 246, 0.08);
    border: 1px solid rgba(59, 130, 246, 0.2);
  }
}

.credit-label {
  font-size: 26rpx;
  color: $text-secondary;
  font-weight: 500;
}

.credit-value {
  font-size: 28rpx;
  color: $text-primary;
  font-weight: 600;

  &.total-value {
    color: $accent-color;
    font-size: 32rpx;
    font-weight: 700;
  }
}
</style>
