<template>
  <view class="score-list-wrapper">
    <scroll-view
      scroll-y
      class="score-list"
      refresher-enabled
      :refresher-triggered="refreshing"
      @refresherrefresh="$emit('refresh')"
    >
      <view
        class="score-item"
        v-for="item in scores"
        :key="item.id"
        @click="$emit('item-click', item)"
      >
        <view class="score-item-content">
          <text class="course-name">{{ item.courseName }}</text>
          <view class="score-badge" :class="getScoreClass(item.score)">
            <text class="score-text">{{ item.score }}</text>
          </view>
        </view>
      </view>

      <view class="empty-state" v-if="!scores || scores.length === 0">
        <uni-icons type="info-filled" size="80" color="#cbd5e1" />
        <text class="empty-text">暂无成绩数据</text>
      </view>
    </scroll-view>
  </view>
</template>

<script setup>
defineProps({
  scores: {
    type: Array,
    default: () => [],
  },
  refreshing: {
    type: Boolean,
    default: false,
  },
  getScoreClass: {
    type: Function,
    required: true,
  },
});

defineEmits(["refresh", "item-click"]);
</script>

<style lang="scss" scoped>
$text-primary: #1e293b;
$text-secondary: #64748b;

.score-list-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.score-list {
  flex: 1;
  height: 100%;
}

.score-item {
  background: rgba(255, 255, 255, 0.9);
  border-radius: 16rpx;
  margin-bottom: 12rpx;
  box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.04);
  position: relative;
  z-index: 1;
  transition: transform 0.2s;

  &:active {
    transform: scale(0.98);
  }
}

.score-item-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20rpx 24rpx;
}

.course-name {
  flex: 1;
  font-size: 30rpx;
  font-weight: 500;
  color: $text-primary;
  margin-right: 16rpx;
}

.score-badge {
  min-width: 80rpx;
  height: 80rpx;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 12rpx;
  flex-shrink: 0;
}

.score-text {
  color: #fff;
  font-size: 32rpx;
  font-weight: 700;
}

.excellent {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.good {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
}

.medium {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
}

.pass {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
}

.fail {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
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
  color: $text-secondary;
}
</style>
