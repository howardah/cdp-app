import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { categories, processCatalog, searchCatalog } from "../processes";
import { openProcessWindow } from "../services/runtime";

export function useAppShell() {
  const route = useRoute();
  const router = useRouter();
  const query = ref("");
  const category = ref<string | undefined>();
  const selectedId = ref(processCatalog[0]?.id ?? "");
  const search = ref<HTMLInputElement | null>(null);
  const theme = ref<"light" | "dark">(
    (localStorage.getItem("cdp-theme") as "light" | "dark") || "dark",
  );
  watch(
    theme,
    (value) => {
      document.documentElement.classList.toggle("dark", value === "dark");
      document.documentElement.style.colorScheme = value;
      localStorage.setItem("cdp-theme", value);
    },
    { immediate: true },
  );
  const reuseType = computed(() =>
    typeof route.query.reuseType === "string" ? route.query.reuseType : "",
  );
  const visible = computed(() =>
    searchCatalog(query.value, category.value as (typeof categories)[number] | undefined).filter(
      (p) =>
        !reuseType.value ||
        p.modes.some((mode) =>
          mode.inputs.some((input) => input.fileTypes.includes(reuseType.value as never)),
        ),
    ),
  );
  const selected = computed(
    () =>
      processCatalog.find((item) => item.id === selectedId.value) ??
      visible.value[0] ??
      processCatalog[0],
  );
  const currentProcessId = computed(() =>
    typeof route.params.processId === "string" ? route.params.processId : "",
  );
  const categoryCounts = computed(() =>
    Object.fromEntries(
      categories.map((item) => [item, processCatalog.filter((p) => p.category === item).length]),
    ),
  );

  function select(id: string) {
    selectedId.value = id;
  }
  function toggleTheme() {
    theme.value = theme.value === "dark" ? "light" : "dark";
  }
  async function openSelected(id: string, modeId?: string) {
    try {
      if (await openProcessWindow(id, modeId)) return;
    } catch {
      // Browser preview has no native windows.
    }
    await router.push({
      name: "process",
      params: { processId: id },
      query: modeId ? { mode: modeId } : undefined,
    });
  }
  function onShortcut(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      search.value?.focus();
    }
  }
  function onListKey(event: KeyboardEvent) {
    if (!visible.value.length) return;
    const index = visible.value.findIndex((p) => p.id === selected.value?.id);
    if (event.key === "ArrowDown" || event.key === "ArrowUp") {
      event.preventDefault();
      const next =
        event.key === "ArrowDown"
          ? (index + 1) % visible.value.length
          : (index - 1 + visible.value.length) % visible.value.length;
      select(visible.value[next].id);
    }
    if (event.key === "Enter" && selected.value) void openSelected(selected.value.id);
  }
  onMounted(() => window.addEventListener("keydown", onShortcut));
  onUnmounted(() => window.removeEventListener("keydown", onShortcut));

  return {
    route,
    router,
    query,
    category,
    selected,
    search,
    theme,
    visible,
    currentProcessId,
    categoryCounts,
    select,
    toggleTheme,
    openSelected,
    onListKey,
    categories,
    processCatalog,
  };
}
