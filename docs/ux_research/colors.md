# Color for Reading UIs — Research & Palette Rules

Research for the personal news portal (Axum + HTMX, single reader, PT-BR content, long-session news reading). Scope: contrast standards (WCAG 2.2 + APCA/WCAG 3 draft), dark-mode reading science, platform dark-theme guidance (Material, Apple HIG), categorical color limits, and concrete rules for **our** palette (dark ink `#0b0f14`, teal `#58e7d6`, amber `#e8a33d`, per-section accents, full light theme). Fonts are covered by a separate research doc and are out of scope here.

Date: 2026-08-04. All contrast numbers below were computed by us from the published formulas: WCAG 2.x relative-luminance ratio ([W3C, Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html)) and the APCA SAPC-4G algorithm ([Myndex/SAPC-APCA](https://github.com/Myndex/apca-introduction)). APCA Lc values are polarity-signed (negative = light text on dark background).

---

## 1. WCAG 2.2 contrast requirements (the compliance floor)

- **1.4.3 Contrast (Minimum), Level AA**: text ≥ **4.5:1**; "large text" (≥24px regular or ≥18.66px bold) ≥ **3:1**. ([W3C Understanding 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html), [WebAIM](https://webaim.org/articles/contrast/))
- **1.4.6 Contrast (Enhanced), Level AAA**: text ≥ **7:1**; large text ≥ **4.5:1**. ([W3C Understanding 1.4.6](https://www.w3.org/WAI/WCAG22/Understanding/contrast-enhanced.html), [Make Things Accessible](https://www.makethingsaccessible.com/guides/contrast-requirements-for-wcag-2-2-level-aa/))
- **1.4.11 Non-text Contrast, Level AA**: UI component boundaries/states and meaningful graphics ≥ **3:1** against adjacent colors — this covers our section accent bars, focus rings, icons, and card borders, not just text. ([W3C Understanding 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html), [Deque](https://dequeuniversity.com/resources/wcag2.1/1.4.11-non-text-contrast))
- The ratios did not change between WCAG 2.1 and 2.2. ([Make Things Accessible](https://www.makethingsaccessible.com/guides/contrast-requirements-for-wcag-2-2-level-aa/))

**Implication for a one-reader app:** WCAG AA is a legal/compat floor, not a comfort target. For a portal read daily for long sessions, aim at AAA-ish (7:1) for body text and use APCA (below) as the perceptual sanity check — especially in the dark theme, where WCAG ratios are least trustworthy.

## 2. Why 4.5:1 can mislead on dark backgrounds — APCA / WCAG 3 draft

- The WCAG 2.x ratio math "far overstates contrast for dark colors"; a pair that passes 4.5:1 "can be functionally unreadable when a color is near black," and WCAG 2 contrast "cannot provide useful guidance when designing 'dark mode.'" ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html), [APCA in a Nutshell](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html))
- APCA (Accessible/Advanced Perceptual Contrast Algorithm, by Andrew Somers/Myndex, candidate model for WCAG 3) outputs a perceptually uniform lightness-contrast value **Lc**: the same Lc means the same perceived readability regardless of how dark the pair is. WCAG 2 and APCA agree only in a narrow range near mid-gray. ([Myndex APCA](https://git.myndex.com/), [apca-introduction](https://github.com/Myndex/apca-introduction))
- **Status caveat:** APCA is a *draft* candidate for WCAG 3, developed within the W3C Silver/Visual Contrast of Text subgroup — it is not yet a normative standard, so we use WCAG 2.2 AA as the compliance floor and APCA as the design target. ([W3C Silver Visual Contrast subgroup](https://www.w3.org/WAI/GL/task-forces/silver/wiki/Visual_Contrast_of_Text_Subgroup))
- APCA recommended levels ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html)):
  - **Lc 90** — preferred for fluent/body text columns (≥14px, weight 400)
  - **Lc 75** — minimum for body text (≥18px, weight 400)
  - **Lc 60** — minimum for non-body content text (≥24px normal or ≥16px bold)
  - **Lc 45** — headlines/large text minimum (≥36px normal or ≥24px bold)
  - **Lc 30** — absolute minimum (placeholder/disabled)
  - **Lc 15** — non-text minimum
  - Dark-mode note: a stated **maximum** of about Lc −90 for large fonts — i.e. in dark mode there is such a thing as *too much* contrast (halation), which WCAG 2 has no concept of. ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html))

## 3. Dark-mode reading science

### 3.1 Positive-polarity advantage (dark-on-light reads better)

- Piepenbrock et al. (2013): dark text on light background beat light-on-dark for **both** visual acuity and proofreading, for **both** young (18–33) and older (60–85) adults; the advantage grows as font size shrinks; participants did not subjectively notice the difference. Authors recommend positive polarity for all ages. ([ResearchGate — Piepenbrock](https://www.researchgate.net/scientific-contributions/Cosima-Piepenbrock-2008470047), [NN/g summary](https://www.nngroup.com/articles/dark-mode/))
- Buchner & Baumgartner: polarity effect holds "irrespective of ambient illumination and colour contrast." ([ResearchGate](https://www.researchgate.net/publication/6321309_Text_-_Background_polarity_affects_performance_irrespective_of_ambient_illumination_and_colour_contrast))
- Mechanism: light backgrounds constrict the pupil; a smaller pupil reduces spherical aberration and increases depth of field → sharper retinal image. Dark backgrounds dilate the pupil → more aberration, fuzzier edges. ([NN/g](https://www.nngroup.com/articles/dark-mode/))
- Nuance, not a slam-dunk: Dobres et al. 2017 found no polarity effect for glanceable reading in daytime; at night light mode still won, especially at small sizes. A 2018 study associated sustained light-mode reading with choroid thinning (a myopia correlate), so light mode is not cost-free either. NN/g's conclusion: default to light for long reading, offer both, respect the OS setting. ([NN/g](https://www.nngroup.com/articles/dark-mode/))
- Newer work is more mixed for non-text tasks: a 2024 visualization study across age groups found performance differences by contrast polarity interact with age and task — evidence for *charts* is weaker than for *text*. ([arXiv 2409.10841](https://arxiv.org/html/2409.10841v1))

**Implication:** our default-dark portal is the scientifically *harder* reading mode. That doesn't mean abandon it (single user, personal preference, likely low-light use), but it means dark-theme tuning has to be more careful than light-theme tuning, and the light theme deserves first-class treatment for daytime reading.

### 3.2 The "~50% astigmatism" halation claim — verified: **prevalence roughly true, the specific reading claim is weakly sourced**

- The claim usually cited ("people with astigmatism, ~50% of the population, find white-on-black harder to read") traces to Jason Harrison (UBC, Sensory Perception and Interaction Research Group). Multiple attempts to find the underlying paper fail: it appears to be informal post-doc-era statements, **not a formal publication** — the citation trail is circular blog-to-blog. Treat the specific "50% + astigmatism ⇒ worse dark-mode reading" causal claim as plausible but unverified. ([H. Locke, Medium](https://medium.com/@h_locke/why-dark-mode-causes-more-accessibility-issues-than-it-solves-54cddf6466f5), [Jessica Otis](https://jessicaotis.com/academia/never-use-white-text-on-a-black-background-astygmatism-and-conference-slides/), [Tatham Oddie](https://tatham.blog/2008/10/13/why-light-text-on-dark-background-is-a-bad-idea/))
- The prevalence part is independently supported: a systematic review/meta-analysis (Hashemi et al., 163 studies) estimates adult astigmatism prevalence at **40.4%** (95% CI 34.3–46.6); a 2023 systematic review reports 30–50%+ depending on population and threshold. So "roughly 40–50% of adults" is fair; "50%" is the optimistic end. ([Hashemi et al., J Curr Ophthalmol](https://www.sciencedirect.com/science/article/pii/S2452232517300227), [PubMed](https://pubmed.ncbi.nlm.nih.gov/29564404/), [Zhang 2023, Optom Vis Sci](https://onlinelibrary.wiley.com/doi/10.1097/OPX.0000000000001998))
- Halation itself (bright text appearing to glow/bleed on dark ground, worse with uncorrected optics and dilated pupils) is a well-described phenomenon in accessibility practice even if the "50%" pairing is folklore. ([Level Access](https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/), [UX Movement](https://uxmovement.com/content/why-you-should-never-use-pure-black-for-text-or-backgrounds/), [BOIA](https://www.boia.org/blog/dark-mode-can-improve-text-readability-but-not-for-everyone))

### 3.3 Pure white on dark = halation → use off-white (recommendation is consistent, evidence is practice-based)

- Consistent practitioner guidance: avoid `#FFFFFF` text on dark; use off-whites around `#E8E8E8`–`#F0F0F0` or rgb(220,220,220), and dark grays rgb(30–40) rather than pure black, to cut the glow without losing legibility. Note honestly: this is design-practice consensus + the APCA Lc −90 ceiling, not a large peer-reviewed literature. ([bushe.co](https://bushe.co/blog/dark-mode-hurts-eyes-how-to-fix/), [Level Access](https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/), [UX Movement](https://uxmovement.com/content/why-you-should-never-use-pure-black-for-text-or-backgrounds/), [APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html))
- Material's dark-theme material states pure white body text on dark surfaces "appears to bleed or blur against the dark background," which is why it specifies white **at 87% opacity** for high-emphasis text instead. ([Google Material dark-theme codelab](https://codelabs.developers.google.com/codelabs/design-material-darktheme))

## 4. Platform dark-theme guidance

### 4.1 Material Design

([Material dark theme](https://m2.material.io/design/color/dark-theme.html), [dark-theme codelab](https://codelabs.developers.google.com/codelabs/design-material-darktheme), [Design for the Dark Theme — Snapp Mobile](https://medium.com/snapp-mobile/design-for-the-dark-theme-9a2185bbb1d5))

- Base surface **`#121212`** (dark gray), not pure black: pure black creates excessive contrast with bright content and leaves no room for elevation.
- **Text opacity ramp on dark**: high emphasis = white @ **87%**, medium = **60%**, disabled = **38%** — a built-in "don't use pure white" rule.
- **Desaturate accents**: saturated colors "visually 'vibrate' against darker backgrounds"; use lighter/desaturated tonal variants (≈ tone 200) in dark theme.
- **Elevation = lighter surfaces**: hierarchy on dark comes from progressively lighter overlay surfaces (e.g. `#121212` → `#1E1E1E` → `#242424`), since shadows are invisible on dark. ([Ounass/Medium](https://medium.com/ounass/dark-mode-in-digital-product-design-3deb852ae98d))
- Dark surfaces should support white text at up to ~**15.8:1** so that lower-emphasis (opacity-reduced) text still clears 4.5:1. ([fourzerothree.in](https://www.fourzerothree.in/p/scalable-accessible-dark-mode), [Ounass/Medium](https://medium.com/ounass/dark-mode-in-digital-product-design-3deb852ae98d))

### 4.2 Apple HIG

([Apple HIG — Dark Mode](https://developer.apple.com/design/human-interface-guidelines/dark-mode), [Apple HIG — Color](https://developer.apple.com/design/human-interface-guidelines/foundations/color/), [Median.co summary](https://median.co/blog/what-are-apples-human-interface-guidelines-for-dark-mode))

- If a light content background must appear in Dark Mode, "choose a slightly darker white that prevents the background from glowing against the surrounding dark content."
- Prefer semantic/adaptive colors (label, secondaryLabel, tertiary, quaternary; system backgrounds) that re-tune per appearance rather than one fixed hex reused in both themes — i.e. **each theme gets its own tuned values**, which is exactly the accent problem we have (see §6).
- Dark Mode may increase **vibrancy** to keep foreground legible on dark; test all colors in both appearances.

## 5. How many categorical colors? (our per-section accents)

- Practical cap for categorical color coding is about **six** distinguishable colors; beyond that discrimination and recall degrade. ([SciFig](https://scifig.ai/blog/color-palettes-scientific-figures), [Figviz Okabe-Ito reference](https://figviz.com/blog/okabe-ito-palette-hex-codes-full-8-color-reference-with-code-examples-2026-wlt9th1n))
- The **Okabe–Ito** 8-color Color Universal Design palette is the standard colorblind-safe categorical set (orange, sky blue, bluish green, yellow, blue, vermilion, reddish purple, black/gray); no palette extends cleanly past ~8 under color-vision-deficiency constraints. ([Okabe & Ito, jfly](https://jfly.uni-koeln.de/color/), [arXiv: Coloring in R's Blind Spot](https://arxiv.org/pdf/2303.04918))
- Okabe–Ito's core rule: **never encode by color alone** — pair color with position, icon, or label. Our section accents already co-occur with section names, which satisfies this. ([jfly](https://jfly.uni-koeln.de/color/))
- Our teal/amber primary pair sits on a blue↔orange axis, which is the axis best preserved under the common red-green deficiencies (protan/deutan) — a good foundation. ([Okabe & Ito, jfly](https://jfly.uni-koeln.de/color/), [Myndex discussion on protan contrast](https://github.com/Myndex/SAPC-APCA/discussions/18))

**Rule for us:** ≤ 6 section accent hues, spaced in hue and in *lightness*, checked against CVD simulation, always accompanied by the section label. If sections exceed 6, reuse hues across low-confusion contexts rather than inventing a 7th color.

## 6. Our palette, measured

Computed with the WCAG 2.x formula and APCA SAPC-4G (implementation per [Myndex/apca-introduction](https://github.com/Myndex/apca-introduction); WCAG formula per [W3C](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html)). Dark background = `#0b0f14`.

### 6.1 Dark theme — current colors

| Pair | WCAG | APCA Lc | Verdict |
|---|---|---|---|
| `#ffffff` on `#0b0f14` | 19.22:1 | −107.5 | **Too hot** — exceeds APCA's ~Lc 90 dark-mode ceiling; halation risk |
| `#e8e8e8` on `#0b0f14` | 15.69:1 | −92.6 | Good for headlines; slightly hot for body |
| `#dde3ea` on `#0b0f14` | 14.87:1 | −88.9 | **Ideal body** (~Lc 90 target) |
| `#c9d1d9` on `#0b0f14` | 12.45:1 | −77.7 | Good body (fluent min Lc 75) |
| `#aeb8c6` on `#0b0f14` | 9.58:1 | −63.1 | Good meta/secondary (small text needs ≥ Lc 60) |
| `#8a94a3` on `#0b0f14` | 6.26:1 | −43.8 | **Passes WCAG AA, fails APCA for small text** — classic dark-mode false pass |
| teal `#58e7d6` on `#0b0f14` | 12.64:1 | −79.0 | Fine at any size, incl. links |
| amber `#e8a33d` on `#0b0f14` | 8.91:1 | −59.7 | Fine for headlines/bold ≥16px; borderline (just under Lc 60) for small regular text — use `#f0b25a` (10.27:1, Lc −66.9) when small |
| `#0b0f14` ink on teal chip | 12.64:1 | +79.4 | Filled teal chips/buttons: use dark ink, not white |
| `#0b0f14` ink on amber chip | 8.91:1 | +61.3 | Same: dark ink on amber |
| surface `#1e1e1e` vs `#121212` | 1.12:1 | ~0 | Elevation alone is invisible as a *boundary*; interactive component edges still need a ≥3:1 cue (border/outline) per 1.4.11 ([W3C](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html)) |

### 6.2 Light theme — the real problem

| Pair | WCAG | APCA Lc | Verdict |
|---|---|---|---|
| teal `#58e7d6` on `#faf9f7` | 1.44:1 | +20.3 | **Unusable as text/link** — fails everything |
| amber `#e8a33d` on `#faf9f7` | 2.05:1 | +38.6 | **Fails for text**; icon/large-decoration only |
| dark teal `#0f766e` on white | 5.47:1 | +76.9 | Good link/accent-text replacement |
| dark amber `#92600a` on white | 5.38:1 | +76.6 | Good replacement (or `#b45309`, 5.02:1, Lc +74) |
| ink `#24292f` on `#faf9f7` | 13.93:1 | +97.9 | Ideal body (avoid pure `#000` for the mirror-image halation reason) |
| meta `#57606a` on `#faf9f7` | 6.07:1 | +78.2 | Good meta |

This is the Apple-HIG point in practice: **accent colors must be theme-specific**, not one hex shared across themes. ([Apple HIG Color](https://developer.apple.com/design/human-interface-guidelines/foundations/color/))

## 7. Rules for our app (keep / fix)

### Keep

1. **Dark ink `#0b0f14` base** — near-black-but-not-black is consistent with Material's rejection of pure `#000` (`#121212` baseline; ours is a hair darker, acceptable — but see rule 4 on elevation). ([codelab](https://codelabs.developers.google.com/codelabs/design-material-darktheme))
2. **Teal `#58e7d6` as primary accent on dark** — measured Lc −79 / 12.6:1; safe at all sizes including small link text.
3. **Teal + amber as the two-hue system** — blue/orange axis is CVD-robust. ([jfly](https://jfly.uni-koeln.de/color/))
4. **Having a full light theme** — the reading-performance evidence favors dark-on-light for long-form reading; keep it healthy and consider it (or auto-switch by time of day) for daytime reading. ([NN/g](https://www.nngroup.com/articles/dark-mode/))

### Fix

1. **Kill pure white text in dark theme.** Anything at `#fff` drops to ≤ `#e8e8e8` (headlines) / `#dde3ea` (body). Target: body Lc ≈ −85 to −90 (≈13–15:1), never past ~Lc −90. ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html), [codelab](https://codelabs.developers.google.com/codelabs/design-material-darktheme))
2. **Raise meta/caption text on dark.** If current meta is around `#8a94a3` (6.3:1 — a WCAG pass), it fails APCA for small text; lift to ≥ `#aeb8c6` (9.6:1, Lc −63) for 12–13px metadata. This is the single most likely cause of "reading comfort poor" beside fonts: the dark theme's mid-grays that *look* compliant are perceptually underpowered. ([APCA in a Nutshell](https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html))
3. **Theme-split the accents.** Light theme swaps teal → `#0f766e`, amber → `#92600a`/`#b45309` for any text-sized use (links, kickers, counts). Current accents on light paper are 1.4:1 and 2.1:1 — hard failures. ([W3C 1.4.3](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html))
4. **Small amber text on dark**: reserve `#e8a33d` for headline-scale/bold; use `#f0b25a` at small sizes (Lc −67 vs −60 borderline). Slightly desaturate accents used over large dark areas to avoid "vibration." ([codelab](https://codelabs.developers.google.com/codelabs/design-material-darktheme))
5. **Non-text 3:1 audit** (WCAG 1.4.11): section accent bars, focus rings, input borders, and card outlines on `#0b0f14` need ≥3:1 vs adjacent surface; elevation tints alone (1.1:1) don't count as component boundaries. ([W3C 1.4.11](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html))
6. **Section accents: ≤6 hues, CVD-checked, never color-only.** Derive them at two lightness levels — a bright variant for dark theme, a dark variant for light theme — the same split as teal/amber. ([jfly](https://jfly.uni-koeln.de/color/), [SciFig](https://scifig.ai/blog/color-palettes-scientific-figures))
7. **Light-theme ink:** body `#24292f`-ish on warm paper (13.9:1), not `#000` on `#fff` (pure-black-on-pure-white is the mirrored halation/glare complaint). ([UX Movement](https://uxmovement.com/content/why-you-should-never-use-pure-black-for-text-or-backgrounds/))

### Target numbers per text role

| Role | Dark theme (on `#0b0f14`) | Light theme (on paper) |
|---|---|---|
| Headlines (condensed, large/bold) | Lc ≥ 60 abs min; aim Lc 75–92 → `#e8e8e8` | ≥ 4.5:1; aim 10:1+ → `#1a1f26`-ish |
| Body / summaries (serif, 15px) | **Lc 85–90, ~13–15:1** → `#dde3ea`–`#d5dbe3`; never `#fff` | **7:1+ (AAA), Lc ≥ 90** → `#24292f` |
| Meta / timestamps / kickers (small) | **Lc ≥ 60, ≥ 9:1** → `#aeb8c6`+ | ≥ 4.5:1, aim 6:1+ → `#57606a` |
| Links / accents (text-size) | teal `#58e7d6` ok; amber → `#f0b25a` small | teal → `#0f766e`; amber → `#92600a`/`#b45309` |
| Disabled/placeholder | Lc ≥ 30 | ≥ 3:1 practical floor |
| Non-text (borders, focus, icons, accent bars) | ≥ 3:1 vs adjacent | ≥ 3:1 vs adjacent |

Evidence-strength note: WCAG numbers are normative; APCA levels are a well-documented draft model (not yet standard); the polarity/reading-speed findings are peer-reviewed; the astigmatism-50% story is part-verified (prevalence ≈40% adults is solid, the specific reading-harm citation is untraceable); off-white-over-pure-white is strong practitioner consensus with thin formal literature. Flagged inline throughout.

---

## Sources

- https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html
- https://www.w3.org/WAI/WCAG22/Understanding/contrast-enhanced.html
- https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html
- https://webaim.org/articles/contrast/
- https://dequeuniversity.com/resources/wcag2.1/1.4.11-non-text-contrast
- https://www.makethingsaccessible.com/guides/contrast-requirements-for-wcag-2-2-level-aa/
- https://git.apcacontrast.com/documentation/APCAeasyIntro.html
- https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html
- https://git.myndex.com/
- https://github.com/Myndex/apca-introduction
- https://github.com/Myndex/SAPC-APCA/discussions/18
- https://www.w3.org/WAI/GL/task-forces/silver/wiki/Visual_Contrast_of_Text_Subgroup
- https://www.nngroup.com/articles/dark-mode/
- https://www.researchgate.net/scientific-contributions/Cosima-Piepenbrock-2008470047
- https://www.researchgate.net/publication/6321309_Text_-_Background_polarity_affects_performance_irrespective_of_ambient_illumination_and_colour_contrast
- https://arxiv.org/html/2409.10841v1
- https://www.sciencedirect.com/science/article/pii/S2452232517300227
- https://pubmed.ncbi.nlm.nih.gov/29564404/
- https://onlinelibrary.wiley.com/doi/10.1097/OPX.0000000000001998
- https://medium.com/@h_locke/why-dark-mode-causes-more-accessibility-issues-than-it-solves-54cddf6466f5
- https://jessicaotis.com/academia/never-use-white-text-on-a-black-background-astygmatism-and-conference-slides/
- https://tatham.blog/2008/10/13/why-light-text-on-dark-background-is-a-bad-idea/
- https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/
- https://uxmovement.com/content/why-you-should-never-use-pure-black-for-text-or-backgrounds/
- https://www.boia.org/blog/dark-mode-can-improve-text-readability-but-not-for-everyone
- https://bushe.co/blog/dark-mode-hurts-eyes-how-to-fix/
- https://m2.material.io/design/color/dark-theme.html
- https://codelabs.developers.google.com/codelabs/design-material-darktheme
- https://medium.com/snapp-mobile/design-for-the-dark-theme-9a2185bbb1d5
- https://medium.com/ounass/dark-mode-in-digital-product-design-3deb852ae98d
- https://www.fourzerothree.in/p/scalable-accessible-dark-mode
- https://developer.apple.com/design/human-interface-guidelines/dark-mode
- https://developer.apple.com/design/human-interface-guidelines/foundations/color/
- https://median.co/blog/what-are-apples-human-interface-guidelines-for-dark-mode
- https://jfly.uni-koeln.de/color/
- https://scifig.ai/blog/color-palettes-scientific-figures
- https://figviz.com/blog/okabe-ito-palette-hex-codes-full-8-color-reference-with-code-examples-2026-wlt9th1n
- https://arxiv.org/pdf/2303.04918
