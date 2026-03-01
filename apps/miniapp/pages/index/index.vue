<template>
  <view class="v7-dashboard">
    <!-- Main Scroll Container -->
    <view class="scroll-container">
      
      <!-- 1. Custom Navigation Bar -->
      <CustomNavBar title="河金小助手" />

      <!-- 1. Header (Greeting Section) -->
      <view class="v7-header greeting-section">
        <view class="header-user">
          <text class="greeting">Hi, {{ studentName || "刘芳年" }}</text>
          <text class="user-id">ID: {{ studentId || "20241216041022" }}</text>
        </view>
          <view class="avatar-wrap action-scale" @click="navigateTo('profile')">
            <view class="avatar-icon-bg">
              <text class="avatar-text">{{ (studentName || "刘").charAt(0) }}</text>
            </view>
          </view>
      </view>

      <!-- 2. Primary Hero Bento (Course & Score) -->
      <view class="hero-bento-grid">
        <!-- Next Course Card -->
        <view class="hero-card card-course action-scale" @click="navigateTo('course')">
          <view class="icon-box-blue">
            <uni-icons type="calendar-filled" size="24" color="#3b82f6" />
          </view>
          <text class="hero-title text-slate-800">课程安排</text>
        </view>

        <!-- Latest Score Card -->
        <view class="hero-card card-score action-scale" @click="navigateTo('score')">
          <view class="icon-box-emerald">
            <uni-icons type="medal-filled" size="24" color="#10b981" />
          </view>
          <text class="hero-title text-slate-800">成绩查询</text>
        </view>
      </view>

      <!-- 3. Secondary Nav (4 Items) -->
      <view class="nav-grid">
        <!-- Exam -->
        <view class="nav-item group action-scale" @click="navigateTo('exam')">
          <view class="nav-icon-box text-rose">
            <uni-icons type="compose" size="24" color="#f43f5e" />
          </view>
          <text class="nav-text">考试安排</text>
        </view>
        
        <!-- Elective -->
        <view class="nav-item group action-scale" @click="navigateTo('elective')">
          <view class="nav-icon-box text-violet">
            <uni-icons type="paperplane-filled" size="24" color="#8b5cf6" />
          </view>
          <text class="nav-text">选课信息</text>
        </view>

        <!-- Plan -->
        <view class="nav-item group action-scale" @click="navigateTo('plan')">
          <view class="nav-icon-box text-blue">
            <uni-icons type="info" size="24" color="#3b82f6" />
          </view>
          <text class="nav-text">执行计划</text>
        </view>

        <!-- Dekt -->
        <view class="nav-item group action-scale" @click="navigateTo('dekt')">
          <view class="nav-icon-box text-amber">
            <uni-icons type="flag-filled" size="24" color="#f59e0b" />
          </view>
          <text class="nav-text">第二课堂</text>
        </view>
      </view>

      <!-- 4. Campus Services List -->
      <view class="section-container">
        <text class="section-title">校园服务</text>
        <view class="list-card">
          <!-- Item 1: Calendar -->
          <view class="list-item action-scale" @click="showImage('xiaoli.png')">
            <view class="list-left">
              <view class="list-icon-bg bg-teal">
                <uni-icons type="calendar" size="16" color="#0d9488" />
              </view>
              <text class="list-text">学校校历</text>
            </view>
            <uni-icons type="right" size="16" color="#cbd5e1" />
          </view>
          
          <view class="list-divider"></view>

          <!-- Item 2: Map -->
          <view class="list-item action-scale" @click="showImage('ditu.png')">
            <view class="list-left">
              <view class="list-icon-bg bg-cyan">
                <uni-icons type="map-pin-ellipse" size="16" color="#0891b2" />
              </view>
              <text class="list-text">学校地图</text>
            </view>
            <uni-icons type="right" size="16" color="#cbd5e1" />
          </view>
        </view>
      </view>

      <!-- 5. Notices -->
      <view class="section-container pb-large">
        <text class="section-title">最新公告</text>
        
        <!-- Notice Component -->
        <view class="notice-card action-scale" @click="toggleNoticeExpand">
          <view class="notice-accent"></view>
          <view class="notice-icon-bg">
            <uni-icons type="sound-filled" size="16" color="#2563eb" />
          </view>
          <view class="notice-content">
            <text class="notice-main-text">{{ displayedNotices[0] || "有问题请及时反馈给管理员" }}</text>
            <text class="notice-contact">有问题请联系 · 微信号: Ez4Nian</text>
            <text class="notice-count" v-if="messages.length > 1">还有 {{messages.length - 1}} 条通知</text>
          </view>
        </view>
      </view>

    </view>
  </view>
