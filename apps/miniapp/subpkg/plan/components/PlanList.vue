<template>
  <view class="course-list-container">
    <scroll-view
      scroll-y
      class="course-list"
    >
      <view class="course-list-content">
        <view class="inset-group-container" v-if="plans && plans.length > 0">
          <view
            v-for="item in plans"
            :key="item.id"
            class="course-item active-scale"
            @click="$emit('itemClick', item)"
          >
            <view class="course-header">
              <text class="course-name">{{ item.courseName }}</text>
              <view class="course-credits">{{ item.credits }} 学分</view>
            </view>
          </view>
        </view>

        <view v-if="!plans || plans.length === 0" class="empty-state-card">
          <view class="empty-emoji">🪴</view>
          <text class="empty-title">你的课程规划空空如也</text>
          <text class="empty-subtitle">快去添加你想选修的课程吧</text>
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

defineEmits(["refresh", "itemClick"]);
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
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: 36rpx;
  box-shadow: var(--shadow-light);
  overflow: hidden;
  margin-bottom: 40rpx;
}

.course-item {
  position: relative;
  z-index: 10;
  border-bottom: 1px solid var(--border-card);
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
  color: var(--text-main);
  line-height: 1.4;
  padding-right: 16rpx;
}

.course-credits {
  padding: 6rpx 16rpx;
  background: var(--bg-body);
  border-radius: 8rpx;
  font-size: 22rpx;
  color: var(--text-sub);
  flex-shrink: 0;
}


</style>
