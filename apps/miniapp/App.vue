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
/* Global CSS Variable System */
page {
  /* Light Mode (Default) Variables */
  --bg-body: #f8fafc;
  --bg-card: #ffffff;
  --bg-card-glass: rgba(255, 255, 255, 0.65);
  --border-card: #f1f5f9;
  --border-glass: rgba(255, 255, 255, 0.9);
  --text-main: #1e293b;
  --text-sub: #64748b;
  --accent-color: #3b82f6;
  --shadow-light: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --custom-navbar-safe-height: calc(env(safe-area-inset-top) + 44px);

  background-color: var(--bg-body);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  color: var(--text-main);
}

@media (prefers-color-scheme: dark) {
  page {
    /* Dark Mode Variables — warm neutral grey, easy on eyes */
    --bg-body: #181818;
    --bg-card: #242424;
    --bg-card-glass: rgba(36, 36, 36, 0.80);
    --border-card: #333333;
    --border-glass: rgba(255, 255, 255, 0.08);
    --text-main: #f0f0f0;
    --text-sub: #909090;
    --accent-color: #3b82f6;
    --shadow-light: 0 4px 16px rgba(0, 0, 0, 0.4);
    --custom-navbar-safe-height: calc(env(safe-area-inset-top) + 44px);
  }
}

/* Universal Glass Card Layout */
.glass-card {
  position: relative;
  z-index: 10;
  background: var(--bg-card-glass);
  backdrop-filter: blur(28px) saturate(120%);
  -webkit-backdrop-filter: blur(28px) saturate(120%);
  border-radius: 40rpx;
  border: 1px solid var(--border-glass);
  box-shadow: var(--shadow-light);
  box-sizing: border-box;
}

/* Click Animation - active-scale */
.active-scale {
  transition: transform 0.15s ease-out, filter 0.15s ease-out;
}
.active-scale:active {
  transform: scale(0.96);
  filter: brightness(0.97);
}

/* Emotional Empty States (Global) */
.empty-state-card {
  margin: 60rpx 40rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80rpx 40rpx;
  background: var(--bg-card-glass);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border-radius: 48rpx;
  box-shadow: var(--shadow-light);
  border: 1px solid var(--border-glass);
  text-align: center;
}
.empty-emoji {
  font-size: 120rpx;
  line-height: 1.2;
  margin-bottom: 30rpx;
  filter: drop-shadow(0 20rpx 20rpx rgba(0,0,0,0.1));
}
.empty-title {
  font-size: 34rpx;
  font-weight: 700;
  color: var(--text-main);
  margin-bottom: 12rpx;
}
.empty-subtitle {
  font-size: 26rpx;
  color: var(--text-sub);
  line-height: 1.6;
}
</style>
