export default {
  onShow() {
    // #ifdef MP-WEIXIN
    wx.showShareMenu({
      withShareTicket: true,
      menus: ["shareAppMessage", "shareTimeline"],
    });
    // #endif
  },
  onShareAppMessage() {
    return {
      title: "查询 课表 成绩 学分 考试安排",
      path: "/pages/login/login",
      imageUrl: "/static/share.png",
    };
  },
  onShareTimeline() {
    return {
      title: "河金小助手 - 查询 课表 成绩 学分 考试安排",
      imageUrl: "/static/share.png",
    };
  },
};
