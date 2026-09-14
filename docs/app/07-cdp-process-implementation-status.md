# CDP Release 8 process implementation status

Last audited: 2026-09-14

This document compares the processes exposed by the CDP Release 8 distribution in `cdpr8` with the process definitions currently exposed in the Composers Desktop GUI.

## How status is determined

- ✅ **Implemented in GUI** means a matching JSON process definition exists in `src/processes/definitions`, is registered in `src/processes/catalog.ts`, and can therefore be rendered by the generic process UI.
- ⬜ **Not implemented in GUI** means the process appears in CDP's local master index but has no matching GUI definition.
- The CDP source of truth for the inventory is `cdpr8/docs/html/ccdpndex.htm`. The installed `cdpr8/_cdp/_cdprogs` directory contains 230 executable files, but executables are not equivalent to user-facing processes: many executables contain several operations or modes, and some are support utilities.
- This is a source/catalog audit, not a claim that every implemented process has been manually smoke-tested on every platform. Packaging should be checked separately for release readiness.

## Summary

| Measure | Count |
| --- | ---: |
| GUI process definitions | 220 |
| CDP executable files in the bundled Release 8 folder | 230 |
| Available entries in the grouped CDP process index | 539 |
| Indexed rows matched to GUI definitions | 223 |
| Distinct GUI definitions found in the CDP index | 220 |

The index count is not a unique-process denominator. CDP repeats some cross-references in multiple groups and lists some modes as separate rows (for example RETIME and SPECFNU). The withdrawn `(ALIAS)` entry is excluded. The authoritative GUI count is the 220 manifest files.

## Implemented GUI catalog

