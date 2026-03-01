<template>
  <transition name="fade">
    <view v-if="visible" class="popup-overlay" @click="$emit('close')">
      <transition name="slide-up" appear>
        <view class="popup-card" @click.stop v-if="visible">
          <view class="popup-header">
            <text class="popup-title">成绩详情</text>
            <view class="popup-close" @click="$emit('close')">
              <uni-icons type="closeempty" size="22" color="#64748b" />
            </view>
          </view>

          <view class="popup-body">
            <view class="detail-item">
              <text class="detail-label">课程名称</text>
              <text class="detail-value">{{ score?.courseName }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">课程代码</text>
              <text class="detail-value">{{ score?.courseId }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">成绩</text>
              <text
                class="detail-value score-highlight"
                :class="getScoreClass(score?.score)"
              >
                {{ score?.score }}
              </text>
            </view>
            <view class="detail-item">
              <text class="detail-label">学期</text>
              <text class="detail-value">{{ score?.term }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">学分</text>
              <text class="detail-value">{{ score?.credit }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">学时</text>
              <text class="detail-value">{{ score?.hours }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">绩点</text>
              <text class="detail-value">{{ score?.gpa }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">考核方式</text>
              <text class="detail-value">{{ score?.examType }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">课程属性</text>
              <text class="detail-value">{{ score?.courseAttr }}</text>
            </view>
            <view class="detail-item">
              <text class="detail-label">课程性质</text>
              <text class="detail-value">{{ score?.courseNature }}</text>
            </view>
          </view>
        </view>
      </transition>
    </view>
  </transition>
</template>

<script setup>
defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  score: {
    type: Object,
    default: null,
  },
  getScoreClass: {
    type: Function,
    required: true,
  },
});

defineEmits(["close"]);
</script>
