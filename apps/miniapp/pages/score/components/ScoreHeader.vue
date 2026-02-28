<template>
  <view class="header-section">
    <view class="header-row">
      <picker
        mode="selector"
        :range="semesterList"
        range-key="key"
        @change="$emit('semester-change', $event)"
        class="semester-picker"
      >
        <view class="picker-box">
          <text class="picker-text">
            {{ currentSemester ? currentSemester.key : "选择学期" }}
          </text>
          <uni-icons type="down" size="16" color="#64748b" />
        </view>
      </picker>

      <view class="sort-btn active-scale" @click="$emit('toggle-sort')">
        <uni-icons
          :type="
            sortOrder === 'desc'
              ? 'arrow-down'
              : sortOrder === 'asc'
                ? 'arrow-up'
                : 'loop'
          "
          size="16"
          color="#3b82f6"
        />
        <text class="sort-text">{{
          sortOrder === "desc" ? "降序" : sortOrder === "asc" ? "升序" : "默认"
        }}</text>
      </view>
    </view>

    <view class="stats-box" v-if="summary">
      <view class="pass-fail-item pass">
        <text class="pass-fail-label">通过</text>
        <text class="pass-fail-value">{{ passCount }}</text>
      </view>
      <view class="pass-fail-item fail">
        <text class="pass-fail-label">挂科</text>
        <text class="pass-fail-value">{{ failCount }}</text>
      </view>
      <view class="stat-item">
        <text class="stat-label">总学分</text>
        <text class="stat-value">
          {{ summary.creditTotal }}
        </text>
      </view>
      <view class="stat-item">
        <text class="stat-label">平均绩点</text>
        <text class="stat-value">
          {{ summary.gpaAverage.toFixed(2) }}
        </text>
      </view>
    </view>
  </view>
</template>

<script setup>
defineProps({
  semesterList: {
    type: Array,
    required: true,
  },
  currentSemester: {
    type: Object,
    default: null,
  },
  sortOrder: {
    type: String,
    default: "default",
  },
  summary: {
    type: Object,
    default: null,
  },
  passCount: {
    type: Number,
    default: 0,
  },
  failCount: {
    type: Number,
    default: 0,
  },
});

defineEmits(["semester-change", "toggle-sort"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-secondary: #64748b;

.header-section {
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border: 1px solid rgba(255, 255, 255, 0.9);
  border-radius: 36rpx;
  padding: 32rpx;
  margin-bottom: 24rpx;
  position: relative;
  z-index: 10;
  box-shadow: 0 12rpx 32rpx rgba(148, 163, 184, 0.06);
}

.header-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16rpx;
  margin-bottom: 32rpx;
}

.semester-picker {
  flex: 1;
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

.sort-btn {
  display: flex;
  align-items: center;
  gap: 6rpx;
  padding: 12rpx 20rpx;
  background: rgba(59, 130, 246, 0.08);
  border-radius: 12rpx;

  &:active {
    background: rgba(59, 130, 246, 0.15);
  }
}

.sort-text {
  color: $accent-color;
  font-size: 26rpx;
  font-weight: 500;
}

.stats-box {
  display: flex;
  gap: 8rpx;
  margin-top: 12rpx;
}

.pass-fail-item,
.stat-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 16rpx 12rpx;
  border-radius: 24rpx;
  border: 1px solid;
  backdrop-filter: blur(8px);
}

.pass-fail-item.pass {
  background: rgba(16, 185, 129, 0.1);
  border-color: #10b981;
}

.pass-fail-item.fail {
  background: rgba(239, 68, 68, 0.1);
  border-color: #ef4444;
}

.stat-item {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.3);
}

.pass-fail-label,
.stat-label {
  font-size: 24rpx;
  color: $text-secondary;
  margin-bottom: 8rpx;
  display: block;
}

.pass-fail-value {
  font-size: 36rpx;
  font-weight: 700;
}

.pass-fail-item.pass .pass-fail-value {
  color: #10b981;
}

.pass-fail-item.fail .pass-fail-value {
  color: #ef4444;
}

.stat-value {
  font-size: 36rpx;
  font-weight: 700;
  color: $accent-color;
}
</style>
