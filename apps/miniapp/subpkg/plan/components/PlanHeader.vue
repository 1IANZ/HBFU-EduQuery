<template>
  <view class="header-section">
    <picker
      mode="selector"
      :range="semesterList"
      @change="$emit('semesterChange', $event)"
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
    required: true,
  },
  currentSemester: {
    type: String,
    default: "",
  },
});

defineEmits(["semesterChange"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;

.header-section {
  background: var(--bg-card);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border-radius: 36rpx;
  padding: 24rpx 32rpx;
  margin-bottom: 24rpx;
  position: relative;
  z-index: 10;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-card);
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
  color: var(--accent-color);
  font-size: 28rpx;
  font-weight: 500;
}
</style>
