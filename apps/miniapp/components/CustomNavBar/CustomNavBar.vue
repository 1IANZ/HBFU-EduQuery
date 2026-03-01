<template>
  <view class="nav-bar-wrapper" :style="{ backgroundColor: bgColor }">
    <view :style="{ height: statusBarHeight + 'px' }"></view>
    <view class="nav-bar-content" :style="{ height: navBarHeight + 'px' }">
      <view class="nav-left" @click="handleBack" v-if="showBack">
        <uni-icons type="left" size="20" :color="textColor" />
      </view>
      <view class="nav-left" v-else></view>
      <view class="nav-center">
        <text class="nav-title" :style="{ color: textColor }">
          {{ title }}
        </text>
      </view>
      <view class="nav-right" :style="{ width: menuButtonWidth + 'px' }"></view>
    </view>
  </view>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const props = defineProps({
  title: {
    type: String,
    default: ''
  },
  showBack: {
    type: Boolean,
    default: false
  },
  bgColor: {
    type: String,
    default: 'transparent'
  },
  textColor: {
    type: String,
    default: 'var(--text-main)'
  }
});

const statusBarHeight = ref(0);
const navBarHeight = ref(44);
const menuButtonWidth = ref(0);

onMounted(() => {
  const windowInfo = uni.getWindowInfo();
  statusBarHeight.value = windowInfo.statusBarHeight || 0;

  // #ifdef MP-WEIXIN
  const menuButtonInfo = uni.getMenuButtonBoundingClientRect();
  navBarHeight.value =
    (menuButtonInfo.top - windowInfo.statusBarHeight) * 2 +
    menuButtonInfo.height;

  menuButtonWidth.value =
    windowInfo.screenWidth - menuButtonInfo.right + menuButtonInfo.width;
  // #endif
  // #ifndef MP-WEIXIN
  menuButtonWidth.value = 80;
  // #endif
});

const handleBack = () => {
  uni.navigateBack();
};
</script>

<style lang="scss" scoped>
.nav-bar-wrapper {
  position: sticky;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  transition: background-color 0.3s ease;
}

.nav-bar-content {
  display: flex;
  align-items: center;
  padding-left: 32rpx;
  position: relative;
}

.nav-left {
  width: 60rpx;
  height: 100%;
  display: flex;
  align-items: center;
  flex-shrink: 0;
  z-index: 10;
}

.nav-center {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
  max-width: 60%;
  height: 100%;
}

.nav-title {
  font-size: 17px; 
  font-weight: 600;
  line-height: normal;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}


.nav-right {
  flex-shrink: 0;
  height: 100%;
  z-index: 10;
}
</style>