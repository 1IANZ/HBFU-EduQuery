<template>
  <view class="header-section">
    <view class="header-text">
      <text class="header-greeting">Hi, {{ studentName }}</text>
      <text class="header-id">{{ studentId }}</text>
    </view>

    <view class="header-actions">
      <view class="icon-btn profile-btn" @click="$emit('profile')">
        <uni-icons type="person" size="20" color="#334155" />
      </view>
      <view class="icon-btn logout-btn" @click="$emit('logout')">
        <uni-icons type="redo" size="20" color="#64748b" />
      </view>
    </view>
  </view>
</template>

<script setup>
defineProps({
  studentName: String,
  studentId: String,
});

defineEmits(["profile", "logout"]);
</script>

<style lang="scss" scoped>
.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 20rpx;
  margin-bottom: 48rpx;
}

.header-greeting {
  font-size: 40rpx;
  font-weight: 700;
  color: #1e293b;
  display: block;
}

.header-id {
  font-size: 24rpx;
  color: #94a3b8;
  margin-top: 6rpx;
  display: block;
  font-family: monospace;
}

.header-actions {
  display: flex;
  gap: 20rpx;
}

.icon-btn {
  width: 80rpx;
  height: 80rpx;
  border-radius: 20rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #ffffff;
  border: 1px solid #e2e8f0;

  &:active {
    background-color: #f1f5f9;
  }
}
</style>
