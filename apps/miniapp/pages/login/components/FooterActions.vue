<template>
  <view class="footer-buttons">
    <view class="contact-btn" @click="showAuthorContact">
      <text class="contact-text">联系作者</text>
    </view>
    <view class="contact-btn" @click="showUpdateLog">
      <text class="contact-text">更新日志</text>
    </view>
  </view>
</template>

<script setup>
import API from "@/utils/api.js";
import { getCachedImage } from "@/utils/imageCache.js";

const showAuthorContact = async () => {
  const remoteUrl = API.image.get("alex.png");
  let urlToPreview = remoteUrl;
  try {
    uni.showLoading({ title: "加载中..." });
    urlToPreview = await getCachedImage(remoteUrl);
  } catch (e) {
    console.error("Cache failed, using remote", e);
  } finally {
    uni.hideLoading();
  }
  uni.previewImage({ urls: [urlToPreview], current: 0 });
};

const showUpdateLog = () => {
  const remoteUrl = API.image.get("update.png");
  uni.previewImage({ urls: [remoteUrl], current: 0 });
};
</script>

<style lang="scss" scoped>
$text-secondary: #64748b;

.footer-buttons {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 40rpx;
  margin-top: 32rpx;
}
</style>
