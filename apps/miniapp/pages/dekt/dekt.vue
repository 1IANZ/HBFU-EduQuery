<template>
  <view class="container">
    <DektList
      class="list-container"
      :groups="groups"
      :refreshing="refreshing"
      :loading="loading"
      @refresh="onRefresh"
      @toggle-group="toggleGroup"
      @item-click="showDetail"
    />

    <DektDetailPopup
      :visible="!!selectedItem"
      :detail-data="detailData"
      :loading="detailLoading"
      @close="closeDetail"
    />
  </view>
</template>

<script setup>
import { onMounted, ref } from "vue";
import DektList from "./components/DektList.vue";
import DektDetailPopup from "./components/DektDetailPopup.vue";
import { useDekt } from "./hooks/useDekt.js";

const studentId = ref("");

const {
  groups,
  refreshing,
  loading,
  selectedItem,
  detailData,
  detailLoading,
  loadData,
  toggleGroup,
  onRefresh,
  showDetail,
  closeDetail,
} = useDekt(studentId);

onMounted(() => {
  const userInfo = uni.getStorageSync("userInfo");
  if (userInfo) {
    studentId.value = userInfo.studentId;
    loadData();
  } else {
    console.error("User info or studentId not found in storage");
    uni.showToast({ title: "用户信息缺失，请重新登录", icon: "none" });
  }
});
</script>

<style lang="scss">
@import "./styles/dekt.scss";
</style>
