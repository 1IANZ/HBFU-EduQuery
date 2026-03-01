<template>
  <view class="score-list-wrapper">
    <scroll-view
      scroll-y
      class="score-list"
    >
      <!-- Skeleton UI -->
      <view class="inset-group-container skeleton-container" v-if="loading && (!scores || scores.length === 0)">
        <view class="skeleton-item" v-for="i in 10" :key="i">
           <view class="skeleton-text skeleton-pulse"></view>
           <view class="skeleton-badge skeleton-pulse"></view>
        </view>
      </view>

      <view class="inset-group-container" v-else-if="scores && scores.length > 0">
        <view
          class="score-card"
          v-for="item in scores"
          :key="item.id"
        >
          <view class="score-item active-scale" @click="$emit('itemClick', item)">
            <view class="score-item-content">
              <text class="course-name">{{ item.courseName }}</text>
              <view class="score-badge" :class="getScoreClass(item.score)">
                <text class="score-text">{{ item.score }}</text>
              </view>
            </view>
          </view>
        </view>
      </view>

      <view class="empty-state-card" v-else-if="!loading && (!scores || scores.length === 0)">
        <view class="empty-emoji">📝</view>
        <text class="empty-title">查不到成绩记录</text>
        <text class="empty-subtitle">去操场跑两圈放松一下再来看看吧</text>
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
  loading: {
    type: Boolean,
    default: false
  }
});

defineEmits(["refresh", "itemClick"]);
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

/* Inset Group Container */
.inset-group-container {
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: 36rpx;
  box-shadow: var(--shadow-light);
  overflow: hidden;
  margin-bottom: 40rpx;
}

.score-item {
  border-bottom: 1px solid var(--border-card);
  transition: background 0.2s;
  
  &:last-child {
    border-bottom: none;
  }
}

.score-item-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 32rpx 36rpx;
}

.course-name {
  font-size: 30rpx;
  font-weight: 600;
  color: var(--text-main);
  margin-right: 24rpx;
}

.score-badge {
  min-width: 88rpx;
  height: 88rpx;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 20rpx;
  flex-shrink: 0;
  box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.1);
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



/* Skeleton Loading */
.skeleton-container {
  padding: 0;
}
.skeleton-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 32rpx 36rpx;
  border-bottom: 1px solid rgba(226, 232, 240, 0.6);
  &:last-child {
    border-bottom: none;
  }
}
.skeleton-pulse {
  background: linear-gradient(90deg, var(--bg-body) 25%, var(--bg-card) 50%, var(--bg-body) 75%);
  background-size: 400% 100%;
  animation: pulse 1.5s ease-in-out infinite;
  border-radius: 12rpx;
}
.skeleton-text {
  height: 40rpx;
  width: 60%;
}
.skeleton-badge {
  width: 88rpx;
  height: 88rpx;
  border-radius: 20rpx;
}
@keyframes pulse {
  0% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(0.98); opacity: 0.5; }
  100% { transform: scale(1); opacity: 0.9; }
}
</style>
