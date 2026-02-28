<script setup>
import { onHide, onLaunch, onShow } from "@dcloudio/uni-app";

onLaunch(() => {
  try {
    const app = getApp();
    if (app && !app.globalData) {
      app.globalData = {};
    }
    if (app && app.globalData) {
      app.globalData.isColdStart = true;
    }
  } catch (e) {}

  if (uni.getUpdateManager) {
    const updateManager = uni.getUpdateManager();
    updateManager.onCheckForUpdate(function (res) {
      if (res.hasUpdate) {
        updateManager.onUpdateReady(function () {
          uni.showModal({
            title: "更新提示",
            content: "新版本已经准备好，是否重启应用？",
            showCancel: false,
            success: function (res) {
              if (res.confirm) {
                updateManager.applyUpdate();
              }
            },
          });
        });
        updateManager.onUpdateFailed(function () {
          uni.showModal({
            title: "已经有新版本了",
            content: "新版本已经上线啦~，请您删除当前小程序，重新搜索打开~",
            showCancel: false,
          });
        });
      }
    });
  }
});

onShow(() => {});

onHide(() => {});
</script>

<style>
page {
  background-color: #f8fafc;
  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial,
    sans-serif;
  color: #1e293b;
}

.blob {
  position: absolute;
  border-radius: 50%;
  z-index: -1;
  filter: blur(60rpx);
  opacity: 0.5;
  pointer-events: none;
}
.blob-1 {
  width: 700rpx;
  height: 700rpx;
  background: radial-gradient(circle, rgba(99, 102, 241, 0.2) 0%, rgba(99, 102, 241, 0) 70%);
  top: -150rpx;
  left: -200rpx;
  animation: float 14s infinite ease-in-out;
}
.blob-2 {
  width: 650rpx;
  height: 650rpx;
  background: radial-gradient(circle, rgba(20, 184, 166, 0.2) 0%, rgba(20, 184, 166, 0) 70%);
  bottom: -100rpx;
  right: -150rpx;
  animation: float 18s infinite ease-in-out reverse;
}
@keyframes float {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(60rpx, 60rpx) scale(1.05); }
}

/* Universal Glass Card Layout */
.glass-card {
  position: relative;
  z-index: 10;
  background: rgba(255, 255, 255, 0.65);
  backdrop-filter: blur(28px) saturate(120%);
  -webkit-backdrop-filter: blur(28px) saturate(120%);
  border-radius: 40rpx;
  border: 1px solid rgba(255, 255, 255, 0.9);
  box-shadow: 0 12rpx 32rpx rgba(148, 163, 184, 0.05);
  box-sizing: border-box;
}

/* Click Animation - active-scale */
.active-scale {
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1), filter 0.2s;
}
.active-scale:active {
  transform: scale(0.94);
  filter: brightness(0.95);
}
</style>
