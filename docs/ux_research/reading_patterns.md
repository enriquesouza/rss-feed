# How People Read Interfaces: Eye-Tracking Evidence and Layout Rules for the News Portal

**Date:** 2026-08-04
**Scope:** Eye-tracking research on how people scan web content (Nielsen Norman Group corpus, Edmund Arnold's Gutenberg diagram, academic reading research), translated into concrete layout rules for this portal (3-column layout: nav rail 240px | center feed | context sidebar 320px, newspaper-style front page, PT-BR content, single reader).
**Out of scope:** Typeface selection — covered by the separate font research.

---

## 1. First: correcting the "we read right-to-left in diagonal" belief

The belief that readers scan "right-to-left in a diagonal" is **not supported by the evidence for Latin-script languages** (which includes Portuguese). What the research actually shows:

- **Scanning starts at the top-LEFT.** NN/g's F-pattern research (232 users, thousands of pages) shows the dominant scan is: a horizontal sweep across the *top*, a second shorter horizontal sweep lower down, then a vertical scan down the *left* edge — the shape of the letter F ([NN/g, original 2006 study](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content-discovered/)).
- **Attention leans hard left.** In NN/g's measurement of 130,000+ fixations from 120+ participants, users spent **80% of viewing time on the left half of the page and only 20% on the right half** (up from 69% in the original 2010 study) ([NN/g, Horizontal Attention Leans Left](https://www.nngroup.com/articles/horizontal-attention-leans-left/)).
- **The "diagonal" that does exist is the Gutenberg reading gravity** — a tendency, on pages with *uniform, evenly distributed* content and weak visual hierarchy, for the eye to drift from the **top-left (primary optical area)** toward the **bottom-right (terminal area)**, sweeping left-to-right along "axes of orientation" on the way down. This is Edmund Arnold's Gutenberg diagram, from newspaper design ([Vanseo Design, "3 Design Layouts: Gutenberg Diagram, Z-Pattern, and F-Pattern"](https://vanseodesign.com/web-design/3-design-layouts/)). So the diagonal runs **top-left → bottom-right**, not right-to-left — and it only dominates when the design gives the eye nothing stronger to grab.
- **Right-to-left scanning is real — but only for right-to-left scripts.** NN/g's 2017 revisit confirmed with eye-tracking data that Arabic (and similar RTL languages) produce a **mirrored F-pattern**, starting top-right ([NN/g, F-Shaped Pattern, 2017 revisit](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content/)). The mechanism even shows up at the perceptual level: the reading "perceptual span" extends ~14 characters *ahead* of fixation in the reading direction and only ~4 behind — and this asymmetry **reverses for Hebrew readers**, proving it follows script direction rather than being hardwired ([McConkie & Rayner 1976; Pollatsek et al. 1981, via "Eye Movements During Reading", Univ. of Pittsburgh](https://sites.pitt.edu/~perfetti/Eye%20Movements%20During%20Reading.htm)).

**Bottom line for a PT-BR portal:** design for top-left entry, left-edge vertical scanning, and a *top-left → bottom-right* gravity. Nothing critical should live only on the right.

---

## 2. The physiological basics: what a "scan" is made of

From decades of reading research (Rayner and colleagues, summarized at [Univ. of Pittsburgh](https://sites.pitt.edu/~perfetti/Eye%20Movements%20During%20Reading.htm)):

- Eyes move in **saccades** (jumps of ~7–9 characters) separated by **fixations** of **200–300 ms** (range 50–500 ms). Reading is not a smooth sweep; comprehension happens only during fixations.
- Roughly **80% of words are fixated, ~20% skipped** — short function words ("de", "que", "the") are skipped most.
- **10–15% of saccades are regressions** (jumps backward), typically signaling processing difficulty. A layout or text that forces regressions is measurably harder to read.
- Fixations are longer on unfamiliar/complex words — cognitive load is visible in the eye record ([ScienceDirect, Fixation Duration overview](https://www.sciencedirect.com/topics/computer-science/fixation-duration); [Neurons Inc glossary](https://www.neuronsinc.com/glossary/fixation-duration)).

Implication: every fixation is a budget item. Scanning patterns below are strategies users adopt to spend as few fixations as possible.

---

## 3. The scanning patterns, from worst to best

NN/g's eye-tracking corpus (500+ participants, 750+ hours across three large studies over 13 years — [How People Read Online report, 2nd ed.](https://www.nngroup.com/reports/how-people-read-web-eyetracking-evidence/)) identifies four text-scanning patterns, ranked by effectiveness ([NN/g, Text Scanning Patterns: Eyetracking Evidence](https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/)):

### 3.1 F-pattern — the *failure mode*, not a design target
Occurs when text is a **wall with no formatting cues** and the user wants efficiency. Users read the first lines fully, then read less and less of each line, ending up fixating only the first words down the left edge. Consequence: **content on the right side and lower down is simply missed** ([NN/g 2017](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content/)). NN/g is explicit: "The F-pattern is the default pattern when there are no strong cues to attract the eyes towards meaningful information." It's what happens when design *fails* — and the ranking places it as the **least effective** pattern ([NN/g, Text Scanning Patterns](https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/)).

### 3.2 Spotted pattern
Fixations jump to words that **visually stand out** — links, bold, color, bullets — or match the user's task keywords. Slightly better than F, and directly steerable by the designer via keyword styling ([NN/g, Text Scanning Patterns](https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/)).

### 3.3 Layer-cake pattern — the one to engineer for
Fixations concentrate on **headings and subheadings**, forming horizontal stripes, with occasional dips into body text. It is triggered by clear, visually distinct, *descriptive* headings, and NN/g calls it "by far the most effective way in which users can scan pages" short of reading everything — users find relevant sections and confidently skip the rest, instead of missing things by accident as in the F-pattern ([NN/g, Layer-Cake Pattern of Scanning](https://www.nngroup.com/articles/layer-cake-pattern-scanning/)). Design requirements: subheadings distinct in size/weight/color (but not ad-like), front-loaded with important words, accurately describing their sections.

### 3.4 Commitment pattern
Near word-by-word reading. Only happens when the user is **highly motivated** and trusts they've found the right content ([NN/g, Text Scanning Patterns](https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/)). For a personal portal with one loyal reader, this is achievable *inside an opened article* — but never on the front page.

### Related: pinball pattern (for visually rich, card-based pages)
On layouts with mixed visual elements (images, cards, panels — e.g., modern SERPs), scanning becomes **nonlinear**, bouncing between visually weighty elements: "The visual weight of elements on the page drives people's scanning patterns" ([NN/g, Pinball Pattern](https://www.nngroup.com/articles/pinball-pattern-search-behavior/)). Two lessons: (a) visual weight is a steering wheel — big lead card gets looked at because it's big; (b) **inconsistent layouts increase cognitive load** — for a repeat-visit personal portal, keeping section layouts consistent beats novelty.

### Related: zigzag image/text layouts
Alternating image-left/image-right rows makes **decorative images harder to ignore** — users stumble on them while scanning; aligned layouts let users bypass decoration efficiently. Informational images work in either layout ([NN/g, Zigzag Image–Text Layouts, ~30–35 participants per variation](https://www.nngroup.com/articles/zigzag-page-layout/)). For the portal: keep thumbnails on a consistent side within each section's dense rows.

### How much scanning vs. reading, overall
The foundational stat: **79% of test users scanned; only 16% read word-by-word**, and rewriting a site to be concise + scannable + objective improved measured usability by **124%** ([Nielsen, How Users Read on the Web, 1997](https://www.nngroup.com/articles/how-users-read-on-the-web/)). This is the oldest finding in the corpus but has been reconfirmed across three decades of studies ([report page](https://www.nngroup.com/reports/how-people-read-web-eyetracking-evidence/)).

---

## 4. The Gutenberg diagram and what earns the terminal area

Arnold's Gutenberg diagram divides a page of **homogeneous, evenly distributed content** into four quadrants ([Vanseo Design](https://vanseodesign.com/web-design/3-design-layouts/)):

| | Left | Right |
|---|---|---|
| **Top** | Primary optical area (entry point) | Strong fallow area (seen, weaker) |
| **Bottom** | Weak fallow area (least attention) | Terminal area (exit point) |

"Reading gravity" pulls the eye from primary optical area to terminal area. Two design consequences:

1. **The terminal area (bottom-right of a unit) is the natural resting point at the end of a scan** — in print design it's where the call-to-action or "continue" element goes, because the eye lands there *after* consuming the unit. For the portal: end-of-section "ver mais →" links, next-section jumps, or "load more" belong bottom-right of each section block.
2. **The caveat is crucial**: the Gutenberg flow "only applies when there's minimal visual hierarchy… once designers apply visual hierarchy and focal points, all three patterns become less relevant" ([Vanseo Design](https://vanseodesign.com/web-design/3-design-layouts/)). Strong hierarchy (headlines, section headers, a dominant lead) **overrides** reading gravity — which is exactly the tool a newspaper front page uses. Evidence status note: the Gutenberg diagram is a print-era design heuristic with weaker direct eye-tracking validation than the NN/g patterns; treat it as secondary to the measured left-lean and F/layer-cake evidence.

---

## 5. Sidebar blindness: the real risk in a 3-column layout

- **Right-rail blindness is severe.** Users have learned that top and right-rail slots are where ads live, and they suppress attention to those regions — even for legitimate content. In one NN/g example, a right rail occupying **25% of the content area received a single fixation — 0.8% of attention, 33× less than its share of space** ([NN/g, Banner Blindness: Old and New Findings, 2018 — three decades of studies: 1997, 2007, 2018](https://www.nngroup.com/articles/banner-blindness-old-and-new-findings/)).
- Users ignore anything that *looks* like an ad: ad-typical placement (top banner, right rail), visual flashiness (animation, colored background boxes), or proximity to real ads. There's even a "hot-potato" effect: after finding ads in a region once, users avoid that region on other pages and sites ([same source](https://www.nngroup.com/articles/banner-blindness-old-and-new-findings/)).
- This compounds with the 80/20 left/right attention split ([NN/g, Horizontal Attention Leans Left](https://www.nngroup.com/articles/horizontal-attention-leans-left/)).

**Mixed-evidence caveat for this portal:** banner blindness is calibrated on ad-saturated commercial webs; a single-user personal portal with *no ads ever* may partially retrain its one reader. But the 80/20 left-lean is not ad-driven — it holds regardless. So the 320px right sidebar should be treated as **low-attention real estate by default**: fine for ambient/optional context (weather, markets ticker, "related"), wrong for anything the reader must not miss. And avoid styling sidebar modules with colored background boxes — that is precisely the ad-lookalike treatment users skip.

The **left nav rail is safe**: NN/g explicitly recommends "conventional top or left navigation layouts" and placing priority content "centrally and in left-aligned areas" ([Horizontal Attention Leans Left](https://www.nngroup.com/articles/horizontal-attention-leans-left/)).

---

## 6. Headlines: the first 2 words / 11 characters rule

When scanning a vertical list of links or headlines (exactly the portal's "destaques" list and dense headline rows), users often read only **the first ~2 words / ~11 characters** of each item before the eye drops to the next line — Nielsen calls this "nano-content." In testing (80 participants, 20 real-site links), links starting with specific, information-carrying words were understood far better; links starting with generic openers failed badly — e.g., Chase's "Introducing…" link scored **15% comprehension** ([Nielsen, First 2 Words: A Signal for the Scanning Eye, 2009](https://www.nngroup.com/articles/first-2-words-a-signal-for-scanning/)).

Combined with the F-pattern stem (vertical scan down the left edge fixating only leading words — [NN/g 2006](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content-discovered/)), this yields the strongest single writing rule for a news UI: **front-load every headline, subheading, and list item with the information-carrying words**. "Clever" or suspenseful headlines that reveal the topic at the end are effectively invisible to a scanner. NN/g's writing guidance since 1997 says the same: meaningful (not clever) subheadings, inverted pyramid, one idea per paragraph, highlighted keywords, half the word count ([How Users Read on the Web](https://www.nngroup.com/articles/how-users-read-on-the-web/)).

Note: 11 characters was measured in English; Portuguese words run longer, so the practical budget may be closer to the first 1–2 words. The principle (front-load) transfers; the exact character count is language-dependent — treat the number as indicative, not exact.

---

## 7. Translation: layout rules for this portal

Each rule below is derived from the cited evidence above.

1. **Lead story goes top-left of the center column** — the primary optical area and the F-pattern entry point. The current "big lead + destaques side list" is correct *if* the lead is left and the destaques list is to its right, not vice versa ([Gutenberg](https://vanseodesign.com/web-design/3-design-layouts/); [80/20 left-lean](https://www.nngroup.com/articles/horizontal-attention-leans-left/)).
2. **Engineer the layer-cake, defeat the F.** The per-subject section structure (section header → lead card → dense rows) is the right skeleton. Make section headers unmistakably distinct (size/weight + the existing per-section accent color) and *descriptive* — they are the stripes the eye rides ([Layer-Cake](https://www.nngroup.com/articles/layer-cake-pattern-scanning/)). An F-pattern on this page means the design failed, not that users "naturally read in an F" ([2017 revisit](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content/)).
3. **Front-load every headline: first 1–2 words carry the story.** Entity/topic first ("Petrobras corta…", "Fed sinaliza…"), never "Entenda por que…", "Saiba como…", "Introducing…"-style openers ([First 2 Words](https://www.nngroup.com/articles/first-2-words-a-signal-for-scanning/)).
4. **In dense headline rows, the left edge is sacred.** The F-stem fixates leading words only; keep headline text flush to a common left axis, put metadata (source, time) *after* or right of the headline, never before it — a timestamp prefix would eat the entire 11-character budget ([F-pattern](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content-discovered/); [First 2 Words](https://www.nngroup.com/articles/first-2-words-a-signal-for-scanning/)).
5. **Right sidebar (320px): ambient-only.** Weather, tickers, "related", stats — nothing that must be seen. Avoid boxed/colored-background modules there (ad-lookalike = skipped). If something in the sidebar is genuinely important, move it into the center column ([Banner Blindness](https://www.nngroup.com/articles/banner-blindness-old-and-new-findings/); [80/20](https://www.nngroup.com/articles/horizontal-attention-leans-left/)).
6. **Terminal area = navigation payoff.** Bottom-right of each section block is where the scanning eye exits: put "ver mais da seção →" there. Bottom-right of the page: next-page / archive link ([Gutenberg terminal area](https://vanseodesign.com/web-design/3-design-layouts/)).
7. **Consistent visual weight per element type; consistent section anatomy.** Varying card sizes/layouts per section forces pinball re-assessment on every visit and raises cognitive load; for a daily-repeat single reader, ruthless consistency is a feature ([Pinball Pattern](https://www.nngroup.com/articles/pinball-pattern-search-behavior/)).
8. **Thumbnails on one consistent side** within dense rows; don't alternate. Only use images that inform; decorative images in alternating positions trap fixations ([Zigzag study](https://www.nngroup.com/articles/zigzag-page-layout/)).
9. **Summaries: scannable, inverted pyramid, half the words.** The serif summary text should open with the conclusion; one idea per paragraph; bold the 1–2 key entities to enable the spotted pattern ([How Users Read on the Web](https://www.nngroup.com/articles/how-users-read-on-the-web/); [Text Scanning Patterns](https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/)).
10. **Article view can court the commitment pattern**: once the reader clicks through, motivation is proven — there, longer line-lengths of continuous prose are fine, and formatting can relax toward book-like reading ([Text Scanning Patterns](https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/)).
11. **Reading comfort ties to fixation economics**: layouts that make the eye regress (ragged left edges, interrupted columns, centered headline blocks) add 10–15%-class regression overhead; keep left alignment and unbroken vertical rhythm ([Eye Movements During Reading](https://sites.pitt.edu/~perfetti/Eye%20Movements%20During%20Reading.htm)).
12. **Do not design for a right-to-left diagonal.** For PT-BR the entry is top-left and gravity flows top-left → bottom-right. Mirrored (top-right entry) layouts are correct only for RTL scripts like Arabic/Hebrew ([2017 F-pattern revisit](https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content/); [perceptual span reversal in Hebrew](https://sites.pitt.edu/~perfetti/Eye%20Movements%20During%20Reading.htm)).

---

## Evidence-quality notes

- Strongest evidence: NN/g eye-tracking corpus (500+ participants, 750+ hours, three decades, replicated) and academic reading research (Rayner tradition).
- Weaker/heuristic: Gutenberg diagram and Z-pattern — print-era heuristics; NN/g has not validated them with the same rigor, and they apply only to low-hierarchy layouts.
- Context caveat: nearly all NN/g data is task-driven browsing by English readers on commercial sites; a single motivated reader on a personal PT-BR portal will skew more toward layer-cake/commitment than the averages suggest. The directional findings (left entry, front-loading, right-rail discounting) still apply.

---

## Sources

- https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content-discovered/ — Nielsen, original F-pattern study (2006), 232 users
- https://www.nngroup.com/articles/f-shaped-pattern-reading-web-content/ — Pernice, F-pattern 2017 revisit (incl. mirrored F for Arabic)
- https://www.nngroup.com/articles/text-scanning-patterns-eyetracking/ — Pernice, four scanning patterns ranked (2019)
- https://www.nngroup.com/articles/layer-cake-pattern-scanning/ — Pernice, layer-cake pattern (2019)
- https://www.nngroup.com/articles/horizontal-attention-leans-left/ — Fessenden, 80/20 left/right attention split (2017)
- https://www.nngroup.com/articles/banner-blindness-old-and-new-findings/ — Pernice, banner/right-rail blindness across 1997–2018 studies
- https://www.nngroup.com/articles/first-2-words-a-signal-for-scanning/ — Nielsen, 11-character nano-content finding (2009)
- https://www.nngroup.com/articles/how-users-read-on-the-web/ — Nielsen, 79% scan / 16% read, +124% usability (1997)
- https://www.nngroup.com/articles/pinball-pattern-search-behavior/ — Moran & Goray, pinball pattern on rich layouts (2019)
- https://www.nngroup.com/articles/zigzag-page-layout/ — Flaherty, zigzag image/text layouts eyetracking (2017)
- https://www.nngroup.com/reports/how-people-read-web-eyetracking-evidence/ — NN/g report, 2nd ed., 500+ participants / 750+ hours
- https://vanseodesign.com/web-design/3-design-layouts/ — Bradley, Gutenberg diagram / reading gravity / Z-pattern (Edmund Arnold attribution)
- https://sites.pitt.edu/~perfetti/Eye%20Movements%20During%20Reading.htm — Rayner-tradition summary: fixations 200–300 ms, saccades 7–9 chars, regressions 10–15%, perceptual span asymmetry and its reversal in Hebrew
- https://www.sciencedirect.com/topics/computer-science/fixation-duration — fixation duration overview
- https://www.neuronsinc.com/glossary/fixation-duration — fixation duration glossary (task-dependent ranges)
