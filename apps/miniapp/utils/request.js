/**
 * 获取存储的 cookies
 * @returns {Object|null} cookies 对象或 null
 */
function getCookies() {
  try {
    const cookies = uni.getStorageSync("userCookies");
    return cookies || null;
  } catch (e) {
    return null;
  }
}

/**
 * 发起请求
 * @param {Object} options - uni.request 的配置选项
 * @returns {Promise}
 */
export function request(options) {
  return new Promise((resolve, reject) => {
    // 自动为 POST 请求注入 cookies (仅针对 /api/user/ 路径)
    if (
      options.method === "POST" &&
      options.url &&
      options.url.includes("/api/user/")
    ) {
      const cookies = getCookies();
      if (cookies) {
        // 将 cookies 注入到请求体中
        options.data = {
          ...options.data,
          cookies: cookies,
        };
      }
    }

    uni.request({
      ...options,
      success: (res) => {
        // 检查是否是会话过期错误
        if (
          res.statusCode === 400 &&
          (res.data?.detail === "未找到对应的会话" ||
            res.data?.message?.includes("会话") ||
            res.data?.message?.includes("登录"))
        ) {
          handleSessionExpired();
          reject(new Error("会话已过期"));
          return;
        }

        // 检查是否返回了 code !== 0 的错误
        if (
          res.data?.code !== 0 &&
          res.data?.message &&
          (res.data.message.includes("会话") ||
            res.data.message.includes("登录") ||
            res.data.message.includes("cookie"))
        ) {
          handleSessionExpired();
          reject(new Error(res.data.message));
          return;
        }

        // 调用原始的 success 回调
        if (options.success) {
          options.success(res);
        }
        resolve(res);
      },
      fail: (err) => {
        // 调用原始的 fail 回调
        if (options.fail) {
          options.fail(err);
        }
        reject(err);
      },
      complete: (res) => {
        if (options.complete) {
          options.complete(res);
        }
      },
    });
  });
}

function handleSessionExpired() {
  uni.showModal({
    title: "提示",
    content: "您的登录信息已过期，请重新登录",
    showCancel: false,
    confirmText: "确定",
    success: (res) => {
      if (res.confirm) {
        // 清除所有登录相关的存储
        uni.removeStorageSync("userInfo");
        uni.removeStorageSync("userCookies");
        uni.reLaunch({
          url: "/pages/login/login",
        });
      }
    },
  });
}

export default {
  request,
  getCookies,
};
