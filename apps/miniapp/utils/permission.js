const whiteList = ["/pages/login/login", "pages/login/login"];

function hasPermission(url) {
  let path = url.split("?")[0];

  if (
    whiteList.includes(path) ||
    whiteList.some((item) => path.endsWith(item))
  ) {
    return true;
  }

  const token = uni.getStorageSync("userCookies");
  return !!token;
}

export default function initPermission() {
  const list = ["navigateTo", "redirectTo", "reLaunch", "switchTab"];

  list.forEach((item) => {
    uni.addInterceptor(item, {
      invoke(args) {
        if (!hasPermission(args.url)) {
          uni.reLaunch({
            url: "/pages/login/login",
          });
          return false;
        }
        return true;
      },
      fail(err) {
        console.error(err);
      },
    });
  });
}
