<template>
  <transition name="fade">
    <view v-if="visible" class="popup-overlay" @click="$emit('close')">
      <transition name="slide-up" appear>
        <view class="popup-card" @click.stop v-if="visible">
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
