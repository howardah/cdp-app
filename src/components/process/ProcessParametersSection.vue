<script setup lang="ts">
import type { ModeDefinition, ParameterDefinition, ParameterValue } from "../../processes";
import ProcessParameterField from "./ProcessParameterField.vue";

defineProps<{
  mode: ModeDefinition;
  values: Record<string, ParameterValue>;
  issueFor: (field: string) => string | undefined;
}>();
defineEmits<{
  setNumber: [parameter: ParameterDefinition, value: string];
  setValue: [parameterId: string, value: ParameterValue];
  chooseBreakpoint: [parameter: ParameterDefinition];
  touched: [];
}>();
</script>

<template>
  <section class="form-section">
    <div class="section-heading">
      <span class="step-index">03</span>
      <div>
        <h2>Parameters</h2>
        <p>Safe starting values are prefilled; adjust them for your material.</p>
      </div>
    </div>
    <ProcessParameterField
      v-for="parameter in mode.parameters.filter((item) => !item.advanced)"
      :key="parameter.id"
      :parameter="parameter"
      :value="values[parameter.id]"
      :issue="issueFor(`parameter.${parameter.id}`)"
      @set-number="(item, value) => $emit('setNumber', item, value)"
      @set-value="(id, value) => $emit('setValue', id, value)"
      @choose-breakpoint="(item) => $emit('chooseBreakpoint', item)"
      @touched="$emit('touched')"
    />
    <details
      v-if="mode.parameters.some((parameter) => parameter.advanced)"
      class="advanced-parameters"
    >
      <summary>
        Advanced parameters
        <small>{{ mode.parameters.filter((parameter) => parameter.advanced).length }}</small>
      </summary>
      <ProcessParameterField
        v-for="parameter in mode.parameters.filter((item) => item.advanced)"
        :key="parameter.id"
        :parameter="parameter"
        :value="values[parameter.id]"
        :issue="issueFor(`parameter.${parameter.id}`)"
        @set-number="(item, value) => $emit('setNumber', item, value)"
        @set-value="(id, value) => $emit('setValue', id, value)"
        @choose-breakpoint="(item) => $emit('chooseBreakpoint', item)"
        @touched="$emit('touched')"
      />
    </details>
  </section>
</template>
