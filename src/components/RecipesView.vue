<script setup lang="ts">
import { computed, ref } from "vue";
import { findMode, findProcess } from "../processes";
import { recipeCatalog, searchRecipes } from "../recipes";

const emit = defineEmits<{
  openProcess: [processId: string, modeId: string];
}>();

const query = ref("");
const selectedId = ref(recipeCatalog[0]?.id ?? "");
const visible = computed(() => searchRecipes(query.value));
const selected = computed(
  () => visible.value.find((recipe) => recipe.id === selectedId.value) ?? visible.value[0],
);

function processTitle(processId: string): string {
  return findProcess(processId)?.title ?? processId;
}

function modeTitle(processId: string, modeId: string): string {
  const process = findProcess(processId);
  return (process && findMode(process, modeId)?.title) ?? modeId;
}

function onListKey(event: KeyboardEvent) {
  if (!visible.value.length) return;
  const index = visible.value.findIndex((recipe) => recipe.id === selected.value?.id);
  if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
  event.preventDefault();
  const next =
    event.key === "ArrowDown"
      ? (index + 1) % visible.value.length
      : (index - 1 + visible.value.length) % visible.value.length;
  selectedId.value = visible.value[next].id;
}
</script>

<template>
  <section class="recipes-workspace">
    <aside class="recipe-index" aria-label="Recipes">
      <div class="recipe-heading">
        <div>
          <p class="eyebrow">GUIDED SIGNAL PATHS</p>
          <h1>Combine processes.</h1>
          <p>Follow a proven route, then adapt it to your material.</p>
        </div>
        <span class="recipe-count">{{ String(recipeCatalog.length).padStart(2, "0") }}</span>
      </div>
      <label class="search-box recipe-search">
        <span aria-hidden="true">⌕</span>
        <input
          v-model="query"
          type="search"
          placeholder="Search recipes…"
          aria-label="Search recipes"
        />
      </label>
      <div
        class="recipe-list"
        tabindex="0"
        role="listbox"
        :aria-label="`${visible.length} recipes`"
        @keydown="onListKey"
      >
        <button
          v-for="(recipe, index) in visible"
          :key="recipe.id"
          class="recipe-row"
          :class="{ selected: selected?.id === recipe.id }"
          role="option"
          :aria-selected="selected?.id === recipe.id"
          @click="selectedId = recipe.id"
        >
          <span class="recipe-number">{{ String(index + 1).padStart(2, "0") }}</span>
          <span
            ><strong>{{ recipe.title }}</strong
            ><small>{{ recipe.summary }}</small></span
          >
          <span aria-hidden="true">→</span>
        </button>
        <div v-if="!visible.length" class="empty-state">
          <span>∅</span><strong>No matching recipes</strong>
          <p>Try a process, outcome, or technique.</p>
          <button class="text-button" @click="query = ''">Clear search</button>
        </div>
      </div>
    </aside>

    <article v-if="selected" class="recipe-detail" aria-live="polite">
      <header class="recipe-detail-header">
        <p class="eyebrow">RECIPE / {{ selected.steps.length }}-STAGE PATCH</p>
        <h2>{{ selected.title }}</h2>
        <p class="recipe-outcome">{{ selected.outcome }}</p>
        <div class="recipe-tags">
          <span v-for="tag in selected.tags" :key="tag">{{ tag }}</span>
        </div>
      </header>

      <section class="recipe-source">
        <span class="eyebrow">START WITH</span>
        <p>{{ selected.sourceGuidance }}</p>
      </section>

      <ol class="recipe-patch" aria-label="Recipe steps">
        <li v-for="(step, index) in selected.steps" :key="`${step.processId}-${step.modeId}`">
          <span class="patch-node" aria-hidden="true">{{ index + 1 }}</span>
          <div class="patch-stage">
            <div class="patch-title">
              <div>
                <small>{{ modeTitle(step.processId, step.modeId) }}</small>
                <h3>{{ processTitle(step.processId) }}</h3>
              </div>
              <button
                class="secondary-button"
                @click="emit('openProcess', step.processId, step.modeId)"
              >
                Open process <span aria-hidden="true">↗</span>
              </button>
            </div>
            <p>{{ step.instruction }}</p>
            <p class="patch-handoff"><span aria-hidden="true">↓</span>{{ step.handoff }}</p>
          </div>
        </li>
      </ol>

      <section v-if="selected.cautions.length" class="recipe-cautions">
        <span aria-hidden="true">i</span>
        <div>
          <strong>Before you begin</strong>
          <p v-for="caution in selected.cautions" :key="caution">{{ caution }}</p>
        </div>
      </section>
    </article>
  </section>
</template>
