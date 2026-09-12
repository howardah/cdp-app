# Choose the next 100 CDP processes

This backlog records the 100 processes selected for the second catalog expansion. Each entry now has a catalog manifest with at least one verified, safely representable mode. Complex operations may retain additional modes for later integration.

Implementation status: all 100 entries are registered in the frontend catalog, bundled Rust catalog, and Rust contract-test catalog. The application now contains 116 process definitions in total.

Before exposing an entry, follow the [process authoring guide](04-adding-processes.md). Verify its HTML reference against executable help, test every visible mode, record the actual CDP version, package its executable for every supported target, and add frontend, Rust, and staged-binary tests.

## Selection principles

The list uses four integration waves:

- **Wave 1**: common editing, spatial, delay, and filtering tasks with clear user outcomes
- **Wave 2**: envelope shaping, filter banks, repetition, and looping
- **Wave 3**: sequencing and mixing workflows that may require richer file relationships
- **Wave 4**: analysis-file processing for advanced spectral workflows

Complexity is an initial estimate. `Low` suggests that the current catalog vocabulary may be sufficient. `Medium` suggests multiple modes, data files, or new binaries. `High` suggests multi-output behavior, specialized file types, or substantial cross-field validation.

## Wave 1: editing and core effects

| # | Proposed catalog entry | CDP command family | User outcome | Local reference | Complexity |
| ---: | --- | --- | --- | --- | --- |
| 1 | Zero-crossing cut | `sfedit zcut` | Keep a mono segment with click-resistant zero-crossing cuts | `cgroedit.htm#ZCUT` | Low |
| 2 | Zero-crossing cuts | `sfedit zcuts` | Extract several mono segments at zero crossings | `cgroedit.htm#ZCUTS` | Medium |
| 3 | Excise multiple regions | `sfedit excises` | Remove several marked regions and close the gaps | `cgroedit.htm#EXCISES` | Low |
| 4 | Join numbered sequence | `sfedit joinseq` | Join a numbered set of soundfiles in sequence | `cgroedit.htm#JOINSEQ` | Medium |
| 5 | Dynamic join | `sfedit joindyn` | Join soundfiles with level control at each boundary | `cgroedit.htm#JOINDYN` | Medium |
| 6 | Noise cut | `sfedit noisecut` | Suppress quiet noise between wanted mono events | `cgroedit.htm#NOISECUT` | Medium |
| 7 | Random chunks | `sfedit randchunks` | Extract randomly positioned chunks from a sound | `cgroedit.htm#RANDCHUNKS` | High |
| 8 | TWixt | `sfedit twixt` | Switch between source files at specified times | `cgroedit.htm#TWIXT` | High |
| 9 | Sphinx | `sfedit sphinx` | Reassemble a sound by switching between related files | `cgroedit.htm#SPHINX` | High |
| 10 | Extract syllables | `sfedit syllables` | Separate vocal or event-like units into files | `cgroedit.htm#SYLLABLES` | High |
| 11 | Radical transformations | `modify radical` | Reverse, shred, scrub, degrade, or modulate sound | `cgromody.htm#RADICAL` | Medium |
| 12 | Convolve sounds | `modify convolve` | Apply one sound's character to another through convolution | `cgromody.htm#CONVOLVE` | Medium |
| 13 | Transposition stack | `modify stack` | Layer transposed copies into a chord or texture | `cgromody.htm#STACK` | Medium |
| 14 | Stereo shudder | `modify shudder` | Add dramatic, scattered stereo amplitude gestures | `cgromody.htm#SHUDDER` | Medium |
| 15 | Scaled pan | `modify scaledpan` | Scale a pan trajectory to the source duration | `cgromody.htm#SCALEDPAN` | Medium |
| 16 | Brassage | `modify brassage` | Reorder and transform small chunks of a sound | `cgromody.htm#BRASSAGE` | High |
| 17 | Sausage | `modify sausage` | Build dense textures from transformed sound segments | `cgromody.htm#SAUSAGE` | High |
| 18 | Spatial curve generator | `modify spaceform` | Generate a sinusoidal stereo movement curve | `cgromody.htm#SPACEFORM` | Medium |
| 19 | Find pan position | `modify findpan` | Estimate the stereo position represented by channel levels | `cgromody.htm#FINDPAN` | Low |
| 20 | Reverb | `reverb` | Add room-like reverberation to a soundfile | `cxreverb.htm` | Medium |
| 21 | Fixed filter | `filter fixed` | Boost or cut frequencies above, below, or around a point | `cgrofilt.htm#FIXED` | Low |
| 22 | Low-pass or high-pass filter | `filter lohi` | Remove frequencies above or below a cutoff | `cgrofilt.htm#LOHI` | Low |
| 23 | Variable filter | `filter variable` | Apply low-pass, high-pass, band-pass, or notch filtering | `cgrofilt.htm#VARIABLE` | Medium |
| 24 | Sweeping filter | `filter sweeping` | Move a resonant filter focus through the spectrum | `cgrofilt.htm#SWEEPING` | Medium |
| 25 | Filter phasing | `filter phasing` | Create phase-shift and phasing effects | `cgrofilt.htm#PHASING` | Medium |