| GUI ID | Display name | CDP command | Visible modes |
| --- | --- | --- | --- |
| `blur-avrg` | Average Spectral Channels | `blur avrg` | Average |
| `blur-blur` | Time Blur Spectrum | `blur blur` | Blur |
| `blur-chorus` | Spectral Chorus | `blur chorus` | Randomize amplitudes |
| `blur-drunk` | Spectral Drunken Walk | `blur drunk` | Walk |
| `blur-noise` | Add Spectral Noise | `blur noise` | Noise |
| `blur-scatter` | Scatter Spectrum | `blur scatter` | Scatter |
| `blur-shuffle` | Shuffle Spectral Windows | `blur shuffle` | Shuffle |
| `blur-spread` | Spread Spectral Peaks | `blur spread` | Formant-wise spread |
| `blur-suppress` | Suppress Spectral Peaks | `blur suppress` | Suppress |
| `blur-weave` | Weave Spectral Windows | `blur weave` | Weave |
| `bounce` | Bounce | `bounce bounce` | bounce |
| `combine-cross` | Cross Spectra | `combine cross` | Cross |
| `combine-diff` | Spectral Difference | `combine diff` | Difference |
| `combine-interleave` | Interleave Spectra | `combine interleave` | Interleave |
| `combine-max` | Spectral Maximum | `combine max` | Maximum |
| `combine-mean` | Spectral Mean | `combine mean` | Mean |
| `combine-sum` | Sum Spectra | `combine sum` | Sum |
| `envel-attack` | Envel Attack | `envel attack` | attack |
| `envel-brktoenv` | Envelope Breakpoint to Binary Envelope | `envel brktoenv` | brktoenv |
| `envel-create` | Envel Create | `envel create` | binary |
| `envel-curtail` | Envel Curtail | `envel curtail` | fade |
| `envel-cyclic` | Envel Cyclic | `envel cyclic` | rising |
| `envel-dbtoenv` | Envelope dB Breakpoint to Binary Envelope | `envel dbtoenv` | dbtoenv |
| `envel-dovetail` | Envel Dovetail | `envel dovetail` | fade |
| `envel-extract` | Envel Extract | `envel extract` | binary |
| `envel-impose` | Envel Impose | `envel impose` | binary |
| `envel-pluck` | Envel Pluck | `envel pluck` | pluck |
| `envel-replace` | Envel Replace | `envel replace` | binary |
| `envel-swell` | Envel Swell | `envel swell` | swell |
| `envel-tremolo` | Envel Tremolo | `envel tremolo` | linear |
| `envel-warp` | Envel Warp | `envel warp` | normalise |
| `envnu-expdecay` | Envnu Exponential Decay | `envnu expdecay` | decay |
| `envnu-peakchop` | Envnu Peak Chop | `envnu peakchop` | envelope |
| `extend-baktobak` | Extend Back-to-Back | `extend baktobak` | baktobak |
| `extend-doublets` | Extend Doublets | `extend doublets` | Default |
| `extend-drunk` | Drunken Time Walk | `extend drunk` | Drunken walk |
| `extend-freeze` | Extend Freeze | `extend freeze` | duration |
| `extend-iterate` | Extend Iterate | `extend iterate` | duration |
| `extend-loop` | Extend Loop | `extend loop` | advance |
| `extend-repetitions` | Timed Repetitions | `extend repetitions` | Repeat at times |
| `extend-scramble` | Scramble Sound | `extend scramble` | Random chunks |
| `extend-sequence` | Sound Sequence | `extend sequence` | Sequence |
| `extend-zigzag` | Zigzag | `extend zigzag` | Random zigzags |
| `filter-bank` | Filter bank | `filter bank` | bank |
| `filter-fixed` | Filter fixed | `filter fixed` | fixed |
| `filter-iterated` | Filter iterated | `filter iterated` | iterated |
| `filter-low-high` | Filter low-high | `filter lohi` | low-high |
| `filter-phasing` | Filter phasing | `filter phasing` | phasing |
| `filter-sweeping` | Filter sweeping | `filter sweeping` | sweeping |
| `filter-userbank` | Filter userbank | `filter userbank` | userbank |
| `filter-variable` | Filter variable | `filter variable` | variable |
| `filter-varibank` | Filter varibank | `filter varibank` | varibank |
| `flatten` | Flatten | `flatten flatten` | flatten |
| `focus-accu` | Accumulate Spectrum | `focus accu` | Accumulate |
| `focus-exag` | Exaggerate Spectral Contour | `focus exag` | Exaggerate |
| `focus-focus` | Focus Spectral Peaks | `focus focus` | Focus by frequency |
| `focus-fold` | Fold Spectrum | `focus fold` | Fold |
| `focus-freeze` | Freeze Spectral Characteristics | `focus freeze` | Freeze amplitudes |
| `focus-hold` | Hold Spectrum | `focus hold` | Hold |
| `focus-step` | Focus Step | `focus step` | Default |
| `hover` | Hover | `hover hover` | Hover |
| `hover2` | Inverted Hover | `hover2 hover2` | Inverted hover |
| `isolate` | Isolate | `isolate` | One segment per output; Grouped segments; Level threshold; Slice file; Dovetail slices |
| `iterline` | Iteration Line | `iterline iterline` | Interpolate |
| `iterlinef` | Multi-Source Iteration Line | `iterlinef iterlinef` | Interpolate |
| `modify-brassage` | Modify Brassage | `modify brassage` | Pitch shift grains |
| `modify-convolve` | Modify Convolve | `modify convolve` | Convolve sounds |
| `modify-findpan` | Modify Find Pan | `modify findpan` | Report pan position |
| `modify-loudness` | Modify Loudness | `modify loudness` | Linear gain; Decibel gain; Normalise; Force level; Balance; Invert phase; Find loudest; Equalise |
| `modify-radical` | Modify Radical | `modify radical` | Reverse sound |
| `modify-revecho` | Modify Reverb Echo | `modify revecho` | Standard delay; Modulated delay; Stadium echo |
| `modify-sausage` | Modify Sausage | `modify sausage` | Granulate sources |
| `modify-scaledpan` | Modify Scaled Pan | `modify scaledpan` | Scale pan trajectory |
| `modify-shudder` | Modify Shudder | `modify shudder` | Shudder stereo sound |
| `modify-space` | Modify Space | `modify space` | Pan mono; Mirror stereo; Narrow stereo |
| `modify-spaceform` | Modify Spaceform | `modify spaceform` | Generate pan curve |
| `modify-speed` | Modify Speed | `modify speed` | Speed ratio; Semitone transposition; Accelerate or decelerate; Vibrato |
| `modify-stack` | Modify Stack | `modify stack` | Stack transposed copies |
| `phasor` | Phasor | `phasor phasor` | phasor |
| `pvoc-analyze` | PVOC Analyze | `pvoc anal` | Analyze sound |
| `pvoc-synthesize` | PVOC Synthesize | `pvoc synth` | Synthesize sound |
| `reverb` | Reverb | `reverb reverb` | reverb |
| `sfecho-echo` | Repeating Echoes | `sfecho echo` | Echo |
| `sfedit-cut` | SFEdit Cut | `sfedit cut` | Time in seconds; Sample count; Grouped samples |
| `sfedit-cutend` | SFEdit Cut End | `sfedit cutend` | Duration in seconds; Sample count; Grouped samples |
| `sfedit-cutmany` | SFEdit Cut Many | `sfedit cutmany` | Extract ranges; Extract ranges; Extract ranges |
| `sfedit-excise` | SFEdit Excise | `sfedit excise` | Remove a range; Remove a range; Remove a range |
| `sfedit-excises` | SFEdit Excise Multiple | `sfedit excises` | Remove marked ranges; Remove marked ranges; Remove marked ranges |
| `sfedit-insert` | SFEdit Insert | `sfedit insert` | Insert a sound; Insert a sound; Insert a sound |
| `sfedit-insil` | SFEdit Insert Silence | `sfedit insil` | Insert a silent interval; Insert a silent interval; Insert a silent interval |
| `sfedit-join` | SFEdit Join | `sfedit join` | Join sounds |
| `sfedit-joindyn` | SFEdit Dynamic Join | `sfedit joindyn` | dynamic |
| `sfedit-joinseq` | SFEdit Join Sequence | `sfedit joinseq` | sequence |
| `sfedit-masks` | SFEdit Masks | `sfedit masks` | Time in seconds; Sample count; Grouped samples |
| `sfedit-noisecut` | SFEdit Noise Cut | `sfedit noisecut` | Suppress noise |
| `sfedit-randchunks` | SFEdit Random Chunks | `sfedit randchunks` | chunks |
| `sfedit-replace` | SFEdit Replace | `sfedit replace` | Replace a range; Replace a range; Replace a range |
| `sfedit-sphinx` | SFEdit Sphinx | `sfedit sphinx` | sequence |
| `sfedit-syllables` | SFEdit Syllables | `sfedit syllables` | seconds |
| `sfedit-twixt` | SFEdit Twixt | `sfedit twixt` | sequence |
| `sfedit-zcut` | SFEdit Zero-Crossing Cut | `sfedit zcut` | Time in seconds; Sample count |
| `sfedit-zcuts` | SFEdit Zero-Crossing Cuts | `sfedit zcuts` | Time in seconds; Sample count |
| `spectstr` | Artifact-Reduced Spectral Stretch | `spectstr stretch` | Stretch |
| `stretch-spectrum` | Stretch Spectrum | `stretch spectrum` | Stretch above frequency |
| `stretch-time` | Spectral Time Stretch | `stretch time` | Stretch |
| `submix-addtomix` | Add Sounds to Mix | `submix addtomix` | Add sounds |
| `submix-balance` | Balance Two Sounds | `submix balance` | Balance |
| `submix-crossfade` | Crossfade Sounds | `submix crossfade` | Linear crossfade |
| `submix-faders` | Apply Mix Faders | `submix faders` | Faders |
| `submix-inbetween` | Generate In-Betweens | `submix inbetween` | Even steps |
| `submix-inbetween2` | Generate Cycle-Aligned In-Betweens | `submix inbetween2` | Interpolate |
| `submix-interleave` | Interleave Channels | `submix interleave` | Interleave |
| `submix-merge` | Merge Two Sounds | `submix merge` | Merge |
| `submix-mergemany` | Merge Many Sounds | `submix mergemany` | Merge |
| `submix-mix` | Render Mixfile | `submix mix` | Render |
| `submix-pan` | Pan Mixfile | `submix pan` | Pan |
| `submix-spacewarp` | Warp Mix Space | `submix spacewarp` | Alternate stereo |
| `submix-sync` | Synchronize Mix | `submix sync` | Synchronize |
| `submix-syncattack` | Synchronize Attacks | `submix syncattack` | Synchronize attacks |
| `submix-timewarp` | Warp Mix Timing | `submix timewarp` | Time warp |
| `combine-make` | Build Spectrum from Pitch and Formants | `combine make` | Build analysis |
| `combine-make2` | Build Spectrum with Envelope | `combine make2` | Build analysis |
| `envel-dbtogain` | Convert Envelope dB to Gain | `envel dbtogain` | Convert to gain |
| `envel-envtobrk` | Convert Binary Envelope to Breakpoints | `envel envtobrk` | Export breakpoints |
| `envel-envtodb` | Convert Binary Envelope to dB Breakpoints | `envel envtodb` | Export dB breakpoints |
| `envel-gaintodb` | Convert Envelope Gain to dB | `envel gaintodb` | Convert to dB |
| `envel-reshape` | Normalise Binary Envelope | `envel reshape` | Normalise |
| `envel-replot` | Normalise Breakpoint Envelope | `envel replot` | Normalise |
| `envel-scaled` | Impose Scaled Breakpoint Envelope | `envel scaled` | Apply envelope |
| `envel-timegrid` | Partition Sound into Time Grids | `envel timegrid` | Create grids |
| `filter-bankfrqs` | Generate Harmonic Filter Frequencies | `filter bankfrqs` | Harmonic series |
| `filter-vfilters` | Create Fixed-Pitch Varibank Files | `filter vfilters` | Create filter files |
| `submix-atstep` | Create Stepped Mixfile | `submix atstep` | Create stepped mix |
| `submix-attenuate` | Adjust Mixfile Level | `submix attenuate` | Adjust level |
| `submix-dummy` | Create Basic Mixfile | `submix dummy` | Start together; Place sequentially |
| `submix-getlevel` | Check Mix Peak Level | `submix getlevel` | Report peak |
| `submix-model` | Replace Mixfile Sources | `submix model` | Replace sources |
| `submix-ongrid` | Create Grid Mixfile | `submix ongrid` | Create grid mix |
| `submix-shuffle` | Duplicate Mix Entries | `submix shuffle` | Duplicate entries |
| `submix-test` | Validate Mixfile | `submix test` | Validate syntax |

