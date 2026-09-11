# CDP Desktop experience and visual design

## Design intent

CDP Desktop should feel like a calm studio instrument: precise enough for long
technical sessions, but legible to someone encountering CDP for the first time.
The interface explains musical consequences before command-line mechanics. CDP
names, modes, units, and command previews remain visible as supporting evidence.

The design's signature is a signal-flow rail connecting **Input**, **Mode**,
**Parameters**, **Output**, and **Run**. It is structural rather than decorative:
each node shows incomplete, valid, queued, running, completed, or error state.
Motion is limited to one progress traversal along this rail and is removed when
the operating system requests reduced motion.

## Information architecture

### Main navigator window

Default size: 1180 × 760 px. Minimum size: 820 × 600 px.

```text
┌ Categories ─────┬ Processes ─────────────┬ Process details ───────────────┐
│ Search shortcut │ Search                  │ Modify Speed                   │
│ All processes   │ Speed and pitch         │ What it does                   │
│ Time domain     │ Adjust loudness         │ Inputs and outputs             │
│ Spectral        │ Join sounds             │ Modes and limitations          │
│ Edit and mix    │ Analyze spectrum        │                                │
│ Utilities       │ ...                     │              [Open process]    │
├─────────────────┴─────────────────────────┴────────────────────────────────┤
│ Queue: 1 running · 2 waiting                              [View queue]      │
└────────────────────────────────────────────────────────────────────────────┘
```

- The category column uses `UNavigationMenu` with counts. Categories come from
  the catalog and are not inferred from executable names.
- The process column uses a search `UInput` and a keyboard-navigable list. Each
  row contains a friendly title, one-line outcome, and compact input/output type
  badges. Cards are not used for every row.
- The details column shows plain-language purpose, use cases, supported modes,
  required inputs, produced outputs, cautions, exact CDP identity, and a link to
  bundled reference documentation.
- `Cmd/Ctrl+K` focuses global search. Up/down changes selection; Enter opens the
  selected process. The visible button remains the primary discoverable action.
- At the minimum width, categories collapse into a filter menu and the process
  list/detail areas become a two-panel splitter. This is desktop adaptation, not
  a separate mobile design.
- An empty search result names the active filters and offers **Clear filters**.

### Guided recipes

**Processes** and **Recipes** are peer destinations in the main-window header.
Recipes are curated signal paths, not automated chains: they explain why the
steps belong together, select a specific mode when opened, and rely on the
existing result-reuse flow for handing files between process windows.

```text
┌ Recipes ─────────────────┬ Pitch, then polish ──────────────────────────┐
│ Pitch, then polish       │ A pitch-shifted gesture at a controlled level │
│ Cut, reorder, reconnect  │                                               │
│ Equalise and assemble    │  1  Modify Speed · Semitone transposition     │
│ Spectral round trip      │  │  Choose an interval                  [Open]│
│                          │  ↓                                            │
│                          │  2  Modify Loudness · Normalise          [Open]│
└──────────────────────────┴───────────────────────────────────────────────┘
```

- The left pane provides a searchable, keyboard-navigable recipe index. The
  right pane explains the outcome, suitable source material, ordered steps,
  handoff instructions, and cautions.
- A patch-cord rail is the section's signature. Its numbered nodes encode real
  execution order; teal-to-amber cable color distinguishes handoff from the
  process form's live status rail without introducing a new palette.
- **Open process** validates the recipe's process and mode, then opens that mode
  with its safe catalog defaults. Recipe prose may suggest adjustments but does
  not silently prefill parameters.
- Recipe progress is not persisted, outputs are not transferred automatically,
  and closing a recipe does not affect queued or running work.
- The queue footer remains available in the Recipes section. At the 820 px
  minimum, the two panes contract while step actions remain visible and usable
  by keyboard.

### Native process windows

Default size: 760 × 820 px. Minimum size: 640 × 560 px. Each launch receives a
unique instance ID, so users may open the same process more than once.

