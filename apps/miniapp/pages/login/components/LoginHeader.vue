<template>
  <view class="header">
    <text class="title">河金小助手</text>
    <text class="subtitle">河北金融学院教务系统查询助手</text>
  </view>
</template>

<script setup></script>

<style lang="scss" scoped>
$text-primary: #1e293b;
$text-secondary: #64748b;

.header {
  text-align: center;
  margin-bottom: 40rpx;
  display: flex;
  flex-direction: column;
  align-items: center;

  .title {
    font-size: 44rpx;
    color: $text-primary;
    font-weight: 700;
    letter-spacing: -0.5px;
    margin-bottom: 8rpx;
  }

  .subtitle {
    font-size: 26rpx;
    color: $text-secondary;
  }
}
</style>