</template>

<script setup>
import { ref, computed } from "vue";
import { onBackPress, onShow } from "@dcloudio/uni-app";
import { getCachedImage } from "@/utils/imageCache.js";
import API from "@/utils/api.js";
import { clearConfigCache } from "@/utils/configCache.js";

import NoticeList from "./components/NoticeList.vue";

import { useDashboardUser } from "./hooks/useDashboardUser";
import { useDashboardConfig } from "./hooks/useDashboardConfig";

const { studentName, studentId, checkLogin, loadUserInfo } = useDashboardUser();

const {
  noticeMessages: messages,
  isNoticeExpanded,
  displayedNotices,
  toggleNoticeExpand,
  courseConfig,
  fetchSystemConfig
} = useDashboardConfig();

const showImage = async (imageName) => {
  const remoteUrl = API.image.get(imageName);
  let urlToPreview = remoteUrl;

  try {
    uni.showLoading({ title: "加载中..." });
    urlToPreview = await getCachedImage(remoteUrl);
  } catch (e) {
    console.error("Cache failed, using remote", e);
  } finally {
    uni.hideLoading();
  }

  uni.previewImage({
    urls: [urlToPreview],
    current: urlToPreview,
  });
};

const navigateTo = (page) => {
  const routes = {
    profile: "/pages/profile/profile",
    course: "/subpkg/course/course",
    score: "/subpkg/score/score",
    exam: "/subpkg/exam/exam",
    elective: "/subpkg/elective/elective",
    plan: "/subpkg/plan/plan",
    dekt: "/subpkg/dekt/dekt",
  };

  if (!routes[page]) {
    uni.showToast({ title: "功能开发中", icon: "none" });
    return;
  }

  let url = routes[page];

  if (page === "course" && courseConfig.value) {
    url += `?semester=${courseConfig.value.semester}&startDate=${courseConfig.value.startDate}`;
  }

  uni.navigateTo({ url });
};

onShow(() => {
  if (!checkLogin()) return;
  loadUserInfo();
  fetchSystemConfig();
});

onBackPress(() => true);
</script>

<style lang="scss" scoped>
/* Core Setup */
.v7-dashboard {
  min-height: 100vh;
  background-color: var(--bg-body);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  display: flex;
  justify-content: center;
}

.scroll-container {
  width: 100%;
  box-sizing: border-box;
}

