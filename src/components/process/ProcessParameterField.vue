<script setup lang="ts">
import type { ParameterDefinition, ParameterValue } from "../../processes";

const props = defineProps<{
  parameter: ParameterDefinition;
  value?: ParameterValue;
  issue?: string;
}>();
const emit = defineEmits<{
  setNumber: [parameter: ParameterDefinition, value: string];
  setValue: [parameterId: string, value: ParameterValue];
  chooseBreakpoint: [parameter: ParameterDefinition];
  touched: [];
}>();

const fieldId = `parameter-${props.parameter.id}`;
const errorId = `error-parameter-${props.parameter.id}`;
</script>

<template>
  <div class="field">
    <label :for="fieldId">
      {{ parameter.label }}
      <small v-if="parameter.unit">{{ parameter.unit }}</small>
    </label>
    <select
      v-if="parameter.kind === 'choice'"
      :id="fieldId"
      :value="value?.kind === 'choice' ? value.value : ''"
      :aria-describedby="issue ? errorId : undefined"
      @change="
        emit('setValue', parameter.id, {
          kind: 'choice',
          value: ($event.target as HTMLSelectElement).value,
        })
      "
      @blur="emit('touched')"
    >
      <option v-for="choice in parameter.choices" :key="choice.value" :value="choice.value">
        {{ choice.label }}
      </option>
    </select>
    <label v-else-if="parameter.kind === 'flag'" class="toggle">
      <input
        :id="fieldId"
        type="checkbox"
        :checked="value?.kind === 'flag' && value.value"
        :aria-describedby="issue ? errorId : undefined"
        @change="
          emit('setValue', parameter.id, {
            kind: 'flag',
            value: ($event.target as HTMLInputElement).checked,
          })
        "
        @blur="emit('touched')"
      />
      <span>Enable this option</span>
    </label>
    <template v-else-if="parameter.kind === 'numberOrBreakpoint'">
      <div class="parameter-toggle">
        <button
          type="button"
          :class="{ selected: value?.kind !== 'file' }"
          @click="
            emit(
              'setNumber',
              parameter,
              String(value?.kind === 'number' ? value.value : parameter.default.value),
            )
          "
        >
          Scalar
        </button>
        <button
          type="button"
          :class="{ selected: value?.kind === 'file' }"
          @click="emit('chooseBreakpoint', parameter)"
        >
          Breakpoint file
        </button>
      </div>
      <input
        v-if="value?.kind !== 'file'"
        :id="fieldId"
        type="number"
        :min="parameter.min"
        :max="parameter.max"
        :step="parameter.step"
        :value="value?.kind === 'number' ? value.value : ''"
        :aria-describedby="issue ? errorId : undefined"
        @input="emit('setNumber', parameter, ($event.target as HTMLInputElement).value)"
        @blur="emit('touched')"
      />
      <div v-else class="file-picker">
        <input :id="fieldId" :value="value.path" readonly />
        <button type="button" class="secondary-button" @click="emit('chooseBreakpoint', parameter)">
          Choose…
        </button>
      </div>
    </template>
    <input
      v-else
      :id="fieldId"
      type="number"
      :min="'min' in parameter ? parameter.min : undefined"
      :max="'max' in parameter ? parameter.max : undefined"
      :step="'step' in parameter ? parameter.step : undefined"
      :value="value?.kind === 'number' ? value.value : ''"
      :aria-describedby="issue ? errorId : undefined"
      @input="emit('setNumber', parameter, ($event.target as HTMLInputElement).value)"
      @blur="emit('touched')"
    />
    <p class="field-help">{{ parameter.description }}</p>
    <p v-if="issue" :id="errorId" class="field-error">{{ issue }}</p>
  </div>
</template>