## Full CDP process index

The labels below preserve the terminology used by CDP's bundled documentation. A repeated label remains repeated when CDP places it in more than one functional group.

### BLUR (13)

✅ BLUR AVRG · ✅ BLUR BLUR · ⬜ CALTRAIN · ✅ BLUR CHORUS · ✅ BLUR DRUNK · ✅ BLUR NOISE · ✅ BLUR SCATTER · ⬜ SELFSIM · ✅ BLUR SHUFFLE · ✅ BLUR SPREAD · ✅ BLUR SUPPRESS · ⬜ SUPPRESS PARTIALS · ✅ WEAVE

### COMBINE (11)

✅ COMBINE CROSS · ✅ COMBINE DIFF · ✅ COMBINE INTERLEAVE · ✅ COMBINE MAKE · ✅ COMBINE MAKE2 · ✅ COMBINE MAX · ✅ COMBINE MEAN · ⬜ SPECROSS · ⬜ SPECSPHINX · ⬜ SPECTWIN · ✅ COMBINE SUM

### DISTORT (38)

✅ DISTORT AVERAGE · ✅ CLIP · ✅ DISTORT CYCLECNT · ✅ DISTORT DELETE · ⬜ DISTCUT · ⬜ DISTMARK · ⬜ DISTMORE BRIGHT · ⬜ DISTMORE DOUBLE · ⬜ DISTMORE SEGSBKWD · ⬜ DISTMORE SEGZIG · ⬜ DISTORTT · ⬜ DISTREP · ⬜ DISTSHIFT · ⬜ DISTWARP · ✅ DISTORT DIVIDE · ✅ DISTORT ENVEL · ✅ DISTORT FILTER · ✅ DISTORT FRACTAL · ⬜ FRACTAL WAVE · ✅ DISTORT HARMONIC · ✅ DISTORT INTERACT · ✅ DISTORT INTERPOLATE · ✅ DISTORT MULTIPLY · ✅ DISTORT OMIT · ✅ DISTORT OVERLOAD · ✅ DISTORT PITCH · ✅ DISTORT PULSED · ⬜ QUIRK · ✅ DISTORT REFORM · ✅ DISTORT REPEAT · ✅ DISTORT REPEAT2 · ✅ DISTORT REPLACE · ✅ DISTORT REPLIM · ✅ DISTORT REVERSE · ⬜ SCRAMBLE · ✅ DISTORT SHUFFLE · ⬜ SPLINTER · ✅ DISTORT TELESCOPE

