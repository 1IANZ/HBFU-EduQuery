const CACHE_KEY = "img_cache_map";
const CACHE_VERSION_KEY = "img_cache_version";

/**
 * Get cached image path or download and cache it
 * @param {string} url - Remote image URL
 * @returns {Promise<string>} - Local file path
 */
export const getCachedImage = (url) => {
  return new Promise((resolve, reject) => {
    if (!url) {
      reject("URL is required");
      return;
    }

    const cacheMap = uni.getStorageSync(CACHE_KEY) || {};
    const cachedPath = cacheMap[url];

    // Helper to proceed to download
    const proceedToDownload = () => {
      downloadAndCache(url, resolve, reject);
    };

    if (cachedPath) {
      // Use FileSystemManager if available (WeChat/App)
      if (uni.getFileSystemManager) {
        const fs = uni.getFileSystemManager();
        fs.access({
          path: cachedPath,
          success: () => {
            resolve(cachedPath);
          },
          fail: (err) => {
            delete cacheMap[url];
            uni.setStorageSync(CACHE_KEY, cacheMap);
            proceedToDownload();
          },
        });
      } else {
        // Fallback for platforms without FileSystemManager (e.g. H5)
        uni.getSavedFileInfo({
          filePath: cachedPath,
          success: () => {
            resolve(cachedPath);
          },
          fail: () => {
            delete cacheMap[url];
            uni.setStorageSync(CACHE_KEY, cacheMap);
            proceedToDownload();
          },
        });
      }
    } else {
      proceedToDownload();
    }
  });
};

/**
 * Download file and save to persistent storage
 */
const downloadAndCache = (url, resolve, reject) => {
  uni.downloadFile({
    url: url,
    success: (res) => {
      if (res.statusCode === 200) {
        uni.saveFile({
          tempFilePath: res.tempFilePath,
          success: (saveRes) => {
            const savedPath = saveRes.savedFilePath;

            // Update cache map
            const cacheMap = uni.getStorageSync(CACHE_KEY) || {};
            cacheMap[url] = savedPath;
            uni.setStorageSync(CACHE_KEY, cacheMap);

            resolve(savedPath);
          },
          fail: (err) => {
            resolve(res.tempFilePath);
          },
        });
      } else {
        reject(`Download failed with status code ${res.statusCode}`);
      }
    },
    fail: (err) => {
      reject(err);
    },
  });
};

/**
 * Clear all cached images
 */
export const clearImageCache = () => {
  const cacheMap = uni.getStorageSync(CACHE_KEY) || {};
  const paths = Object.values(cacheMap);

  paths.forEach((path) => {
    uni.removeSavedFile({
      filePath: path,
      complete: () => {
        // Ignore errors
      },
    });
  });

  uni.removeStorageSync(CACHE_KEY);
};

/**
 * Check and update cache version. If version mismatch, clear cache.
 * @param {string} newVersion - The current version string (e.g. semester ID)
 */
export const checkCacheVersion = (newVersion) => {
  if (!newVersion) return;

  const currentVersion = uni.getStorageSync(CACHE_VERSION_KEY);

  if (currentVersion !== newVersion) {
    clearImageCache();
    uni.setStorageSync(CACHE_VERSION_KEY, newVersion);
  }
};