## Wave 2: envelopes, filter banks, and repetition

| # | Proposed catalog entry | CDP command family | User outcome | Local reference | Complexity |
| ---: | --- | --- | --- | --- | --- |
| 26 | Phasor | `phasor` | Add a phasing effect to a mono signal | `cgrofilt.htm#PHASOR` | Medium |
| 27 | Iterated filter | `filter iterated` | Repeat sound with cumulative filtering | `cgrofilt.htm#ITERATED` | Medium |
| 28 | Filter bank | `filter bank` | Process sound through a bank of resonant filters | `cgrofilt.htm#BANK` | High |
| 29 | User filter bank | `filter userbank` | Apply a user-defined fixed filter bank | `cgrofilt.htm#USERBANK` | High |
| 30 | Variable filter bank | `filter varibank` | Apply a time-varying user-defined filter bank | `cgrofilt.htm#VARIBANK` | High |
| 31 | Emphasize attack | `envel attack` | Strengthen or reshape a sound's attack | `cgroenvl.htm#ATTACK` | Medium |
| 32 | Curtail with fade | `envel curtail` | Shorten a sound with a controlled fade | `cgroenvl.htm#CURTAIL` | Low |
| 33 | Dovetail | `envel dovetail` | Fade the beginning, end, or both ends of a sound | `cgroenvl.htm#DOVETAIL` | Low |
| 34 | Tremolo | `envel tremolo` | Apply periodic amplitude modulation | `cgroenvl.htm#TREMOLO` | Low |
| 35 | Cyclic envelope | `envel cyclic` | Repeat an envelope pattern across a sound | `cgroenvl.htm#CYCLIC` | Medium |
| 36 | Swell | `envel swell` | Fade into and out from a selected peak | `cgroenvl.htm#SWELL` | Medium |
| 37 | Pluck attack | `envel pluck` | Give a mono sound a short plucked onset | `cgroenvl.htm#PLUCK` | Medium |
| 38 | Warp envelope | `envel warp` | Reshape the amplitude contour of a sound | `cgroenvl.htm#WARP` | High |
| 39 | Impose envelope | `envel impose` | Apply an extracted envelope to another sound | `cgroenvl.htm#IMPOSE` | High |
| 40 | Replace envelope | `envel replace` | Replace a sound's amplitude contour | `cgroenvl.htm#REPLACE` | High |
| 41 | Extract envelope | `envel extract` | Save a sound's amplitude contour as envelope data | `cgroenvl.htm#EXTRACT` | Medium |
| 42 | Create envelope | `envel create` | Generate envelope data from breakpoint instructions | `cgroenvl.htm#CREATE` | Medium |
| 43 | Exponential decay | `envnu expdecay` | Apply a true exponential decay to silence | `cgroenvnu.htm#EXPDECAY` | Low |
| 44 | Peak chop | `envnu peakchop` | Isolate peaks from a source sound | `cgroenvnu.htm#PEAKCHOP` | High |
| 45 | Flatten events | `flatten` | Equalize the levels of separate sound events | `cgroenvl.htm#FLATTEN` | Medium |
| 46 | Back-to-back reverse | `extend baktobak` | Join a sound to a reversed copy | `cgroextd.htm#BAKTOBAK` | Low |
| 47 | Bounce | `bounce` | Create accelerating repeats that decay in level | `cgroextd.htm#BOUNCE` | Medium |
| 48 | Freeze segment | `extend freeze` | Extend a selected segment through fluid repetition | `cgroextd.htm#FREEZE` | Medium |
| 49 | Iterate sound | `extend iterate` | Repeat a sound with controlled variation | `cgroextd.htm#ITERATE` | Medium |
| 50 | Loop segments | `extend loop` | Repeat or advance through selected source segments | `cgroextd.htm#LOOP` | High |

