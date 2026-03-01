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
$accent-color: var(--accent-color, #3b82f6);

.tab-switcher {
  display: flex;
  position: relative;
  margin-bottom: 48rpx;
  background: var(--bg-card-glass);
  border-radius: 28rpx;
  padding: 8rpx;
  border: 1px solid var(--border-glass);
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
  color: var(--text-sub);
  transition: all 0.3s;
}

.tab-indicator {
  position: absolute;
  width: 50%;
  height: calc(100% - 16rpx);
  background: var(--bg-card);
  border-radius: 24rpx;
  top: 8rpx;
  left: 8rpx;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: var(--shadow-light);
  z-index: 1;

  &.tab-qrcode {
    transform: translateX(calc(100% - 8rpx));
  }
}
</style>
