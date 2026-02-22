const CONFIG_STORAGE_KEY = "systemConfig";

export function getConfig(callback) {
  const cachedConfig = uni.getStorageSync(CONFIG_STORAGE_KEY);
  if (cachedConfig) {
    callback && callback(cachedConfig);
  } else {
    callback && callback(null);
  }
}

export function clearConfigCache() {
  uni.removeStorageSync(CONFIG_STORAGE_KEY);
}
