<template>
  <view class="dashboard-page fluid-layout">
    <!-- Atmosphere Blobs -->
    <view class="blob blob-1"></view>
    <view class="blob blob-2"></view>

    <!-- Unified Hero Section -->
    <view class="hero-section">
      <view class="hero-top">
        <view class="user-info">
          <text class="greeting">Hi, {{ studentName || '同学' }}</text>
          <text class="student-subtitle">{{ studentId || '加载中...' }}</text>
        </view>
        <view class="hero-actions">
          <view class="icon-btn" hover-class="icon-hover" @click="navigateTo('profile')">
            <uni-icons type="person" size="22" color="#1e293b" />
          </view>
          <view class="icon-btn logout-btn" hover-class="icon-hover" @click="handleLogout">
            <uni-icons type="redo" size="22" color="#ef4444" />
          </view>
        </view>
      </view>

      <!-- Primary Actions inside Hero -->
      <view class="primary-action-row">
        <view class="primary-btn course-btn active-scale" @click="navigateTo('course')">
          <view class="p-icon"><uni-icons type="calendar-filled" size="24" color="#fff" /></view>
          <text class="p-text">课程安排</text>
        </view>
        <view class="primary-btn score-btn active-scale" @click="navigateTo('score')">
          <view class="p-icon"><uni-icons type="medal-filled" size="24" color="#fff" /></view>
          <text class="p-text">成绩查询</text>
        </view>
      </view>
    </view>

    <!-- Horizontal Strip (Secondary Actions) -->
    <view class="action-strip-container">
      <scroll-view scroll-x class="action-strip" :show-scrollbar="false" enhanced>
        <view class="strip-content">
          <view class="strip-item active-scale" @click="navigateTo('exam')">
            <view class="circle-icon red-light"><uni-icons type="compose" size="28" color="#ef4444" /></view>
            <text class="strip-text">考试安排</text>
          </view>
          <view class="strip-item active-scale" @click="navigateTo('elective')">
            <view class="circle-icon purple-light"><uni-icons type="paperplane-filled" size="28" color="#8b5cf6" /></view>
            <text class="strip-text">选课中心</text>
          </view>
          <view class="strip-item active-scale" @click="navigateTo('plan')">
            <view class="circle-icon indigo-light"><uni-icons type="info" size="28" color="#6366f1" /></view>
            <text class="strip-text">执行计划</text>
          </view>
          <view class="strip-item active-scale" @click="navigateTo('dekt')">
            <view class="circle-icon orange-light"><uni-icons type="flag-filled" size="28" color="#f59e0b" /></view>
            <text class="strip-text">第二课堂</text>
          </view>
        </view>
      </scroll-view>
    </view>

    <!-- Inset Grouped Lists Instead of Blocks -->
    <view class="content-body">
      
      <!-- Tools Inset Group -->
      <view class="section-heading">校园服务</view>
      <view class="inset-list">
        <view class="list-item active-scale" @click="showImage('xiaoli.png')">
          <view class="item-left">
            <uni-icons type="calendar" size="22" color="#14b8a6" />
            <text class="item-text">学校校历</text>
          </view>
          <uni-icons type="right" size="16" color="#cbd5e1" class="chevron" />
        </view>
        <view class="list-item active-scale" @click="showImage('ditu.png')">
          <view class="item-left">
            <uni-icons type="map-pin-ellipse" size="22" color="#06b6d4" />
            <text class="item-text">学校地图</text>
          </view>
          <uni-icons type="right" size="16" color="#cbd5e1" class="chevron" />
        </view>
      </view>

      <!-- Notices Inset Group -->
      <view class="section-heading mt-lg">最新公告</view>
      <view class="inset-list p-0">
        <NoticeList
          :messages="noticeMessages"
          :displayed="displayedNotices"
          :expanded="isNoticeExpanded"
          @toggle="toggleNoticeExpand"
          class="fluid-notice-list"
        />
      </view>
      
    </view>
    
    <view class="safe-bottom-padding"></view>
  </view>
</template>

<script setup>
import { onBackPress, onShow } from "@dcloudio/uni-app";
import { getCachedImage } from "@/utils/imageCache.js";
import API from "@/utils/api.js";
import { clearConfigCache } from "@/utils/configCache.js";

import NoticeList from "./components/NoticeList.vue";

import { useDashboardUser } from "./hooks/useDashboardUser";
import { useDashboardConfig } from "./hooks/useDashboardConfig";

const { studentName, studentId, checkLogin, loadUserInfo } = useDashboardUser();
const {
  noticeMessages,
  isNoticeExpanded,
  displayedNotices,
  toggleNoticeExpand,
  courseConfig,
  fetchSystemConfig,
} = useDashboardConfig();

const handleLogout = () => {
  uni.showModal({
    title: "提示",
    content: "确定要退出登录吗？",
    success: (res) => {
      if (res.confirm) {
        uni.setStorageSync("logout_flag", true);
        uni.removeStorageSync("user_creds");
        uni.removeStorageSync("userInfo");
        uni.removeStorageSync("userCookies");
        clearConfigCache();
        uni.reLaunch({
          url: "/pages/login/login",
        });
      }
    },
  });
};

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
    course: "/pages/course/course",
    score: "/pages/score/score",
    exam: "/pages/exam/exam",
    elective: "/pages/elective/elective",
    plan: "/pages/plan/plan",
    dekt: "/pages/dekt/dekt",
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

<style lang="scss">
@import "./styles/dashboard.scss";
</style>
