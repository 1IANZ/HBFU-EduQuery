<template>
  <view class="standard-card" @click="$emit('click')">
    <view class="card-icon" :class="theme">
      <slot name="icon" />
    </view>
    <text class="card-text">{{ title }}</text>
  </view>
</template>

<script setup>
defineProps({
  title: String,
  theme: String,
});
defineEmits(["click"]);
</script>

<style lang="scss" scoped>
.standard-card {
  background-color: #ffffff;
  border-radius: 24rpx;
  padding: 32rpx;
  height: 200rpx;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  border: 1px solid rgba(255, 255, 255, 0.6);
}

.card-icon {
  width: 72rpx;
  height: 72rpx;
  border-radius: 18rpx;
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-text {
  font-size: 30rpx;
  font-weight: 600;
  color: #334155;
}
</style>
