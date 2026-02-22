<template>
  <view class="course-list-container">
    <scroll-view
      scroll-y
      class="course-list"
      refresher-enabled
      :refresher-triggered="refreshing"
      @refresherrefresh="$emit('refresh')"
    >
      <view class="course-list-content">
        <view
          v-for="item in courses"
          :key="item.courseId"
          class="course-card"
          @click="$emit('item-click', item)"
        >
          <view class="course-header">
            <text class="course-name">{{ item.courseName }}</text>
            <view class="course-credits">{{ item.credits }} 学分</view>
          </view>
        </view>

        <view v-if="courses.length === 0" class="empty-state">
          <uni-icons type="info-filled" size="80" color="#cbd5e1" />
          <text class="empty-text">暂无选课信息</text>
        </view>
      </view>
    </scroll-view>
  </view>
</template>

<script setup>
defineProps({
  courses: {
    type: Array,
    default: () => [],
  },
  refreshing: {
    type: Boolean,
    default: false,
  },
});

defineEmits(["refresh", "item-click"]);
</script>

<style lang="scss" scoped>
$text-primary: #1e293b;
$text-secondary: #64748b;

.course-list-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.course-list {
  flex: 1;
  height: 100%;
}

.course-list-content {
  padding-bottom: 120rpx;
}

.course-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 24rpx;
  margin-bottom: 16rpx;
  padding: 24rpx 28rpx;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.05);
  position: relative;
  z-index: 1;
  transition: transform 0.2s;

  &:active {
    transform: scale(0.98);
  }
}

.course-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.course-name {
  flex: 1;
  font-size: 32rpx;
  font-weight: 600;
  color: $text-primary;
  line-height: 1.4;
  padding-right: 16rpx;
}

.course-credits {
  padding: 6rpx 16rpx;
  background: #f1f5f9;
  border-radius: 8rpx;
  font-size: 22rpx;
  color: $text-secondary;
  flex-shrink: 0;
}

.empty-state {
  text-align: center;
  padding: 120rpx 0;
  color: $text-secondary;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24rpx;
}

.empty-text {
  font-size: 28rpx;
}
</style>
