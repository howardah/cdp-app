export interface RecipeStep {
  processId: string;
  modeId: string;
  instruction: string;
  handoff: string;
}

export interface RecipeDefinition {
  id: string;
  title: string;
  summary: string;
  outcome: string;
  sourceGuidance: string;
  tags: string[];
  cautions: string[];
  steps: RecipeStep[];
}
