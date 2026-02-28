<template>
  <view class="glass-card">
    <view class="table-header">
      <view class="time-header">
        <text class="header-text">时间</text>
      </view>
      <view
        class="day-header"
        :class="{ 'day-header-today': isToday(day.value) }"
        v-for="day in weekDays"
        :key="day.value"
      >
        <view class="day-content" :class="{ 'today-content': isToday(day.value) }">
          <text class="header-text" :class="{ 'today-text': isToday(day.value) }">
            {{ day.name }}
          </text>
          <view v-if="isToday(day.value)" class="today-badge">今</view>
        </view>
      </view>
    </view>
    <view class="table-body">
      <view class="table-row" v-for="section in sections" :key="section.id">
        <view class="time-cell">
          <text class="time-name">{{ section.name }}</text>
          <text class="time-range">{{ section.time }}</text>
        </view>

        <view
          class="course-cell"
          :class="{ 'course-cell-today': isToday(day.value) }"
          v-for="day in weekDays"
          :key="day.value"
        >
          <view
            v-if="getCourse(day.value, section.id, currentWeek)"
            class="course-card active-scale"
            :style="{
              backgroundColor: getCourseColor(
                getCourse(day.value, section.id, currentWeek),
              ),
            }"
            @click="
              $emit(
                'course-click',
                getCourse(day.value, section.id, currentWeek),
              )
            "
          >
            <view class="course-content">
              <text class="course-name">
                {{ getCourse(day.value, section.id, currentWeek).name }}
              </text>
              <text class="course-room">
                {{ getCourse(day.value, section.id, currentWeek).classroom }}
              </text>
            </view>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup>
const props = defineProps({
  weekDays: {
    type: Array,
    required: true,
  },
  sections: {
    type: Array,
    required: true,
  },
  currentWeek: {
    type: Number,
    required: true,
  },
  actualWeek: {
    type: Number,
    default: 0,
  },
  getCourse: {
    type: Function,
    required: true,
  },
  getCourseColor: {
    type: Function,
    required: true,
  },
});

defineEmits(["course-click"]);

// 获取今天是星期几
const getTodayDayName = () => {
  const dayMap = [
    "星期日",
    "星期一",
    "星期二",
    "星期三",
    "星期四",
    "星期五",
    "星期六",
  ];
  return dayMap[new Date().getDay()];
};

// 判断是否是今天（只有当前显示的周是实际周才高亮）
const isToday = (dayValue) => {
  // actualWeek 为 0 或与 currentWeek 相等时才显示今天标识
  if (props.actualWeek > 0 && props.actualWeek !== props.currentWeek) {
    return false;
  }
  return dayValue === getTodayDayName();
};
</script>

<style lang="scss" scoped>
$accent-color: #3b82f6;
$text-primary: #1e293b;
$text-secondary: #64748b;

/* Glass Card - Modern Dashboard Style */
.glass-card {
  flex: 1;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 36rpx;
  border: 1px solid rgba(255, 255, 255, 0.9);
  box-shadow: 0 12rpx 32rpx rgba(148, 163, 184, 0.08);
  position: relative;
  z-index: 10;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  margin-top: 4rpx; /* Minimal margin since header was tightened */
  padding-bottom: 24rpx;
}

/* Table Header */
.table-header {
  display: flex;
  background: rgba(255, 255, 255, 0.4);
  border-bottom: 1px solid rgba(226, 232, 240, 0.6);
  flex-shrink: 0;
  padding: 12rpx 0; /* Reduced vertical padding inside table header */
}

.time-header {
  width: 90rpx;
  min-width: 90rpx;
  height: 60rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.day-header {
  flex: 1 1 0;
  min-width: 0;
  height: 60rpx;
  display: flex;
  justify-content: center;
  align-items: center;
}

.day-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  position: relative;
  border-radius: 12rpx;
  transition: all 0.3s ease;
}

.today-content {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
  transform: scale(1.05);
}

.header-text {
  color: #64748b;
  font-size: 24rpx;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.today-text {
  color: $accent-color;
  font-weight: 800;
}

.today-badge {
  position: absolute;
  top: -12rpx;
  right: -8rpx;
  background: linear-gradient(135deg, #ef4444 0%, #f97316 100%);
  color: white;
  font-size: 16rpx;
  font-weight: 800;
  padding: 2rpx 8rpx;
  border-radius: 20rpx;
  box-shadow: 0 4rpx 10rpx rgba(239, 68, 68, 0.3);
  z-index: 10;
  border: 1px solid rgba(255,255,255,0.8);
}

/* Table Body */
.table-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 0 8rpx;
}

.table-row {
  flex: 1;
  display: flex;
  min-height: 150rpx;
  border-bottom: 1px dashed rgba(148, 163, 184, 0.2);

  &:last-child {
    border-bottom: none;
  }
}

/* Soft Time Column */
.time-cell {
  width: 90rpx;
  min-width: 90rpx;
  padding: 16rpx 4rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.time-name {
  font-size: 22rpx;
  color: #94a3b8;
  font-weight: 600;
  text-align: center;
  margin-bottom: 2rpx;
}

.time-range {
  font-size: 16rpx;
  color: #cbd5e1;
  text-align: center;
}

/* Base Course Cell - Grid layout without hard vertical lines */
.course-cell {
  flex: 1 1 0;
  min-width: 0;
  padding: 6rpx;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: stretch;
  transition: background 0.3s ease;
}

.course-cell-today {
  background: linear-gradient(to bottom, rgba(59, 130, 246, 0.08), rgba(59, 130, 246, 0.02)); 
  border-radius: 16rpx;
  box-shadow: inset 0 0 20rpx rgba(59, 130, 246, 0.05);
}

.course-content {
  width: 100%;
  padding: 12rpx 8rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.course-card {
  width: 100%;
  height: 100%;
  border-radius: 20rpx; 
  position: relative;
  z-index: 5;
  
  /* Upgraded 3D Glass Pill Effect */
  box-shadow: 0 8rpx 20rpx rgba(0, 0, 0, 0.12), inset 0 2rpx 0 rgba(255, 255, 255, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.4);
  backdrop-filter: saturate(150%) blur(10px);

  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);

  &.active-scale:active {
    transform: scale(0.92);
    filter: brightness(0.9);
  }
}

.course-name {
  font-size: 22rpx;
  font-weight: 700;
  color: #ffffff;
  text-align: center;
  line-height: 1.3;
  margin-bottom: 6rpx;
  
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  word-break: break-all;
  text-shadow: 0 2rpx 4rpx rgba(0, 0, 0, 0.1);
}

.course-room {
  font-size: 16rpx;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.95);
  text-align: center;
  line-height: 1.2;
  
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  word-break: break-all;
}
</style>
