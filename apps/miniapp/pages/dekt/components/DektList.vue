<template>
  <view class="list-container">
    <scroll-view
      scroll-y
      class="scroll-view-content"
      refresher-enabled
      :refresher-triggered="refreshing"
      @refresherrefresh="$emit('refresh')"
    >
      <view class="list-content">
        <view
          class="group-section"
          v-for="(group, gIndex) in groups"
          :key="gIndex"
        >
          <view class="group-header" @click="$emit('toggle-group', gIndex)">
            <view class="header-left">
              <view class="header-indicator"></view>
              <text class="group-title">{{ group.category }}</text>
            </view>
            <view class="header-right">
              <text class="group-credit">{{ group.totalCredit }} 分</text>
              <uni-icons
                :type="group.isExpanded ? 'up' : 'down'"
                size="14"
                color="#94a3b8"
              />
            </view>
          </view>

          <view class="group-body" v-if="group.isExpanded">
            <view
              class="dekt-card"
              v-for="(item, iIndex) in group.items"
              :key="item.id"
              @click="$emit('item-click', item)"
            >
              <view class="card-left">
                <text class="activity-name">{{
                  item.activityName || item.subCategory
                }}</text>
                <view class="card-meta">
                  <text class="meta-tag">{{ item.subCategory }}</text>
                  <text class="meta-tag">{{ item.semester }}</text>
                </view>
              </view>
              <view class="card-right">
                <view class="credit-badge">
                  <text class="credit-num">{{ item.credit }}</text>
                </view>
              </view>
            </view>

            <view v-if="group.items.length === 0" class="empty-group">
              <text>该分类下暂无活动</text>
            </view>
          </view>
        </view>

        <view v-if="groups.length === 0 && !loading" class="empty-state">
          <uni-icons type="info-filled" size="60" color="#cbd5e1" />
          <text>暂无第二课堂记录</text>
        </view>
      </view>
    </scroll-view>
  </view>
</template>

<script setup>
defineProps({
  groups: {
    type: Array,
    default: () => [],
  },
  refreshing: {
    type: Boolean,
    default: false,
  },
  loading: {
    type: Boolean,
    default: false,
  },
});

defineEmits(["refresh", "toggle-group", "item-click"]);
</script>

<style lang="scss" scoped>
$primary-color: #3b82f6;
$text-main: #1e293b;
$text-sub: #64748b;

.list-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  position: relative;
  z-index: 1;
}

.scroll-view-content {
  flex: 1;
  height: 100%;
}

.list-content {
  padding-bottom: 40rpx;
}

/* Group Section */
.group-section {
  margin-bottom: 24rpx;
  background: #fff;
  border-radius: 20rpx;
  overflow: hidden;
  box-shadow: 0 2rpx 12rpx rgba(0, 0, 0, 0.03);
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24rpx 32rpx;
  background: #fff;

  &:active {
    background: #f8fafc;
  }
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16rpx;
}

.header-indicator {
  width: 6rpx;
  height: 32rpx;
  background: $primary-color;
  border-radius: 4rpx;
}

.group-title {
  font-size: 30rpx;
  font-weight: 600;
  color: $text-main;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16rpx;
}

.group-credit {
  font-size: 28rpx;
  color: $primary-color;
  font-weight: 600;
}

.group-body {
  padding: 24rpx;
  background: #f8fafc;
  border-top: 1px solid #f1f5f9;
}

/* Item Card */
.dekt-card {
  background: #ffffff;
  border-radius: 16rpx;
  padding: 24rpx;
  margin-bottom: 16rpx;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border: 1px solid #e2e8f0;

  &:last-child {
    margin-bottom: 0;
  }

  &:active {
    transform: scale(0.99);
  }
}

.card-left {
  flex: 1;
  margin-right: 24rpx;
}

.activity-name {
  font-size: 28rpx;
  font-weight: 500;
  color: $text-main;
  margin-bottom: 8rpx;
  display: block;
  line-height: 1.4;
}

.card-meta {
  display: flex;
  gap: 12rpx;
}

.meta-tag {
  font-size: 20rpx;
  color: $text-sub;
  background: #f1f5f9;
  padding: 2rpx 10rpx;
  border-radius: 6rpx;
}

.card-right {
  flex-shrink: 0;
}

.credit-badge {
  background: #eff6ff;
  color: $primary-color;
  padding: 6rpx 16rpx;
  border-radius: 10rpx;
  font-weight: 700;
  font-size: 28rpx;
}

.empty-group {
  text-align: center;
  padding: 20rpx;
  color: $text-sub;
  font-size: 24rpx;
}

.empty-state {
  padding-top: 100rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20rpx;
  color: $text-sub;
  font-size: 28rpx;
}
</style>