### ENVEL (27)

✅ ENVEL ATTACK · ✅ ENVEL BRKTOENV · ✅ ENVEL CREATE · ✅ ENVEL CURTAIL · ✅ ENVEL CYCLIC · ✅ ENVEL DBTOENV · ✅ ENVEL DBTOGAIN · ✅ ENVEL DOVETAIL · ✅ ENVEL ENVTOBRK · ✅ ENVEL ENVTODB · ✅ ENVEL EXTRACT · ✅ FLATTEN · ✅ ENVEL GAINTODB · ✅ ENVEL IMPOSE · ✅ ENVEL PLUCK · ✅ ENVEL REPLACE · ✅ ENVEL RESHAPE · ✅ ENVEL REPLOT · ✅ ENVEL SCALED · ⬜ SPIKE · ✅ ENVEL SWELL · ✅ ENVEL TIMEGRID · ⬜ TOPANTAIL2 · ⬜ TREMENV · ✅ ENVEL TREMOLO · ⬜ TREMOLO · ✅ ENVEL WARP

### ENVNU (2)

✅ EXPDECAY · ✅ PEAKCHOP

### EXTEND (31)

✅ EXTEND BAKTOBAK · ✅ BOUNCE · ⬜ CERACU · ✅ EXTEND DOUBLETS · ✅ EXTEND DRUNK · ⬜ DVDWIND · ✅ SFECHO ECHO · ⬜ ENVSPEAK · ✅ EXTEND FREEZE · ✅ HOVER · ✅ HOVER2 · ✅ EXTEND ITERATE · ✅ ITERLINE · ✅ ITERLINEF · ✅ EXTEND LOOP · ⬜ MADRID · ⬜ MOTOR · ⬜ PULSER · ⬜ PULSER MULTI · ⬜ REPEATER · ✅ EXTEND REPETITIONS · ⬜ ROTOR · ✅ EXTEND SCRAMBLE · ✅ EXTEND SEQUENCE · ⬜ EXTEND SEQUENCE2 · ⬜ SHIFTER · ⬜ SHRINK · ⬜ SORTER · ⬜ STUTTER · ⬜ TESSELATE · ✅ EXTEND ZIGZAG

