<template>
  <view class="header-section">
    <!-- 学期选择 -->
    <picker
      mode="selector"
      :range="semesterList"
      range-key="key"
      @change="onSemesterChange"
      class="semester-picker"
    >
      <view class="picker-box">
        <text class="picker-text">
          {{ currentSemester?.key || "选择学期" }}
        </text>
        <uni-icons type="down" size="16" color="#64748b" />
      </view>
    </picker>

    <!-- 周切换 -->
    <view class="week-nav">
      <view class="nav-btn active-scale" @click="$emit('prev-week')">
        <uni-icons type="left" size="18" color="#3b82f6" />
      </view>
      <text class="week-text">第 {{ currentWeek }} 周</text>
      <view class="nav-btn active-scale" @click="$emit('next-week')">
        <uni-icons type="right" size="18" color="#3b82f6" />
      </view>
    </view>

    <!-- 季节 -->
    <view class="season-tag">{{ seasonLabel }}</view>
  </view>
</template>

<script setup>
const props = defineProps({
  semesterList: {
    type: Array,
    required: true,
  },
  currentSemester: {
    type: Object,
    default: null,
  },
  currentWeek: {
    type: Number,
    required: true,
  },
  seasonLabel: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(["semester-change", "prev-week", "next-week"]);

const onSemesterChange = (e) => {
  emit("semester-change", e.detail.value);
};
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-primary: #1e293b;
$text-secondary: #64748b;

.header-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20rpx 32rpx; /* Reduced vertical padding */
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 36rpx;
  border: 1px solid rgba(255, 255, 255, 0.9);
  margin-bottom: 8rpx; /* Significantly reduced to bring table closer */
  position: relative;
  z-index: 10;
  box-shadow: 0 8rpx 24rpx rgba(148, 163, 184, 0.06);
}

.semester-picker {
  flex: 1;
  max-width: 280rpx;
}

.picker-box {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8rpx 14rpx;
  background: rgba(59, 130, 246, 0.08);
  border-radius: 12rpx;
}

.picker-text {
  font-size: 26rpx;
  color: $accent-color;
  font-weight: 500;
}

.week-nav {
  display: flex;
  align-items: center;
  gap: 8rpx;
}

.nav-btn {
  width: 48rpx;
  height: 48rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(59, 130, 246, 0.08);
  border-radius: 50%;

  &:active {
    background: rgba(59, 130, 246, 0.15);
  }
}

.week-text {
  font-size: 28rpx;
  color: $text-primary;
  font-weight: 600;
  min-width: 100rpx;
  text-align: center;
}

.season-tag {
  font-size: 20rpx;
  color: #fff;
  background: $accent-color;
  padding: 6rpx 12rpx;
  border-radius: 10rpx;
}
</style>
