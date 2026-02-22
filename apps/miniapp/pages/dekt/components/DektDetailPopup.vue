<template>
  <view v-if="visible" class="popup-overlay" @click="$emit('close')">
    <view class="popup-card" @click.stop>
      <view class="popup-header">
        <text class="popup-title">活动详情</text>
        <view class="popup-close" @click="$emit('close')">
          <uni-icons type="closeempty" size="22" color="#64748b" />
        </view>
      </view>

      <view class="popup-body">
        <view class="detail-row" v-if="loading">
          <text class="loading-text">加载详情中...</text>
        </view>
        <block v-else>
          <view class="detail-row" v-for="(val, key) in detailData" :key="key">
            <text class="detail-label">{{ key }}</text>
            <text class="detail-value">{{ val }}</text>
          </view>
        </block>
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
  detailData: {
    type: Object,
    default: () => ({}),
  },
  loading: {
    type: Boolean,
    default: false,
  },
});

defineEmits(["close"]);
</script>

<style lang="scss" scoped>
$primary-color: #3b82f6;
$text-main: #1e293b;
$text-sub: #64748b;

.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  z-index: 999;
  display: flex;
  align-items: center;
  justify-content: center;
}

.popup-card {
  width: 85%;
  background: #fff;
  border-radius: 24rpx;
  overflow: hidden;
  animation: popupScan 0.2s ease-out;
}

@keyframes popupScan {
  from {
    transform: scale(0.9);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

.popup-header {
  background: $primary-color;
  padding: 24rpx 32rpx;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.popup-title {
  color: #fff;
  font-size: 32rpx;
  font-weight: 700;
}

.popup-body {
  padding: 32rpx;
  max-height: 60vh;
  overflow-y: auto;
}

.detail-row {
  margin-bottom: 24rpx;
  border-bottom: 1px dashed #e2e8f0;
  padding-bottom: 24rpx;

  &:last-child {
    border-bottom: none;
    margin-bottom: 0;
  }
}

.detail-label {
  display: block;
  font-size: 24rpx;
  color: $text-sub;
  margin-bottom: 8rpx;
}

.detail-value {
  display: block;
  font-size: 28rpx;
  color: $text-main;
  line-height: 1.5;
}

.loading-text {
  text-align: center;
  color: $text-sub;
  display: block;
}
</style>