### FILTER (13)

✅ FILTER BANK · ✅ FILTER BANKFRQS · ⬜ FILTRAGE · ✅ FILTER FIXED · ✅ FILTER ITERATED · ✅ FILTER LOHI · ✅ FILTER PHASING · ✅ PHASOR · ✅ FILTER SWEEPING · ✅ FILTER USERBANK · ✅ FILTER VARIABLE · ✅ FILTER VARIBANK/2 · ✅ FILTER VFILTERS

### FOCUS (9)

✅ FOCUS ACCU · ✅ FOCUS EXAG · ✅ FOCUS FOCUS · ✅ FOCUS FOLD · ✅ FOCUS FREEZE · ✅ FOCUS HOLD · ⬜ SPECFOLD · ✅ FOCUS STEP · ⬜ SUPERACCU

### FORMANTS (8)

⬜ FORMANTS GET · ⬜ FORMANTS GETSEE · ⬜ FORMANTS PUT · ⬜ FORMANTS SEE · ⬜ SPECENV · ⬜ FORMANTS VOCODE · ⬜ ONEFORM · ⬜ SPECFNU

### GRAIN (18)

✅ GRAIN ALIGN · ✅ GRAIN ASSESS · ✅ GRAIN COUNT · ✅ GRAIN DUPLICATE · ⬜ GRAINEX · ✅ GRAIN GREV · ⬜ NEWTEX · ✅ GRAIN NOISE_EXTEND · ✅ GRAIN FIND · ✅ GRAIN OMIT · ✅ GRAIN REMOTIF · ✅ GRAIN REORDER · ✅ GRAIN REPITCH · ✅ GRAIN REPOSITION · ✅ GRAIN RERHYTHM · ✅ GRAIN REVERSE · ✅ GRAIN R_EXTEND · ✅ GRAIN TIMEWARP

### HILITE (9)

⬜ HILITE ARPEG · ⬜ HILITE BAND · ⬜ HILITE BLTR · ⬜ HILITE FILTER · ⬜ GLISTEN · ⬜ HILITE GREQ · ⬜ HILITE PLUCK · ⬜ HILITE TRACE · ⬜ HILITE VOWELS

### HOUSE (18)

⬜ HOUSEKEEP BAKUP · ⬜ HOUSEKEEP BATCHEXPAND · ⬜ HOUSEKEEP BUNDLE · ⬜ CHANPHASE · ⬜ HOUSEKEEP CHANS · ⬜ HOUSEKEEP COPY · ⬜ HOUSEKEEP DEGLITCH · ⬜ HOUSEKEEP DISK · ⬜ HOUSEKEEP ENDCLICKS · ⬜ HOUSEKEEP EXTRACT · ⬜ HOUSEKEEP GATE · ⬜ GATE · ⬜ PAIREX · ⬜ HOUSEKEEP REMOVE · ⬜ REPAIR · ⬜ HOUSEKEEP RESPEC · ⬜ HOUSEKEEP SORT · ⬜ TOSTEREO

### MODIFY (17)

✅ MODIFY BRASSAGE · ✅ MODIFY CONVOLVE · ⬜ DSHIFT · ✅ MODIFY FINDPAN · ✅ MODIFY LOUDNESS · ⬜ NEWDELAY · ⬜ PHASE · ✅ MODIFY RADICAL · ✅ MODIFY REVECHO · ✅ MODIFY SAUSAGE · ✅ MODIFY SCALEDPAN · ✅ MODIFY SHUtdER · ✅ MODIFY SPACE · ✅ MODIFY SPACEFORM · ✅ MODIFY SPEED · ✅ MODIFY STACK · ⬜ VERGES

