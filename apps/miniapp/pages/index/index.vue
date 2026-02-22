<template>
  <view class="dashboard-container">
    <view class="content-wrapper">
      <DashboardHeader
        :student-name="studentName"
        :student-id="studentId"
        @profile="navigateTo('profile')"
        @logout="handleLogout"
      />

      <scroll-view scroll-y class="scroll-container">
        <view class="section-header">
          <text class="section-title">ACADEMIC</text>
          <view class="section-line"></view>
        </view>

        <FeatureGrid :items="featureItems" />

        <NoticeList
          :messages="noticeMessages"
          :displayed="displayedNotices"
          :expanded="isNoticeExpanded"
          @toggle="toggleNoticeExpand"
        />
      </scroll-view>
    </view>
  </view>
</template>

<script setup>
import { onBackPress, onShow } from "@dcloudio/uni-app";
import { getCachedImage } from "@/utils/imageCache.js";
import API from "@/utils/api.js";
import { clearConfigCache } from "@/utils/configCache.js";

import DashboardHeader from "./components/DashboardHeader.vue";
import FeatureGrid from "./components/FeatureGrid.vue";
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

const featureItems = [
  {
    key: "course",
    title: "课程安排",
    icon: "calendar-filled",
    color: "#3b82f6",
    theme: "theme-blue",
    onClick: () => navigateTo("course"),
  },
  {
    key: "score",
    title: "成绩查询",
    icon: "medal-filled",
    color: "#10b981",
    theme: "theme-green",
    onClick: () => navigateTo("score"),
  },
  {
    key: "exam",
    title: "考试安排",
    icon: "compose",
    color: "#ef4444",
    theme: "theme-red",
    onClick: () => navigateTo("exam"),
  },
  {
    key: "elective",
    title: "选课中心",
    icon: "paperplane-filled",
    color: "#8b5cf6",
    theme: "theme-purple",
    onClick: () => navigateTo("elective"),
  },
  {
    key: "plan",
    title: "执行计划",
    icon: "map-filled",
    iconProps: { transform: "rotate(180)" },
    color: "#6366f1",
    theme: "theme-indigo",
    onClick: () => navigateTo("plan"),
  },
  {
    key: "dekt",
    title: "第二课堂",
    icon: "flag-filled",
    color: "#f59e0b",
    theme: "theme-orange",
    onClick: () => navigateTo("dekt"),
  },
  {
    key: "calendar",
    title: "学校校历",
    icon: "calendar",
    color: "#14b8a6",
    theme: "theme-teal",
    onClick: () => showImage("xiaoli.png"),
  },
  {
    key: "map",
    title: "学校地图",
    icon: "map-pin-ellipse",
    color: "#06b6d4",
    theme: "theme-cyan",
    onClick: () => showImage("ditu.png"),
  },
];

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
