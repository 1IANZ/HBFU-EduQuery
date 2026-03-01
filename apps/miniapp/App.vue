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
  --shadow-light: 0 1px 2px 0 rgba(0, 0, 0, 0.05);

  background-color: var(--bg-body);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  color: var(--text-main);
}

@media (prefers-color-scheme: dark) {
  page {
    /* Dark Mode Variables */
    --bg-body: #0f172a; /* slate-900 */
    --bg-card: #1e293b; /* slate-800 */
    --bg-card-glass: rgba(30, 41, 59, 0.65);
    --border-card: #334155; /* slate-700 */
    --border-glass: rgba(255, 255, 255, 0.1);
    --text-main: #f8fafc; /* slate-50 */
    --text-sub: #94a3b8; /* slate-400 */
    --shadow-light: 0 4px 6px -1px rgba(0, 0, 0, 0.5);
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
  transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1), filter 0.2s;
}
.active-scale:active {
  transform: scale(0.94);
  filter: brightness(0.95);
}
</style>