### MORPH (4)

⬜ MORPH BRIDGE · ⬜ MORPH GLIDE · ⬜ MORPH MORPH · ⬜ NEWMORPH

### MULTICHAN (29)

⬜ BROWNIAN · ⬜ CASCADE · ⬜ CRUMBLE · ⬜ CRYSTAL · ⬜ FLUTTER · ⬜ FRACTURE · ⬜ FRAME SHIFT · ⬜ MCHANPAN · ⬜ MCHANREV · ⬜ MCHITER · ⬜ MCHSHRED · ⬜ MCHSTEREO · ⬜ MCHZIG · ⬜ MTON · ⬜ MULTIMIX CREATE · ⬜ NEWMIX · ⬜ PANORAMA · ⬜ SPIN STEREO · ⬜ SPIN QUAD · ⬜ STRANS MULTI · ⬜ TEXMCHAN · ⬜ TRANSIT SIMPLE · ⬜ TRANSIT FILTERED · ⬜ TRANSIT DOPPLER · ⬜ TRANSIT DOPLFILT · ⬜ TRANSIT SEQUENCE · ⬜ TRANSIT LIST · ⬜ NEWTEX · ⬜ WRAPPAGE

### MCTOOLKIT (13)

⬜ ABFPAN · ⬜ ABFPAN2 · ⬜ CHANNELX · ⬜ CHORDER · ⬜ CHXFORMAT · ⬜ COPYSFX · ⬜ FMDCODE · ⬜ INTERLX · ⬜ NJOIN · ⬜ NMIX · ⬜ PAPLAY · ⬜ RMSINFO · ⬜ SFPROPS

### ONEFORM (3)

⬜ ONEFORM GET · ⬜ ONEFORM PUT · ⬜ ONEFORM COMBINE

### PITCH (9)

⬜ PITCH ALTHARMS · ⬜ PITCH CHORD · ⬜ PITCH CHORDF · ⬜ PITCH OCTMOVE · ⬜ PITCH PICK · ⬜ SPECTUNE · ⬜ PITCH TRANSP · ⬜ PITCH TUNE · ⬜ TUNEVARY

### PITCHINFO (5)

⬜ PITCHINFO CONVERT · ⬜ PITCHINFO HEAR · ⬜ PITCHINFO INFO · ⬜ PITCHINFO SEE · ⬜ PITCHINFO ZEROS

### PSOW (23)

⬜ PSOW CHOP · ⬜ PSOW CUTATGRAIN · ⬜ PSOW DELETE · ⬜ PSOW DUPL · ⬜ PSOW FEATURES · ⬜ FOFEX EXTRACT · ⬜ FOFEX CONSTRUCT · ⬜ PSOW GRAB · ⬜ PSOW IMPOSE · ⬜ PSOW INTERLEAVE · ⬜ PSOW INTERP · ⬜ PSOW LOCATE · ✅ PTOBRK · ⬜ PSOW REINFORCE · ⬜ PSOW REPLACE · ⬜ PSOW SPACE · ⬜ PSOW SPLIT · ⬜ PSOW STRETCH · ⬜ PSOW STRTRANS · ⬜ PSOW SUSTAIN · ⬜ PSOW SUSTAIN2 · ⬜ PSOW SYNTH · ⬜ TWEET

### PVOC (8)

⬜ ANA2PVX · ✅ PVOC ANAL · ⬜ PVOC EXTRACT · ⬜ FTURANAL ANAL · ⬜ FTURANAL SYNTH · ✅ PVOC SYNTH · ⬜ PVOCEX2 · ⬜ PVPLAY

### REPITCH (29)

✅ REPITCH ANALENV · ✅ REPITCH APPROX · ✅ BRKTOPI · ✅ REPITCH COMBINE · ✅ REPITCH COMBINEB · ✅ REPITCH CUT · ✅ REPITCH EXAG · ✅ REPITCH FIX · ✅ REPITCH GENERATE · ✅ REPITCH GETPITCH · ✅ REPITCH INSERTSIL · ✅ REPITCH INSERTZEROS · ✅ REPITCH INTERP · ✅ REPITCH INVERT · ✅ REPITCH NOISETOSIL · ✅ REPITCH PCHSHIFT · ✅ REPITCH PCHTOTEXT · ✅ REPITCH PITCHTOSIL · ✅ REPITCH QUANTISE · ✅ REPITCH RANDOMISE · ✅ REPITCH SMOOTH · ✅ REPITCH SYNTH · ✅ REPITCH TRANSPOSE · ✅ REPITCH TRANSPOSEF · ✅ REPITCH VIBRATO · ✅ REPITCH VOWELS · ✅ COMBINE MAKE · ✅ COMBINE MAKE2 · ✅ PTOBRK

