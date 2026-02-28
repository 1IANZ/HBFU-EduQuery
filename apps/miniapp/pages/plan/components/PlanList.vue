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
        <view class="inset-group-container" v-if="plans && plans.length > 0">
          <view
            v-for="item in plans"
            :key="item.id"
            class="course-item active-scale"
            @click="$emit('item-click', item)"
          >
            <view class="course-header">
              <text class="course-name">{{ item.courseName }}</text>
              <view class="course-credits">{{ item.credits }} 学分</view>
            </view>
          </view>
        </view>

        <view v-if="!plans || plans.length === 0" class="empty-state">
          <uni-icons type="info-filled" size="80" color="#cbd5e1" />
          <text class="empty-text">暂无课程信息</text>
        </view>
      </view>
    </scroll-view>
  </view>
</template>

<script setup>
defineProps({
  plans: {
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

/* Inset Group Container */
.inset-group-container {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border: 1px solid rgba(255, 255, 255, 0.9);
  border-radius: 36rpx;
  box-shadow: 0 8rpx 24rpx rgba(148, 163, 184, 0.04);
  overflow: hidden;
  margin-bottom: 40rpx;
}

.course-item {
  position: relative;
  z-index: 10;
  border-bottom: 1px solid rgba(226, 232, 240, 0.6);
  padding: 32rpx 36rpx;
  transition: background 0.2s;
  
  &:last-child {
    border-bottom: none;
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
