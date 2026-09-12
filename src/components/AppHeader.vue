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

<style scoped>
@reference "../styles.css";

.topbar {
  @apply flex h-18 items-center justify-between border-b border-muted px-7;
  container-type: inline-size;
}

.brand-lockup,
.topbar-actions,
.section-switch {
  @apply flex items-center;
}

.brand-mark {
  @apply mr-3 text-3xl leading-none text-primary;
}

.brand-lockup {
  & strong {
    @apply block text-sm font-bold tracking-[0.1em];
  }

  & small {
    @apply mt-0.5 block text-[10px] tracking-[0.13em] text-muted;
    font-family: "IBM Plex Mono", monospace;
  }
}

.section-switch {
  @apply gap-1 rounded-md border border-muted bg-muted p-1;

  & :deep(button) {
    @apply min-h-7 rounded-sm px-4 text-xs font-bold tracking-wide text-muted;
  }

  & :deep(button.active) {
    @apply bg-elevated text-primary ring-1 ring-muted;
  }
}

.topbar-actions {
  @apply gap-3 text-sm text-muted;
}

.status-dot {
  @apply size-2 rounded-full bg-primary ring-4 ring-primary/15;
}

.icon-button {
  @apply ml-4 text-xl text-muted;
}
</style>
