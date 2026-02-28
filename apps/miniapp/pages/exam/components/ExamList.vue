<template>
  <view class="exam-list-container">
    <scroll-view
      scroll-y
      class="exam-list"
      refresher-enabled
      :refresher-triggered="refreshing"
      @refresherrefresh="$emit('refresh')"
    >
      <view class="inset-group-container" v-if="examList && examList.length > 0">
        <view
          class="exam-item active-scale"
          v-for="item in examList"
          :key="item.id"
          @click="$emit('item-click', item)"
        >
          <view class="card-header">
            <text class="course-name">{{ item.courseName }}</text>
            <view
              class="status-badge"
              :class="getExamStatus(item.examTime).class"
            >
              {{ getExamStatus(item.examTime).text }}
            </view>
          </view>

          <view class="card-body">
            <view class="info-row">
              <uni-icons type="calendar" size="18" color="#64748b" />
              <text class="info-label">考试时间：</text>
              <text class="info-value">{{ item.examTime }}</text>
            </view>
            <view class="info-row">
              <uni-icons type="location" size="18" color="#64748b" />
              <text class="info-label">考试地点：</text>
              <text class="info-value">{{ item.examLocation }}</text>
            </view>
          </view>
        </view>
      </view>

      <view v-if="!examList || examList.length === 0" class="empty-state">
        <uni-icons type="info-filled" size="80" color="#cbd5e1" />
        <text class="empty-text">暂无考试安排</text>
      </view>
    </scroll-view>
  </view>
</template>

<script setup>
defineProps({
  examList: {
    type: Array,
    default: () => [],
  },
  refreshing: {
    type: Boolean,
    default: false,
  },
  getExamStatus: {
    type: Function,
    required: true,
  },
});

defineEmits(["refresh", "item-click"]);
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-primary: #1e293b;
$text-secondary: #64748b;

.exam-list-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.exam-list {
  flex: 1;
  height: 100%;
}

.exam-list-content {
  padding-bottom: 20rpx;
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

.exam-item {
  border-bottom: 1px solid rgba(226, 232, 240, 0.6);
  padding: 32rpx 36rpx;
  position: relative;
  z-index: 10;
  transition: background 0.2s;
  
  &:last-child {
    border-bottom: none;
  }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16rpx;
}

.course-name {
  flex: 1;
  font-size: 32rpx;
  font-weight: 600;
  color: $text-primary;
  line-height: 1.4;
  padding-right: 16rpx;
}

.status-badge {
  padding: 6rpx 16rpx;
  border-radius: 8rpx;
  font-size: 22rpx;
  flex-shrink: 0;
  font-weight: 500;

  &.status-upcoming {
    background: rgba(59, 130, 246, 0.1);
    color: #3b82f6;
  }

  &.status-ongoing {
    background: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  &.status-finished {
    background: #f1f5f9;
    color: #64748b;
  }

  &.status-unknown {
    background: #f1f5f9;
    color: #94a3b8;
  }
}

.card-body {
  display: flex;
  flex-direction: column;
  gap: 12rpx;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 8rpx;
}

.info-label {
  font-size: 26rpx;
  color: $text-secondary;
  flex-shrink: 0;
}

.info-value {
  font-size: 26rpx;
  color: $text-primary;
  flex: 1;
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
