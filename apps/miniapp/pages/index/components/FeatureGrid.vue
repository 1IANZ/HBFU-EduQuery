<template>
  <view class="grid-container">
    <FeatureCard
      v-for="item in items"
      :key="item.key"
      :title="item.title"
      :theme="item.theme"
      @click="item.onClick"
    >
      <template #icon>
        <uni-icons
          :type="item.icon"
          size="26"
          :color="item.color"
          v-bind="item.iconProps || {}"
        />
      </template>
    </FeatureCard>
  </view>
</template>

<script setup>
import FeatureCard from "./FeatureCard.vue";

defineProps({
  items: Array,
});
</script>

<style lang="scss" scoped>
.grid-container {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24rpx;
  padding-bottom: 40rpx;
}
</style>
