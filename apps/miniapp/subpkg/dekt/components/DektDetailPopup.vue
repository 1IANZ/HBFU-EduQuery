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
    default: false,
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
  inset: 0;
  width: 100vw;
  height: 100vh;
  box-sizing: border-box;
  padding-top: var(--custom-navbar-safe-height);
  padding-bottom: env(safe-area-inset-bottom);
  background: rgba(0, 0, 0, 0.4);
  z-index: 999;
  display: flex;
  align-items: center;
  justify-content: center;
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
  flex: 1;
  min-height: 0;
  padding: 40rpx 40rpx 20rpx;
  overflow-y: auto;
}

.detail-row {
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
}

.loading-text {
  text-align: center;
  color: var(--text-sub);
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