```text
┌ Modify Speed · Semitones ────────────────────────────────────────────────┐
│ Changes pitch and duration together.                         [Reference] │
│                                                                          │
│  ● Input       [ Drop or choose a soundfile… ]                           │
│  │             mono · 48 kHz · 3.75 s                                   │
│  ● Mode        ( ) Speed ratio  (●) Semitones  ( ) Acceleration …        │
│  │                                                                       │
│  ● Parameters  Transposition [ -7.00 ] semitones                         │
│  │             ▸ Use a breakpoint file   ▸ Advanced                      │
│  ● Output      […/source-speed.wav]                         [Choose…]     │
│  │                                                                       │
│  ○ Run         Command preview ▸                                         │
├──────────────────────────────────────────────────────────────────────────┤
│ Ready                                                        [Run]       │
└──────────────────────────────────────────────────────────────────────────┘
```

- The header uses the friendly title first and the executable/operation name as
  secondary information.
- Each file row has a read-only path field, **Choose**, clear, and drag/drop
  affordances. Native Tauri dialogs provide actual filesystem paths; browser
  `File` objects are not used as the processing contract.
- Successful inspection adds type, duration, sample rate, channel count, and
  sample format below soundfile inputs. Analysis and data files show their
  recognized CDP type.
- Modes use `URadioGroup` when four or fewer choices are shown and `USelect`
  otherwise. Changing mode preserves values shared by stable parameter IDs and
  resets incompatible values to documented defaults after confirmation.
- Every input is wrapped by `UFormField`. Numeric parameters use
  `UInputNumber`; enumerations use `USelect`; flags use `USwitch` or `UCheckbox`.
- A scalar-or-breakpoint parameter starts as a numeric value. **Vary over time**
  switches it to a compatible file selector while retaining the scalar value if
  the user switches back.
- Advanced and infrequently used options live in `UCollapsible`. Required
  parameters, input selection, mode, and output never do.
- The sticky footer contains one solid primary action. `Cmd/Ctrl+Enter` runs a
  valid form. During execution it becomes **Cancel processing** with an error
  outline rather than adding a second equally weighted action.
- Closing a process window does not cancel its job. The main window's queue
  panel can reopen its status/results. Application quit requests graceful
  cancellation of the active child.

## Process guidance

Each process definition supplies four layers of explanation:

1. **Outcome:** one sentence describing the audible or file-level result.
2. **When to use it:** two or three concrete compositional tasks.
3. **Controls:** field descriptions in user language, including units and safe
   starting values.
4. **Technical detail:** exact CDP name, mode number, constraints, command
   preview, and bundled reference link.

Warnings are placed next to the decision that creates risk. Examples include
PVOC Analyze requiring mono input, speed changes altering both duration and
pitch, and Join requiring compatible sound formats. Information requiring user
action uses a persistent `UAlert`, not a toast.

## Queue and run states

| State | Rail/status behavior | Available action |
| --- | --- | --- |
| Incomplete | Unfilled nodes, field-level directions | Complete fields |
| Ready | All required nodes marked valid | Run |
| Queued | Position and preceding job shown | Cancel queued job |
| Running | Indeterminate progress and elapsed time | Cancel processing |
| Cancelling | Controls locked while child termination completes | None |
| Completed | Output node becomes the active result section | Play, reveal, reuse |
| Failed | Failing node and actionable message shown | Correct and run again |
| Cancelled | Values remain intact | Run again |

CDP tools do not provide a consistent percentage protocol. The first release
therefore shows indeterminate progress and elapsed time rather than fabricated
percentages. Raw stdout/stderr appears in a collapsed **Technical log**, with
path-like secrets limited to paths already chosen by the user.

The main navigator's queue action opens a `USlideover`. It lists the active job,
FIFO waiting jobs, and session-completed jobs. Waiting jobs can be cancelled but
not reordered in v1.

## Results and reuse

- A result section lists every artifact independently, even when a process uses
  a generic output root.
- Recognized soundfiles provide play/pause, seek, duration, and stop-on-window-
  close behavior using the WebView audio element. Waveform rendering is deferred.
