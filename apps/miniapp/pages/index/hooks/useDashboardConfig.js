import { ref, computed } from "vue";
import { getConfig } from "@/utils/configCache.js";
import { checkCacheVersion } from "@/utils/imageCache.js";

export function useDashboardConfig() {
  const noticeMessages = ref([]);
  const isNoticeExpanded = ref(false);
  const courseConfig = ref(null);

  const displayedNotices = computed(() => {
    if (isNoticeExpanded.value) return noticeMessages.value;
    return noticeMessages.value.slice(0, 3);
  });

  const toggleNoticeExpand = () => {
    isNoticeExpanded.value = !isNoticeExpanded.value;
  };

  const fetchSystemConfig = () => {
    getConfig((configData) => {
      if (!configData) return;
      const msgs = configData?.notice?.messages;
      if (msgs) {
        noticeMessages.value = Array.isArray(msgs) ? msgs : [msgs];
      } else {
        noticeMessages.value = [];
      }

      if (configData.course) {
        courseConfig.value = configData.course;
        const cacheVersion = configData.course.imageVersion;
        if (cacheVersion) checkCacheVersion(cacheVersion);
      }
    });
  };

  return {
    noticeMessages,
    isNoticeExpanded,
    displayedNotices,
    toggleNoticeExpand,
    courseConfig,
    fetchSystemConfig,
  };
}
