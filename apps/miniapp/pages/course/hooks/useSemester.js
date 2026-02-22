import { ref } from "vue";
import API from "@/utils/api.js";
import { request } from "@/utils/request.js";
import { getConfig } from "@/utils/configCache.js";

export function useSemester(studentIdRef) {
  const semesterList = ref([]);
  const currentSemester = ref(null);

  const loadSemesters = (defaultSemesterValue = null, onLoaded) => {
    if (!studentIdRef.value) return;

    request({
      url: API.user.semester(studentIdRef.value, false),
      method: "POST",
      success: (res) => {
        if (res.data.code === 0 && res.data.data) {
          semesterList.value = res.data.data;

          let target = null;

          if (defaultSemesterValue) {
            target = semesterList.value.find(
              (s) => s.value === defaultSemesterValue,
            );
          }

          if (!target) {
            getConfig((config) => {
              const code =
                config?.data?.course?.semester || config?.course?.semester;
              target = semesterList.value.find((s) => s.value === code);
              currentSemester.value = target || semesterList.value[0] || null;
              onLoaded?.();
            });
            return;
          }

          currentSemester.value = target || semesterList.value[0] || null;
          onLoaded?.();
        }
      },
    });
  };

  return {
    semesterList,
    currentSemester,
    loadSemesters,
  };
}