- Analysis and text artifacts show their type and size but no audio control.
- **Reveal in Finder** highlights the artifact through Tauri's opener plugin.
- **Use as input** returns to the navigator, filters the catalog to processes
  accepting the artifact type, and preselects the artifact when a process opens.
- A missing or externally moved artifact is marked unavailable and offers
  **Locate file**; it is never silently removed from the session result.

## Validation and errors

- Validate on blur and on Run. Do not show errors on an untouched initial form.
- Frontend validation provides immediate guidance, but Rust repeats all
  validation before compiling a command.
- Output paths are suggested beside the primary input. If a path already exists,
  Run is blocked and **Use next available name** produces a suffixed path. V1
  never overwrites files.
- Messages use a stable code internally and plain language externally:
  **“This process needs a mono soundfile. Choose a mono file, or split the
  channels first.”** Technical details can name the rejected channel count.
- A failed run keeps every form value and path so the user can correct and retry.
- Toasts are reserved for non-blocking confirmation such as copying a command;
  errors and warnings remain inline.

## Visual system

### Type

- **Atkinson Hyperlegible**: navigation, headings, form labels, descriptions,
  buttons, and messages. Its letterforms support the newcomer-first goal.
- **IBM Plex Mono**: parameter values, units, modes, extensions, durations,
  command previews, and technical logs.
- Bundle only required WOFF2 weights with the app. Use system fallbacks while
  fonts load and never depend on a network font service.

### Palette and semantic mapping

| Token | Hex | Use |
| --- | --- | --- |
| Carbon | `#151A1E` | Dark canvas and strongest text in light mode |
| Slate | `#242C32` | Dark elevated surfaces |
| Mist | `#F2F5F5` | Light canvas and inverse text |
| Signal teal | `#2EB6A6` | Primary action, valid flow, active selection |
| Meter amber | `#DFA14A` | Queue, attention, caution |
| Fault red | `#D85C65` | Validation and processing failures |

Create full shade ramps for custom `brand` and `meter` colors in Tailwind's
`@theme`, then map Nuxt UI `primary` to `brand`, `warning` to `meter`, `error` to
the red ramp, and `neutral` to a cool neutral ramp. Components and application
markup use semantic utilities such as `bg-default`, `bg-elevated`,
`text-default`, `text-muted`, and `border-muted`; raw palette classes are limited
to the centralized theme definition.

Use one solid primary button per view. Secondary actions are neutral outline or
ghost variants. The base radius is 6 px: enough to separate controls without
making the application feel like a collection of soft cards. Dividers and
alignment carry most grouping; `UCard` is reserved for results or genuine
contained objects.

### Motion and sound

- Use 120–180 ms color and position transitions for direct manipulation.
- Animate only the signal-flow progress traversal during a run.
- Under `prefers-reduced-motion: reduce`, remove traversal and use static state
  changes.
- The application never plays a result automatically and does not add interface
  sound effects.

## Accessibility acceptance

- The complete navigator and process workflow works without a pointing device.
- Focus is visible in both color modes and returns to the invoking control after
  dialogs or overlays close.
- Every form control has a visible label; help and error IDs are associated via
  the Nuxt UI form field contract.
- Drag/drop is always paired with a choose-file button.
- Mode, queue, and run states use text and icons in addition to color.
- Technical monospace text is never smaller than 12 px; normal interface copy is
  never smaller than 14 px.
- Light and dark token combinations meet WCAG AA contrast for normal text and
  interactive boundaries.

## Nuxt UI implementation rules

- Wrap every route in `UApp`; this supplies toast, tooltip, and overlay contexts.
- Configure Nuxt UI through `@nuxt/ui/vite` and `@nuxt/ui/vue-plugin`; this is a
  standalone Vue application, not a Nuxt application.
- Prefer component variants and global theme configuration over repeated `ui`
  overrides. Inspect generated Vue theme files before overriding component slots.
- Use locally installed Lucide icons with `i-lucide-*` names.
- Use `UCommandPalette` for the optional command-style process finder, not as the
  only way to browse processes.
