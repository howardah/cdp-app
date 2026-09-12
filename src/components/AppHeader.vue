<script setup lang="ts">
defineProps<{ theme: "light" | "dark"; section: "navigator" | "recipes" }>();
const emit = defineEmits<{ navigate: [section: "navigator" | "recipes"]; toggleTheme: [] }>();
</script>

<template>
  <header class="topbar">
    <div class="brand-lockup">
      <span class="brand-mark" aria-hidden="true">⌁</span
      ><span><strong>CDP</strong><small>DESKTOP STUDIO</small></span>
    </div>
    <nav class="section-switch" aria-label="Explore">
      <UButton
        variant="ghost"
        color="neutral"
        :class="{ active: section === 'navigator' }"
        :aria-current="section === 'navigator' ? 'page' : undefined"
        @click="emit('navigate', 'navigator')"
        >Processes</UButton
      >
      <UButton
        variant="ghost"
        color="neutral"
        :class="{ active: section === 'recipes' }"
        :aria-current="section === 'recipes' ? 'page' : undefined"
        @click="emit('navigate', 'recipes')"
        >Recipes</UButton
      >
    </nav>
    <div class="topbar-actions">
      <span class="status-dot"></span><span class="ready-label">Runtime ready</span>
      <UButton
        color="neutral"
        variant="ghost"
        class="icon-button"
        :aria-label="`Switch to ${theme === 'dark' ? 'light' : 'dark'} theme`"
        @click="emit('toggleTheme')"
        >{{ theme === "dark" ? "☼" : "☾" }}</UButton
      >
    </div>
  </header>
</template>
