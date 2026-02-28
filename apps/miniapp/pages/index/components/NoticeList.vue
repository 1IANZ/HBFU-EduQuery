<template>
  <view v-if="messages.length > 0">
    <view class="notice-list">
      <view class="notice-item active-scale" v-for="(msg, index) in displayed" :key="index">
        <text class="notice-title">{{ msg }}</text>
        <text class="notice-author">Alex</text>
      </view>

      <view
        class="notice-expand-btn"
        v-if="messages.length > 3"
        @click="$emit('toggle')"
      >
        <text>
          {{
            expanded ? "收起更多通知" : `展开剩余 ${messages.length - 3} 条通知`
          }}
        </text>
        <uni-icons :type="expanded ? 'up' : 'down'" size="14" />
      </view>
    </view>
  </view>
</template>

<script setup>
defineProps({
  messages: Array,
  displayed: Array,
  expanded: Boolean,
});
defineEmits(["toggle"]);
</script>

<style lang="scss" scoped>
.notice-list {
  display: flex;
  flex-direction: column;
  gap: 16rpx;
}
/* Note: .notice-item styles are kept in dashboard.scss globally for this module context, we remove the scoped explicit background here so it inherits */
.notice-title {
  font-size: 28rpx;
  flex: 1;
  margin-right: 16rpx;
  /* Truncate long text so it doesn't overlap */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.notice-author {
  color: #cbd5e1;
  flex-shrink: 0;
}
.notice-expand-btn {
  text-align: center;
  color: #94a3b8;
  padding: 16rpx;
}
</style>