## Wave 3: sequencing and mixing

| # | Proposed catalog entry | CDP command family | User outcome | Local reference | Complexity |
| ---: | --- | --- | --- | --- | --- |
| 51 | Drunken time walk | `extend drunk` | Wander forward and backward through a source | `cgroextd.htm#DRUNK` | Medium |
| 52 | Repeating echoes | `sfecho echo` | Repeat a sound with changing delay and level | `cgroextd.htm#ECHOES` | Medium |
| 53 | Hover | `hover` | Zigzag through a file at a controlled frequency | `cgroextd.htm#HOVER` | Medium |
| 54 | Inverted hover | `hover2` | Zigzag through a file using inverted copies | `cgroextd.htm#HOVER2` | Medium |
| 55 | Iteration line | `iterline` | Repeat one sound along a transposition line | `cgroextd.htm#ITERLINE` | High |
| 56 | Multi-source iteration line | `iterlinef` | Repeat several sounds along a transposition line | `cgroextd.htm#ITERLINEF` | High |
| 57 | Timed repetitions | `extend repetitions` | Repeat a source at specified times | `cgroextd.htm#REPETITIONS` | Medium |
| 58 | Sound sequence | `extend sequence` | Build a transposed sequence from a source | `cgroextd.htm#SEQUENCE` | High |
| 59 | Scramble sound | `extend scramble` | Reorder sound segments into new versions | `cgroextd.htm#SCRAMBLE` | High |
| 60 | Zigzag | `extend zigzag` | Read a sound backward and forward between positions | `cgroextd.htm#ZIGZAG` | Medium |
| 61 | Merge two sounds | `submix merge` | Mix two soundfiles with timing and gain controls | `cgromixr.htm#MERGE` | Medium |
| 62 | Merge many sounds | `submix mergemany` | Combine several soundfiles without authoring a mixfile | `cgromixr.htm#MERGEMANY` | Medium |
| 63 | Crossfade | `submix crossfade` | Fade smoothly between two soundfiles | `cgromixr.htm#CROSSFADE` | Medium |
| 64 | Balance sounds | `submix balance` | Move between two sounds with a balance contour | `cgromixr.htm#BALANCE` | Medium |
| 65 | Render mixfile | `submix mix` | Render a mix described by a CDP mixfile | `cgromixr.htm#MIX` | High |
| 66 | Generate in-betweens | `submix inbetween` | Create intermediate mixtures between two sounds | `cgromixr.htm#INBETWEEN` | High |
| 67 | Generate paired in-betweens | `submix inbetween2` | Create a second form of intermediate sound sequence | `cgromixr.htm#INBETWEEN2` | High |
| 68 | Interleave channels | `submix interleave` | Combine mono files as channels of one output | `cgromixr.htm#INTERLEAVE` | Medium |
| 69 | Pan mixfile | `submix pan` | Change spatial positions recorded in a mixfile | `cgromixr.htm#PAN` | High |
| 70 | Warp mix space | `submix spacewarp` | Transform spatial positions across a mix | `cgromixr.htm#SPACEWARP` | High |
| 71 | Synchronize mix | `submix sync` | Align sounds in a mixfile to timing markers | `cgromixr.htm#SYNC` | High |
| 72 | Synchronize attacks | `submix syncattack` | Align detected attacks across mixed sounds | `cgromixr.htm#SYNCATTACK` | High |
| 73 | Warp mix timing | `submix timewarp` | Transform event times in a mixfile | `cgromixr.htm#TIMEWARP` | High |
| 74 | Apply mix faders | `submix faders` | Mix files through independent level contours | `cgromixr.htm#FADERS` | High |
| 75 | Add sounds to mix | `submix addtomix` | Add soundfiles to an existing mix description | `cgromixr.htm#ADDTOMIX` | High |

## Wave 4: spectral transformation