/* 1. Greeting Section (Below NavBar) */
.greeting-section {
  background-color: transparent;
  padding: 40rpx 32rpx 16rpx 32rpx;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-user {
  display: flex;
  flex-direction: column;
}

.greeting {
  font-size: 48rpx;
  font-weight: 800;
  color: var(--text-main); /* slate-800 */
  letter-spacing: -0.025em;
}

.user-id {
  font-size: 28rpx;
  color: var(--text-sub); /* slate-400 */
  margin-top: 4rpx;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-weight: 500;
}

.header-actions {
  display: flex;
  gap: 24rpx;
  align-items: center;
}

.icon-btn {
  width: 80rpx;
  height: 80rpx;
  border-radius: 50%;
  background-color: var(--bg-body); /* slate-50 */
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
}

.avatar-wrap {
  width: 96rpx;
  height: 96rpx;
  border-radius: 50%;
  border: 4rpx solid var(--border-card);
  overflow: hidden;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}
.avatar-icon-bg {
  width: 100%;
  height: 100%;
  background-color: var(--bg-body);
  display: flex;
  align-items: center;
  justify-content: center;
}
.avatar-text {
  font-size: 40rpx;
  font-weight: 700;
  color: var(--text-sub); /* slate-500 */
}

/* 2. Hero Bento Grid */
.hero-bento-grid {
  padding: 48rpx 32rpx 0 32rpx;
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 32rpx;
}

.hero-card {
  border-radius: 48rpx; /* 3xl */
  padding: 40rpx;
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
  box-sizing: border-box;
}

.card-course, .card-score {
  background-color: var(--bg-card);
  box-shadow: var(--shadow-light); /* shadow-sm */
  border: 1px solid var(--border-card); /* slate-100 */
}
.icon-box-blue {
  width: 80rpx;
  height: 80rpx;
  background-color: rgba(59, 130, 246, 0.15);
  border-radius: 32rpx; /* 2xl */
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 32rpx;
}

.icon-box-emerald {
  width: 80rpx;
  height: 80rpx;
  background-color: rgba(16, 185, 129, 0.15);
  border-radius: 32rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 32rpx;
}

.hero-title {
  font-size: 36rpx; /* text-lg */
  font-weight: 700;
  line-height: 1.2;
}
.text-slate-800 { color: var(--text-main); }

.hero-subtext {
  font-size: 24rpx; /* text-xs */
  margin-top: 12rpx;
}
.text-blue-100 { color: #dbeafe; opacity: 0.9; }
.text-slate-400 { color: var(--text-sub); }


/* 3. Secondary Nav Grid */
.nav-grid {
  padding: 64rpx 32rpx 0 32rpx;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 32rpx 16rpx; /* gap-y-4 gap-x-2 */
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.nav-icon-box {
  width: 112rpx; /* w-14 */
  height: 112rpx; /* h-14 */
  background-color: var(--bg-card);
  border-radius: 32rpx; /* 2xl */
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  border: 1px solid var(--border-card); /* slate-50 */
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 16rpx;
  transition: box-shadow 0.2s;
}

/* We use group-hover approximation with active state for mobile */
.nav-item:active .nav-icon-box {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1); /* shadow-md */
}

.nav-text {
  font-size: 22rpx; /* text-[11px] */
  color: var(--text-main); /* slate-600 */
  font-weight: 500;
}

/* 4 & 5. Shared Section Styles */
.section-container {
  padding: 64rpx 32rpx 0 32rpx;
}
.pb-large {
  padding-bottom: 64rpx;
}

.section-title {
  font-size: 26rpx; /* text-[13px] */
  font-weight: 700;
  color: var(--text-sub); /* slate-400 */
  margin-bottom: 24rpx;
  padding: 0 8rpx;
  text-transform: uppercase;
  letter-spacing: 0.05em; /* tracking-wider */
  display: block;
}

/* List Card */
.list-card {
  background-color: var(--bg-card);
  border-radius: 48rpx; /* rounded-[24px] approx */
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-card);
  overflow: hidden;
}

.list-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 32rpx;
  background-color: transparent;
  transition: background-color 0.2s;
}

.list-item:active {
  background-color: var(--bg-body);
}

.list-left {
  display: flex;
  align-items: center;
  gap: 32rpx;
}

.list-icon-bg {
  width: 64rpx; /* w-8 */
  height: 64rpx; /* h-8 */
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}
.bg-teal { background-color: var(--bg-body); }
.bg-cyan { background-color: var(--bg-body); }

.list-text {
  font-weight: 600; /* font-semibold */
  color: var(--text-main); /* slate-700 */
  font-size: 30rpx; /* text-[15px] */
}

.list-divider {
  height: 1px;
  background-color: var(--bg-body); /* slate-50 */
  margin-left: 128rpx; /* ml-16 */
}

/* Notice Card */
.notice-card {
  background-color: var(--bg-body);
  border-radius: 48rpx;
  padding: 20rpx 32rpx;
  border: 1px solid var(--border-card);
  display: flex;
  align-items: center;
  gap: 24rpx;
  position: relative;
  overflow: hidden;
}

.notice-accent {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 8rpx; /* w-1 */
  background-color: #3b82f6; /* blue-500 */
  border-top-left-radius: 48rpx;
  border-bottom-left-radius: 48rpx;
}

.notice-icon-bg {
  width: 56rpx;
  height: 56rpx;
  border-radius: 50%;
  background-color: var(--bg-card);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.notice-content {
  display: flex;
  flex-direction: column;
}

.notice-main-text {
  font-size: 28rpx; /* text-[14px] */
  color: var(--text-main); /* slate-700 */
  font-weight: 500;
  line-height: 1.625;
}

.notice-sub-info {
  font-size: 24rpx; /* text-[12px] */
  color: var(--text-sub); /* slate-500 */
  margin-top: 8rpx;
  display: flex;
  align-items: center;
  gap: 8rpx;
}

.notice-badge {
  font-family: ui-monospace, monospace;
  color: var(--accent-color);
  background-color: var(--bg-body);
  padding: 2rpx 8rpx;
  border-radius: 8rpx;
}
.notice-contact {
  font-size: 22rpx;
  color: var(--text-sub);
  margin-top: 6rpx;
}
.notice-count {
  font-size: 22rpx;
  color: var(--text-sub);
  margin-top: 6rpx;
}

/* Shared Interaction */
.action-scale {
  transition: transform 0.15s cubic-bezier(0.4, 0, 0.2, 1);
}
.action-scale:active {
  transform: scale(0.95);
}

</style>