### RETIME (14)

⬜ 1: PULSED PEAKS · ⬜ 2: SYNCHRONISE PEAKS · ⬜ 3: SHORTEN EVENTS · ⬜ 4: PULSED · ⬜ 5: SPEED · ⬜ 6: REPOSITION AT BEATS · ⬜ 7: REPOSITION AT TIMES · ⬜ 8: REPEAT EVENT(S) · ⬜ 9: MASK EVENTS · ⬜ 10: ACCENTS · ⬜ 11: FIND DURATIONS · ⬜ 12: FIND START · ⬜ 13: MOVE FOUND PEAK · ⬜ 14: MOVE SPECIFIED PEAK

### REVERB (5)

⬜ FASTCONV · ✅ REVERB · ⬜ ROOMRESP · ⬜ ROOMVERB · ⬜ TAPDELAY

### EDIT (34)

⬜ CANTOR · ⬜ CONSTRICT · ✅ SFEDIT CUT · ✅ SFEDIT CUTEND · ✅ SFEDIT CUTMANY · ⬜ ENVCUT · ✅ SFEDIT EXCISE · ✅ SFEDIT EXCISES · ✅ SFEDIT INSERT · ✅ SFEDIT INSIL · ✅ ISOLATE · ✅ SFEDIT JOIN · ✅ SFEDIT JOINDYN · ✅ SFEDIT JOINSEQ · ⬜ MANYSIL · ✅ SFEDIT MASKS · ✅ SFEDIT NOISECUT · ⬜ PACKET · ⬜ PARTITION · ⬜ PREFIX SILENCE · ✅ SFEDIT RANDCHUNKS · ⬜ SFEDIT RANDCUTS · ✅ SFEDIT REPLACE · ⬜ REJOIN · ⬜ RETIME · ⬜ SILEND · ✅ SFEDIT SPHINX · ⬜ SFEDIT SUBTRACT · ✅ SFEDIT SYLLABLES · ✅ SFEDIT TWIXT · ⬜ WAVEFORM · ✅ SFEDIT ZCUT · ✅ SFEDIT ZCUTS · ⬜ DISTCUT

### INFO (22)

✅ SNDINFO CHANDIFF · ✅ SNDINFO DIFF · ✅ SNDINFO FINDHOLE · ✅ SNDINFO LEN · ✅ SNDINFO LENS · ✅ SNDINFO LOUDCHAN · ⬜ SNDINFO MAXI · ✅ SNDINFO MAXSAMP · ✅ SNDINFO MAXSAMP2 · ⬜ ONSET · ⬜ SNDINFO PEAKFIND · ⬜ SNDINFO PRNTSND · ✅ SNDINFO PROPS · ⬜ SEARCH SIGSTART · ✅ SNDINFO SMPTIME · ✅ SNDINFO SUMLEN · ✅ SNDINFO TIMEDIFF · ✅ SNDINFO TIMESMP · ⬜ SNDINFO UNITS · ⬜ SNDINFO ZCROSS · ⬜ RETIME Mode 12 · ⬜ SFPROPS

### SPEC (8)

⬜ ANALJOIN · ⬜ SPEC BARE · ⬜ SPEC CLEAN · ⬜ SPEC CUT · ⬜ SPEC GAIN · ⬜ SPEC GATE · ⬜ SPEC GRAB · ⬜ SPEC MAGNIFY

### SPECFNU (23)

⬜ 1 NARROW FORMANTS · ⬜ 2 SQUEEZE SPECTRUM · ⬜ 3 INVERT FORMANTS · ⬜ 4 ROTATE FORMANTS · ⬜ 5 SPECTRAL NEGATIVE · ⬜ 6 SUPPRESS FORMANTS · ⬜ 7 GENERATE FILTER(S) FROM FORMANTS · ⬜ 8 MOVE FORMANTS BY · ⬜ 9 MOVE FORMANTS TO · ⬜ 10 ARPEGGIATE · ⬜ 11 OCTAVE-SHIFT · ⬜ 12 TRANSPOSE · ⬜ 13 FREQ-SHIFT · ⬜ 14 RESPACE PARTIALS · ⬜ 15 PITCH-INVERT · ⬜ 16 PITCH-EXAGG/SMOOTH · ⬜ 17 PITCH-QUANTISE · ⬜ 18 PITCH-RANDOMISE · ⬜ 19 RANDOMISE PARTIALS · ⬜ 20 SEE SPEC ENVELOPES · ⬜ 21 SEE SPEC PEAKS/TROUGHS · ⬜ 22 GET LOUDNESS TROUGHS · ⬜ 23 SINE SPEECH

