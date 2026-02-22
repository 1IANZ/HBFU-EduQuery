import { ref, onMounted } from "vue";
import API from "@/utils/api.js";
import { request } from "@/utils/request.js";

export function useDekt(studentIdRef) {
  const groups = ref([]);
  const refreshing = ref(false);
  const loading = ref(true);

  const selectedItem = ref(null);
  const detailData = ref({});
  const detailLoading = ref(false);

  const processData = (data) => {
    const list = data.list || [];
    const categoryMap = new Map();

    list.forEach((item) => {
      if (!categoryMap.has(item.category)) {
        categoryMap.set(item.category, {
          category: item.category,
          totalCredit: 0,
          items: [],
          isExpanded: false,
        });
      }
      const group = categoryMap.get(item.category);
      group.items.push(item);

      const credit = parseFloat(item.credit);
      if (!isNaN(credit)) {
        group.totalCredit += credit;
      }
    });

    groups.value = Array.from(categoryMap.values()).map((g) => ({
      ...g,
      totalCredit: Number.isInteger(g.totalCredit)
        ? g.totalCredit
        : g.totalCredit.toFixed(1),
    }));
  };

  const loadData = (showLoading = true) => {
    if (!studentIdRef.value) return;

    if (showLoading) {
      uni.showLoading({ title: "加载中..." });
      loading.value = true;
    }

    request({
      url: API.user.dekt(studentIdRef.value),
      method: "POST",
      data: {},
      success: (res) => {
        if (res.data.code === 0 && res.data.data) {
          processData(res.data.data);
        } else {
          uni.showToast({
            title: res.data.message || "获取失败",
            icon: "none",
          });
        }
      },
      fail: () => {
        uni.showToast({ title: "网络错误", icon: "none" });
      },
      complete: () => {
        if (showLoading) uni.hideLoading();
        refreshing.value = false;
        loading.value = false;
      },
    });
  };

  const toggleGroup = (index) => {
    groups.value[index].isExpanded = !groups.value[index].isExpanded;
  };

  const onRefresh = () => {
    refreshing.value = true;
    loadData(false);
  };

  const showDetail = (item) => {
    selectedItem.value = item;
    detailData.value = {};
    detailLoading.value = true;

    request({
      url: API.user.dektDetail(studentIdRef.value, item.operationId),
      method: "POST",
      data: {},
      success: (res) => {
        if (res.data.code === 0 && res.data.data) {
          detailData.value = res.data.data;
        } else {
          detailData.value = { 提示: "加载详情失败" };
        }
      },
      complete: () => {
        detailLoading.value = false;
      },
    });
  };

  const closeDetail = () => {
    selectedItem.value = null;
  };

  return {
    groups,
    refreshing,
    loading,
    selectedItem,
    detailData,
    detailLoading,
    loadData,
    toggleGroup,
    onRefresh,
    showDetail,
    closeDetail,
  };
}
