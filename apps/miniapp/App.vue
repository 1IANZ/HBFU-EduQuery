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

<style></style>