### SPECINFO (10)

⬜ SPECINFO CHANNEL · ⬜ SPECINFO FREQUENCY · ⬜ GET_PARTIALS HARMONIC · ⬜ SPECINFO LEVEL · ⬜ SPECINFO OCTVU · ⬜ SPECINFO PEAK · ⬜ PEAK EXTRACT · ⬜ SPECINFO PRINT · ⬜ SPECINFO REPORT · ⬜ SPECINFO WINDOWCNT

### SPECNU (10)

⬜ FRACTAL SPECTRUM · ⬜ SPECNU CLEAN · ⬜ MATRIX · ⬜ SPECNU RAND · ⬜ SPECNU REMOVE · ⬜ SPECNU SLICE · ⬜ SPECGRIDS · ⬜ SPECULATE · ⬜ SPECNU SQUEEZE · ⬜ SPECNU SUBTRACT

### STRANGE (4)

⬜ STRANGE GLIS · ⬜ STRANGE INVERT · ⬜ STRANGE SHIFT · ⬜ STRANGE WAVER

### STRETCH (4)

✅ SPECTSTR · ✅ STRETCH SPECTRUM · ✅ STRETCH TIME · ⬜ STRETCHA

### SUBMIX (24)

✅ SUBMIX ADDTOMIX · ✅ SUBMIX ATSTEP · ✅ SUBMIX ATTENUATE · ✅ SUBMIX BALANCE · ✅ SUBMIX CROSSFADE · ✅ SUBMIX DUMMY · ✅ SUBMIX FADERS · ⬜ SUBMIX FILEFORMAT · ✅ SUBMIX GETLEVEL · ✅ SUBMIX INBETWEEN · ✅ SUBMIX INBETWEEN2 · ✅ SUBMIX INTERLEAVE · ✅ SUBMIX MERGE · ✅ SUBMIX MERGEMANY · ✅ SUBMIX MIX · ✅ SUBMIX MODEL · ✅ SUBMIX ONGRID · ✅ SUBMIX PAN · ✅ SUBMIX SHUFFLE · ✅ SUBMIX SPACEWARP · ✅ SUBMIX SYNC · ✅ SUBMIX SYNCATTACK · ✅ SUBMIX TEST · ✅ SUBMIX TIMEWARP

### SYNTH (18)

⬜ SYNTH CHORD · ⬜ CLICKNEW · ⬜ SYNTH CLICKS · ⬜ IMPULSE · ⬜ MULTIOSC · ⬜ MULTISYNTH · ⬜ NEWSYNTH · ⬜ NEWSCALES · ⬜ SYNTH NOISE · ⬜ PULSER SYNTH · ⬜ SYNTH SILENCE · ⬜ SYNTH SPECTRA · ⬜ SYNFILT · ⬜ SYNSPLINE · ⬜ TS OSCIL · ⬜ TS TRACE · ⬜ TSCONVERT · ⬜ SYNTH WAVE

### UTILS (12)

⬜ ASCIIGET · ⬜ CDPCONV · ⬜ COPYSFX · ⬜ DIRSF · ⬜ GETCOL · ⬜ LISTAUDEVS · ⬜ PAPLAY · ⬜ PUTCOL · ⬜ PVPLAY · ⬜ RECSF · ⬜ VECTORS · ⬜ TSCONVERT

### TEXTURE (14)

⬜ TEXTURE SIMPLE · ⬜ TEXTURE GROUPED · ⬜ TEXTURE DECORATED · ⬜ TEXTURE MOTIFS · ⬜ TEXTURE MOTIFSIN · ⬜ TEXTURE ORNATE · ⬜ TEXTURE POSTDECOR · ⬜ TEXTURE POSTORNATE · ⬜ TEXTURE PREDECOR · ⬜ TEXTURE PREORNATE · ⬜ TEXTURE TIMED · ⬜ TEXTURE TGROUPED · ⬜ TEXTURE TMOTIFS · ⬜ TEXTURE TMOTIFSIN

## Maintenance notes

Re-run this audit whenever files are added to or removed from `src/processes/definitions`, when `src/processes/catalog.ts` changes, or when the bundled CDP distribution is upgraded. A process should only move to ✅ after its manifest is registered; executable presence alone is insufficient because the GUI also needs inputs, parameters, modes, constraints, output behavior, documentation provenance, and safe command compilation.
