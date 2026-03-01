<template>
  <view class="header-section">
    <picker
      mode="selector"
      :range="semesterList"
      @change="$emit('semester-change', $event)"
      class="semester-picker"
    >
      <view class="picker-box">
        <text class="picker-text">
          {{ currentSemester || "选择学期" }}
        </text>
        <uni-icons type="down" size="16" color="#64748b" />
      </view>
    </picker>
  </view>
</template>

<script setup>
defineProps({
  semesterList: {
    type: Array,
    default: () => [],
  },
  currentSemester: {
    type: String,
    default: "",
  },
});

defineEmits(["semester-change"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;

.header-section {
  background: rgba(255, 255, 255, 0.9);
  border-radius: 20rpx;
  padding: 16rpx;
  margin-bottom: 10rpx;
  position: relative;
  z-index: 1;
  box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.04);
}

.semester-picker {
  width: 100%;
}

.picker-box {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12rpx 20rpx;
  background: rgba(59, 130, 246, 0.08);
  border-radius: 12rpx;
}

.picker-text {
  color: $accent-color;
  font-size: 28rpx;
  font-weight: 500;
}
</style>
