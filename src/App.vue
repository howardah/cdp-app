<script setup lang="ts">
import ProcessView from "./components/ProcessView.vue";
import RecipesView from "./components/RecipesView.vue";
import { useAppShell } from "./composables/useAppShell";
import { processCatalog } from "./processes";
import QueueBar from "./components/QueueBar.vue";
import AppHeader from "./components/AppHeader.vue";
const {
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
} = useAppShell();
</script>
<template>
  <UApp>
    <main
      class="app-shell"
      :class="{ 'process-window': route.name === 'process' }"
      :data-theme="theme"
    >
      <ProcessView v-if="route.name === 'process'" :process-id="currentProcessId" />
      <template v-else>
        <AppHeader
          :theme="theme"
          :section="route.name === 'recipes' ? 'recipes' : 'navigator'"
          @navigate="(section) => router.push({ name: section })"
          @toggle-theme="toggleTheme"
        />
        <div class="app-content">
          <RecipesView v-if="route.name === 'recipes'" @open-process="openSelected" />
          <template v-else>
            <label class="mobile-filter">
              <span>Process category</span>
              <select v-model="category">
                <option :value="undefined">All processes</option>
                <option v-for="item in categories" :key="item" :value="item">
                  {{ item.replaceAll("-", " ") }} ({{ categoryCounts[item] }})
                </option>
              </select>
            </label>
            <section class="workspace">
              <aside class="category-panel" aria-label="Process categories">
                <div class="panel-kicker">
                  CATALOG <span>R8 · {{ String(processCatalog.length).padStart(2, "0") }}</span>
                </div>
                <nav class="category-nav">
                  <button
                    class="category-item"
                    :class="{ active: !category }"
                    @click="category = undefined"
                  >
                    <span class="category-icon">◈</span><span>All processes</span
                    ><span class="category-count">{{ processCatalog.length }}</span></button
                  ><button
                    v-for="item in categories"
                    :key="item"
                    class="category-item"
                    :class="{ active: category === item }"
                    @click="category = item"
                  >
                    <span class="category-icon">{{
                      item === "spectral"
                        ? "∿"
                        : item === "edit-and-mix"
                          ? "⊞"
                          : item === "utilities"
                            ? "⌘"
                            : "⌁"
                    }}</span
                    ><span>{{ item.replaceAll("-", " ") }}</span
                    ><span class="category-count">{{ categoryCounts[item] || "—" }}</span>
                  </button>
                </nav>
                <div class="category-foot">
                  <div class="legend-title">FLOW STATUS</div>
                  <div><i class="legend-dot valid"></i> Ready to configure</div>
                  <div><i class="legend-dot idle"></i> Not selected</div>
                </div>
              </aside>
              <section class="process-panel" aria-label="Processes">
                <div class="panel-heading">
                  <div>
                    <div class="eyebrow">EXPLORE PROCESSES</div>
                    <h1>Find your next sound.</h1>
                  </div>
                  <button class="shortcut" @click="search?.focus()">
                    <kbd>⌘</kbd><kbd>K</kbd>
                  </button>
                </div>
                <label class="search-box"
                  ><span aria-hidden="true">⌕</span
                  ><input
                    ref="search"
                    v-model="query"
                    type="search"
                    placeholder="Search by name, outcome, or CDP term…"
                    aria-label="Search processes"
                  /><kbd>⌘ K</kbd></label
                >
                <div class="result-meta">
                  <span>{{ visible.length }} processes</span
                  ><span v-if="query || category">Filtered catalog</span>
                </div>
                <div
                  class="process-list"
                  tabindex="0"
                  role="listbox"
                  :aria-label="`${visible.length} processes`"
                  @keydown="onListKey"
                >
                  <button
                    v-for="item in visible"
                    :key="item.id"
                    class="process-row"
                    :class="{ selected: selected?.id === item.id }"
                    role="option"
                    :aria-selected="selected?.id === item.id"
                    @click="select(item.id)"
                  >
                    <span class="process-signal"></span
                    ><span class="process-copy"
                      ><strong>{{ item.title }}</strong
                      ><span>{{ item.summary }}</span></span
                    ><span class="type-badges"
                      ><em
                        v-for="type in [
                          ...new Set(
                            item.modes.flatMap((m) => m.inputs.flatMap((i) => i.fileTypes)),
                          ),
                        ]"
                        :key="type"
                        >{{ type }}</em
                      ></span
                    ><span class="row-arrow">→</span>
                  </button>
                  <div v-if="!visible.length" class="empty-state">
                    <span>∅</span><strong>No matching processes</strong>
                    <p>Try another term or clear the active filters.</p>
                    <button
                      class="text-button"
                      @click="
                        query = '';
                        category = undefined;
                      "
                    >
                      Clear filters
                    </button>
                  </div>
                </div>
              </section>
              <article v-if="selected" class="detail-panel" aria-live="polite">
                <div class="detail-top">
                  <span class="detail-category"
                    >{{ selected.category.replaceAll("-", " ") }} / PROCESS
                    {{
                      String(processCatalog.findIndex((p) => p.id === selected?.id) + 1).padStart(
                        2,
                        "0",
                      )
                    }}</span
                  >
                </div>
                <div class="detail-title">
                  <span class="detail-mark"></span>
                  <div>
                    <h2>{{ selected.title }}</h2>
                    <p>
                      {{ selected.identity.executable
                      }}<span v-if="selected.identity.operation">
                        · {{ selected.identity.operation }}</span
                      >
                    </p>
                  </div>
                </div>
                <div class="outcome-block">
                  <span class="eyebrow">WHAT IT DOES</span>
                  <p>{{ selected.description }}</p>
                </div>
                <div class="detail-section">
                  <span class="eyebrow">SIGNAL CONTRACT</span>
                  <div class="contract-row">
                    <div>
                      <small>INPUT</small
                      ><strong>{{
                        [
                          ...new Set(
                            selected.modes.flatMap((m) => m.inputs.flatMap((i) => i.fileTypes)),
                          ),
                        ].join(", ")
                      }}</strong>
                    </div>
                    <span class="contract-arrow">→</span>
                    <div>
                      <small>OUTPUT</small
                      ><strong>{{
                        [...new Set(selected.modes.map((m) => m.output.kind))].join(", ")
                      }}</strong>
                    </div>
                  </div>
                </div>
                <div class="detail-section">
                  <span class="eyebrow">AVAILABLE MODES</span>
                  <div class="mode-list">
                    <span v-for="mode in selected.modes" :key="mode.id"
                      ><i>{{ mode.cliMode ?? "—" }}</i
                      >{{ mode.title }}</span
                    >
                  </div>
                </div>
                <div class="detail-note">
                  <span>i</span>
                  <p>{{ selected.useCases.join(" · ") }}</p>
                </div>
                <button id="open-process" class="primary-button" @click="openSelected(selected.id)">
                  Open process <span>↗</span>
                </button>
              </article>
            </section>
          </template>
        </div>
        <QueueBar />
      </template>
    </main>
  </UApp>
</template>
