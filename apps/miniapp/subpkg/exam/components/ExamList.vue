<template>
  <view class="exam-list-container">
    <scroll-view
      scroll-y
      class="exam-list"
    >
      <view class="inset-group-container" v-if="examList && examList.length > 0">
        <view
          class="exam-item active-scale"
          v-for="item in examList"
          :key="item.id"
          @click="$emit('itemClick', item)"
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

      <view v-if="!examList || examList.length === 0" class="empty-state-card">
        <view class="empty-emoji">🏖️</view>
        <text class="empty-title">近期没有考试</text>
        <text class="empty-subtitle">好好享受属于你的放松时光吧</text>
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

defineEmits(["refresh", "itemClick"]);
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
  background: var(--bg-card);
  border: 1px solid var(--border-card);
  border-radius: 36rpx;
  box-shadow: var(--shadow-light);
  overflow: hidden;
  margin-bottom: 40rpx;
}

.exam-item {
  border-bottom: 1px solid var(--border-card);
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
    color: var(--accent-color);
  }

  &.status-ongoing {
    background: rgba(16, 185, 129, 0.1);
    color: #10b981;
  }

  &.status-finished {
    background: var(--bg-card);
    color: var(--text-sub);
  }

  &.status-unknown {
    background: var(--bg-card);
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
  color: var(--text-sub);
  flex-shrink: 0;
}

.info-value {
  font-size: 26rpx;
  color: var(--text-main);
  flex: 1;
}


</style>
