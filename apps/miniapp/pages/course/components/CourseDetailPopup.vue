<template>
  <view v-if="visible" class="popup-overlay" @click="$emit('close')">
    <view class="popup-card" @click.stop>
      <view class="popup-header">
        <text class="popup-title">课程详情</text>
        <view class="popup-close" @click="$emit('close')">
          <uni-icons type="closeempty" size="22" color="#64748b" />
        </view>
      </view>

      <view class="popup-body">
        <view class="info-item">
          <text class="info-label">课程名称</text>
          <text class="info-value">{{ course?.name }}</text>
        </view>
        <view class="info-item">
          <text class="info-label">授课教师</text>
          <text class="info-value">{{ course?.teacher }}</text>
        </view>
        <view class="info-item">
          <text class="info-label">上课地点</text>
          <text class="info-value">{{ course?.classroom }}</text>
        </view>
        <view class="info-item">
          <text class="info-label">上课时间</text>
          <text class="info-value">{{ course?.timeRange }}</text>
        </view>
        <view class="info-item">
          <text class="info-label">课程节次</text>
          <text class="info-value">{{ course?.duration }}</text>
        </view>
        <view class="info-item">
          <text class="info-label">上课周次</text>
          <text class="info-value">{{ course?.weeks }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup>
defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  course: {
    type: Object,
    default: null,
  },
});

defineEmits(["close"]);
</script>