| # | Proposed catalog entry | CDP command family | User outcome | Local reference | Complexity |
| ---: | --- | --- | --- | --- | --- |
| 76 | Spectral time stretch | `stretch time` | Change analysis-file duration without direct pitch transposition | `cstretch.htm#TIME` | Medium |
| 77 | Stretch spectrum | `stretch spectrum` | Expand or compress spectral frequencies | `cstretch.htm#SPECTRUM` | Medium |
| 78 | Artifact-reduced spectral stretch | `spectstr` | Time-stretch analysis data while suppressing artifacts | `cstretch.htm#SPECTSTR` | Medium |
| 79 | Average spectral channels | `blur avrg` | Average energy across neighboring spectral channels | `cblur.htm#AVRG` | Medium |
| 80 | Time blur spectrum | `blur blur` | Average spectral data over time | `cblur.htm#BLUR` | Medium |
| 81 | Spectral chorus | `blur chorus` | Add random frequency or amplitude variation | `cblur.htm#CHORUS` | Medium |
| 82 | Spectral drunken walk | `blur drunk` | Wander through analysis windows | `cblur.htm#DRUNK` | Medium |
| 83 | Add spectral noise | `blur noise` | Randomize spectral data to add noise-like character | `cblur.htm#NOISE` | Medium |
| 84 | Scatter spectrum | `blur scatter` | Randomly thin spectral components | `cblur.htm#SCATTER` | Medium |
| 85 | Shuffle spectrum | `blur shuffle` | Reorder analysis windows using a pattern | `cblur.htm#SHUFFLE` | High |
| 86 | Spread spectral peaks | `blur spread` | Broaden prominent peaks in the spectrum | `cblur.htm#SPREAD` | Medium |
| 87 | Suppress spectral peaks | `blur suppress` | Reduce the most prominent spectral channels | `cblur.htm#SUPPRESS` | Medium |
| 88 | Weave spectral windows | `blur weave` | Traverse analysis windows with a supplied pattern | `cblur.htm#WEAVE` | High |
| 89 | Cross spectra | `combine cross` | Replace one spectrum's amplitudes with another's | `ccombine.htm#CROSS` | High |
| 90 | Spectral difference | `combine diff` | Produce the difference between two spectra | `ccombine.htm#DIFF` | Medium |
| 91 | Interleave spectra | `combine interleave` | Alternate window groups from several analyses | `ccombine.htm#INTERLEAVE` | High |
| 92 | Spectral maximum | `combine max` | Keep the strongest value from two spectra | `ccombine.htm#MAX` | Medium |
| 93 | Spectral mean | `combine mean` | Create an averaged spectrum from two analyses | `ccombine.htm#MEAN` | Medium |
| 94 | Sum spectra | `combine sum` | Add two compatible spectra together | `ccombine.htm#SUM` | Medium |
| 95 | Accumulate spectrum | `focus accu` | Sustain spectral bands as their energy changes | `cfocus.htm#ACCU` | Medium |
| 96 | Exaggerate spectral contour | `focus exag` | Increase contrast in a spectrum's contour | `cfocus.htm#EXAG` | Medium |
| 97 | Focus spectral peaks | `focus focus` | Concentrate energy around prominent spectral peaks | `cfocus.htm#FOCUS` | Medium |
| 98 | Fold spectrum | `focus fold` | Fold spectral components by octave | `cfocus.htm#FOLD` | Medium |
| 99 | Freeze spectral characteristics | `focus freeze` | Hold selected spectral characteristics over time | `cfocus.htm#FREEZE` | High |
| 100 | Hold spectrum | `focus hold` | Hold a spectrum from a selected analysis time | `cfocus.htm#HOLD` | Medium |

## Integration gates

For each candidate:

1. Confirm that the operation appears in `cdpr8/docs/html/ccdpndex.htm` and read its detailed page.
2. Run the local executable with no arguments and operation-specific help arguments.
3. Record modes, ordered inputs, parameter order, flags, ranges, defaults, constraints, and output behavior.
4. Confirm that the existing catalog contract can describe the process without unsafe command fragments.
5. Package and smoke-test the required executable for every supported platform.
6. Add the manifest to both catalog entry points and add exact command-vector tests.

Do not treat the order above as a promise of release. Promote a candidate only after its executable behavior and output discovery pass the complete definition of done in the process authoring guide.
