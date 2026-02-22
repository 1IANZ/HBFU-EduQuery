<template>
  <view class="tab-switcher">
    <view
      class="tab-item"
      :class="{ active: modelValue === 'password' }"
      @click="$emit('update:modelValue', 'password')"
    >
      <text class="tab-text">密码登录</text>
    </view>

    <view
      class="tab-item"
      :class="{ active: modelValue === 'qrcode' }"
      @click="$emit('update:modelValue', 'qrcode')"
    >
      <text class="tab-text">扫码登录</text>
    </view>

    <view
      class="tab-indicator"
      :class="{ 'tab-qrcode': modelValue === 'qrcode' }"
    ></view>
  </view>
</template>

<script setup>
defineProps({
  modelValue: {
    type: String,
    default: "password",
  },
});
defineEmits(["update:modelValue"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-secondary: #64748b;

.tab-switcher {
  display: flex;
  position: relative;
  margin-bottom: 48rpx;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 28rpx;
  padding: 8rpx;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.tab-item {
  flex: 1;
  text-align: center;
  padding: 20rpx 0;
  position: relative;
  z-index: 2;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  &.active .tab-text {
    color: $accent-color;
    font-weight: 600;
  }
}

.tab-text {
  font-size: 28rpx;
  color: $text-secondary;
  transition: all 0.3s;
}

.tab-indicator {
  position: absolute;
  width: 50%;
  height: calc(100% - 16rpx);
  background: white;
  border-radius: 24rpx;
  top: 8rpx;
  left: 8rpx;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.08);
  z-index: 1;

  &.tab-qrcode {
    transform: translateX(calc(100% - 8rpx));
  }
}
</style>
