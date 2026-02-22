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
        <text class="header-text" :class="{ 'today-text': isToday(day.value) }">
          {{ day.name }}
        </text>
        <view v-if="isToday(day.value)" class="today-badge">今</view>
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
            class="course-card"
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

/* Glass Card */
.glass-card {
  flex: 1;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 32rpx;
  border: 1px solid rgba(255, 255, 255, 0.8);
  box-shadow: 0 8rpx 20rpx -6rpx rgba(0, 0, 0, 0.05);
  position: relative;
  z-index: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Table Header */
.table-header {
  display: flex;
  background: $accent-color;
  flex-shrink: 0;
}

.time-header {
  width: 100rpx;
  min-width: 100rpx;
  height: 70rpx;
  padding: 0 8rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-right: 1px solid rgba(255, 255, 255, 0.2);
  flex-shrink: 0;
  box-sizing: content-box;
}

.day-header {
  flex: 1 1 0;
  min-width: 0;
  height: 70rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border-right: 1px solid rgba(255, 255, 255, 0.2);
  position: relative;

  &:last-child {
    border-right: none;
  }
}

.day-header-today {
  background: linear-gradient(135deg, #ef4444 0%, #f97316 100%);
}

.header-text {
  color: #fff;
  font-size: 26rpx;
  font-weight: 600;
}

.today-text {
  font-weight: 700;
}

.today-badge {
  position: absolute;
  top: 4rpx;
  right: 4rpx;
  font-size: 16rpx;
  color: #fff;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.3);
  padding: 2rpx 6rpx;
  border-radius: 6rpx;
  line-height: 1;
}

.course-cell-today {
  background: rgba(239, 68, 68, 0.06);
}

/* Table Body */
.table-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.table-row {
  flex: 1;
  display: flex;
  border-bottom: 1px solid #f1f5f9;

  &:last-child {
    border-bottom: none;
  }
}

.time-cell {
  width: 100rpx;
  min-width: 100rpx;
  padding: 16rpx 8rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: #f8fafc;
  border-right: 1px solid #f1f5f9;
  flex-shrink: 0;
  box-sizing: content-box;
}

.time-name {
  font-size: 20rpx;
  color: $text-primary;
  font-weight: 600;
  text-align: center;
  margin-bottom: 4rpx;
}

.time-range {
  font-size: 16rpx;
  color: $text-secondary;
  text-align: center;
}
.course-content {
  width: 100%;
  padding: 0 8rpx;

  display: flex;
  flex-direction: column;
  align-items: center;
}
.course-cell {
  flex: 1 1 0;
  min-width: 0;
  height: 140rpx;
  padding: 6rpx;
  border-right: 1px solid #f1f5f9;
  position: relative;

  &:last-child {
    border-right: none;
  }
}

.course-card {
  width: 100%;
  height: 100%;
  border-radius: 12rpx;
  position: relative;
  z-index: 5;

  display: flex;
  align-items: center;
  justify-content: center;

  box-sizing: border-box;
}

.course-name {
  font-size: 20rpx;
  font-weight: 700;
  color: #fff;

  text-align: center;
  line-height: 1.35;

  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;

  overflow: hidden;
  text-overflow: ellipsis;
  word-break: break-all;
}
.course-room {
  margin-top: 6rpx;

  font-size: 14rpx;
  font-weight: 400;
  color: rgba(255, 255, 255, 0.9);

  text-align: center;
  line-height: 1.3;

  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;

  overflow: hidden;
  word-break: break-all;
}
</style>
