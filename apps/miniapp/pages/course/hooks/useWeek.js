import { ref } from "vue";

export function useWeek() {
  const currentWeek = ref(1);
  const actualWeek = ref(0); // 实际的当前周（用于判断今日标识）

  const calcCurrentWeek = (startDateStr) => {
    if (!startDateStr) return 1;
    try {
      const startDate = new Date(startDateStr);
      const now = new Date();
      const diffTime = now - startDate;
      const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
      const week = Math.floor(diffDays / 7) + 1;
      return Math.max(1, Math.min(week, 25));
    } catch {
      return 1;
    }
  };

  const initWeek = (startDateStr) => {
    const week = calcCurrentWeek(startDateStr);
    currentWeek.value = week;
    actualWeek.value = week;
  };

  const prevWeek = () => {
    if (currentWeek.value > 1) currentWeek.value--;
  };

  const nextWeek = () => {
    if (currentWeek.value < 20) currentWeek.value++;
  };

  return {
    currentWeek,
    actualWeek,
    calcCurrentWeek,
    initWeek,
    prevWeek,
    nextWeek,
  };
}
