# Typographic Hierarchy Research — h1–h6, the Newspaper Stack, and a Concrete Spec for the Portal

**Date:** 2026-08-04
**Scope:** Semantic heading structure, type scales, the editorial (newspaper) stack, body size and line-height/line-length evidence, vertical rhythm, ALL-CAPS cost, and `text-wrap: balance` — translated into an exact hierarchy spec for the rss-feed portal (Axum + HTMX, 3-column layout, PT-BR, dark-first).
**Out of scope:** Typeface selection (a separate font research track is running). This doc treats the current stack (Avenir Next Condensed headlines, Iowan Old Style summaries, SF Pro Text body) as given and specifies *sizes, weights, line-heights, casing, and spacing*.

---

## 1. Semantic heading structure (h1–h6) — the rules and why they matter

### 1.1 The rules

- **Headings must be nested by rank, not chosen for their looks.** The W3C WAI Page Structure tutorial: headings "should be nested by their rank"; skipping ranks (h2 → h4) "can be confusing and should be avoided where possible." It is only acceptable to "skip" upward when closing a subsection (an h2 can follow an h4). Source: [W3C WAI Headings tutorial](https://www.w3.org/WAI/tutorials/page-structure/headings/).
- **One h1 per page describing the main topic.** MDN's guidance on heading elements: avoid multiple `<h1>` per page and do not skip levels — "always start from `<h1>`, followed by `<h2>` and so on." Source: [MDN `<h1>`–`<h6>` reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements).
- **Heading level ≠ visual size.** Both WAI and MDN stress that levels convey document structure; visual prominence is CSS's job. This is the license we need for a newspaper front page: the *masthead* can be the h1 while the *capa* headline — visually the largest element — is an h2 inside its section. Sources: [W3C WAI](https://www.w3.org/WAI/tutorials/page-structure/headings/), [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements).

### 1.2 The evidence: headings are the #1 navigation instrument for screen-reader users

WebAIM's Screen Reader User Survey #10 (2023–24): **71.6% of respondents navigate long pages by headings** (vs. 13.6% using Find, 6.4% reading through), and **88.8% find heading levels very or somewhat useful**. Advanced users navigate by headings even more (78%). Sources: [WebAIM Survey #10](https://webaim.org/projects/screenreadersurvey10/), [summary at University of Rochester](https://www.rochester.edu/digital-accessibility/key-takeaways-from-webaims-screen-reader-user-survey-10/).

This is a single-reader personal portal, but the same structure that serves a screen reader serves *scanning*: a correct h1→h2→h3→h4 outline is also the portal's skimmable skeleton, and it costs nothing.

**Verdict for our portal:** one `<h1>` (portal masthead on the capa; the article title on an article page), `<h2>` per section (including the capa/lead section), `<h3>` for lead cards, `<h4>` for dense headline rows. Never express "smaller headline" by jumping to `<h5>`/`<h6>` — use a class.

---

## 2. Type scales — how to generate the size ramp

### 2.1 Modular scale ratios

A modular scale multiplies a base size by a fixed ratio. The named ratios most used on the web: **1.200 (Minor Third), 1.250 (Major Third), 1.333 (Perfect Fourth)**. Practitioner guidance: Perfect Fourth (1.333) gives strong contrast between levels (good for editorial/display pages); Major Third (1.25) is "the safer choice for complex UI" (dense interfaces). Sources: [Cieden — type scale types](https://cieden.com/book/sub-atomic/typography/different-type-scale-types), [Creative Market — What is a typographic scale?](https://creativemarket.com/blog/typographic-scale), [Baseline type scale generator](https://baseline.is/tools/type-scale-generator). Interactive tooling: [typescale.com-style generators](https://designyourway.net/t/type-scale-generator/). The modular-scale framework was formalized by Tim Brown and Scott Kellum (modularscale.com), per [Cieden](https://cieden.com/book/sub-atomic/typography/different-type-scale-types).

**Nuance for our case:** a news front page is *both* dense UI (headline rows, meta) *and* display editorial (capa hed). Evidence-aligned compromise: run the utility range (12→25px) on ~1.25, then take an extra step of the same ratio for the capa headline so the lead visibly outranks everything (see spec, §9). This mirrors how Material separates a restrained mid-ramp from oversized "display" roles (§2.2).

### 2.2 Material Design 3 type ramp — role names worth stealing

Material 3 defines **five roles: Display, Headline, Title, Body, Label**, each in large/medium/small. Roles are semantic ("Title is hierarchical text used for way-finding"; "Label styles are smaller, utilitarian styles"; "Display… used sparingly" for hero moments). Sources: [Material Design — Designing a Material theme: typography](https://m3.material.io/blog/design-material-theme-type), [Material Web typography tokens](https://material-web.dev/theming/typography/). Reference values (M3 defaults): Display 57/45/36, Headline 32/28/24, Title 22/16/14, Body 16/14/12, Label 14/12/11 px — per the [M3 typography cheatsheet](https://medium.com/@vosarat1995/material-3-you-typography-cheatsheet-ffc58c540181). Note **Body Large = 16px** — Google's default reading size (relevant to §4).

### 2.3 Apple HIG text styles

Apple's Dynamic Type default (Large) ramp: **Large Title 34pt, Title 1 28, Title 2 22, Title 3 20, Headline 17 (semibold), Body 17, Callout 16, Subhead 15, Footnote 13, Caption 12/11**. Sources: [Apple HIG — Typography](https://developer.apple.com/design/human-interface-guidelines/typography), [Learn UI Design — iOS font size guidelines](https://www.learnui.design/blog/ios-font-size-guidelines.html). Two portable lessons: (a) Apple's *body* is 17pt — bigger than our current 15px; (b) the ramp differentiates near-body levels by **weight, not just size** (Headline = same 17 as Body but semibold) — exactly the trick our dense headline rows need. Also relevant since we use SF Pro: Apple switches optical variants around 20pt (SF Pro Text ≤19pt, SF Pro Display ≥20pt), per [Learn UI Design](https://www.learnui.design/blog/ios-font-size-guidelines.html).

---

## 3. The newspaper editorial stack — names and functions

The full stack, per journalism references ([Poynter's journalism glossary](https://www.poynter.org/reporting-editing/2025/journalism-words-reporting-terms-off-the-record/), [bmon — "The hed, the dek, the lede and the nut graf"](https://www.bmon.co.uk/2019/04/the-hed-the-dek-the-lede-and-the-nut-graf/), [Carlana — "What do you call the parts of a story?"](https://blog.carlana.net/post/2020/article-bits/)):

| Term | What it is | Web equivalent in our portal |
|---|---|---|
| **Kicker / eyebrow** | Small label above the headline signaling category or context ([Poynter](https://www.poynter.org/reporting-editing/2025/journalism-words-reporting-terms-off-the-record/), [Wikipedia — Kicker](https://en.wikipedia.org/wiki/Kicker)) | Section/feed tag above heds, in the per-section accent color |
| **Hed** | The headline — "tells readers what the story is about… includes a verb" ([Poynter](https://www.poynter.org/reporting-editing/2025/journalism-words-reporting-terms-off-the-record/)) | h2/h3/h4 headline text |
| **Dek (subhed)** | Subtitle/standfirst expanding the hed ([bmon](https://www.bmon.co.uk/2019/04/the-hed-the-dek-the-lede-and-the-nut-graf/)) | Our serif summary line (Iowan Old Style) |
| **Byline** | Author/source credit ([Poynter](https://www.poynter.org/reporting-editing/2025/journalism-words-reporting-terms-off-the-record/)) | Feed name in the meta row |
| **Lede** | Opening sentence/paragraph carrying the most important facts ([Wikipedia — Lead paragraph](https://en.wikipedia.org/wiki/Lead_paragraph), [Poynter](https://www.poynter.org/reporting-editing/2025/journalism-words-reporting-terms-off-the-record/)) | First paragraph on the article/reader view |
| **Body** | The running text | Article/summary text |

The eyebrow pattern on the web: a short category label above the headline, "significantly smaller than the headline (often 12–14px)," commonly uppercase to read as a label, forming the scannable three-tier eyebrow→headline→body hierarchy. Sources: [UX Movement — Increasing headline clicks with eyebrow text](https://uxmovement.com/content/increasing-headline-clicks-with-eyebrow-text/), [Socialectric — Eyebrow text in web design](https://www.socialectric.com/insights/eyebrow-text-web-design). Markup note: a kicker is **not** a separate heading — keep it a `<p>`/`<span>` inside or before the h*, so it doesn't pollute the heading outline ([Sebastian Greger — How to correctly mark up a headline kicker](https://sebastiangreger.net/2022/01/how-to-correctly-mark-up-a-headline-kicker)).

**Why this matters here:** the user's portal already has all these slots (accent-colored tags, condensed heds, serif summaries, meta rows). Naming them and giving each *one* fixed style is the hierarchy: every card and row becomes the same predictable kicker→hed→dek→meta sandwich, which is what makes a newspaper page scannable.

---

## 4. Body size — the evidence

- **16px is the evidence-backed floor.** Smashing Magazine's canonical argument: 16px on screen approximates print body text at typical viewing distance (~20–28"), and studies comparing 10/12/14pt "repeatedly found that bigger font implies better readability." Source: [Smashing Magazine — "16 Pixels: For Body Copy. Anything Less Is A Costly Mistake"](https://www.smashingmagazine.com/2011/10/16-pixels-body-copy-anything-less-costly-mistake/).
- **The larger-still school (iA / Medium, ~20–21px).** iA's "100% Easy-2-Read" standard argues body text should match the size at which you'd comfortably read print at arm's length — in practice much larger than legacy web defaults; iA's own site and iA Writer use large responsive body sizes. Source: [iA — Responsive Typography: The Basics](https://ia.net/topics/responsive-typography-the-basics). Secondary reporting credits Medium's ~21px body with a claimed ~40% increase in reading time ([DeveloperUX — Typography in UX](https://developerux.com/2025/02/12/typography-in-ux-best-practices-guide/)) — **treat that number as weak evidence** (secondhand, no published methodology), but the direction (bigger reading text for long-form) is consistent with the controlled-study findings above and with a CHI 2016 study finding larger sizes preferred/faster up to a plateau ([Pielot — Optimal font size for web pages](https://pielot.org/2016/01/optimal-font-size-for-web-pages/)).
- Platform defaults agree: browser default and Material Body Large are 16px ([M3 cheatsheet](https://medium.com/@vosarat1995/material-3-you-typography-cheatsheet-ffc58c540181)); Apple Body is 17pt ([Apple HIG](https://developer.apple.com/design/human-interface-guidelines/typography)).

**Verdict for our portal:** the current **15px body is below every modern reference point** (16px browser/Material, 17pt Apple) and is a plausible direct cause of the reported reading discomfort. Raise interface body to **16px** and the *reading view* (article summaries/lede) to **18px** — split the difference between the 16px floor and the iA/Medium long-form school: card UI stays dense, sustained reading gets the bigger size.

---

## 5. Line-height × line-length interplay

- **Line spacing: 120–145% of the point size** for most text — [Butterick, Practical Typography — Line spacing](https://practicaltypography.com/line-spacing.html). Butterick adds that fonts that "run small" need less, and vice versa.
- **Line length: 45–90 characters** including spaces ([Butterick — Line length](https://practicaltypography.com/line-length.html)); classical print standard **45–75, ideal ~66** ([Bringhurst via webtypography.net §2.1.2](http://webtypography.net/2.1.2)); e-commerce/UX research narrows to **50–75 cpl** with too-long lines making it "difficult to gauge where the line starts and ends" and too-short lines breaking rhythm ([Baymard — Readability: The Optimal Line Length](https://baymard.com/blog/line-length-readability)).
- **WCAG 1.4.8 (AAA)** sets the accessibility ceiling: ≤80 characters per line, line spacing ≥1.5 within paragraphs, paragraph spacing ≥1.5× line spacing, no full justification. Source: [W3C — Understanding SC 1.4.8](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html).
- **The interplay:** the two variables trade off — Baymard notes overly long lines are hard to track back from line-end to next-line-start ([Baymard](https://baymard.com/blog/line-length-readability)); WCAG's Understanding doc makes the same tracking argument for capping length and raising spacing together ([W3C](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html)). Practical rule: at or near the max measure, sit at the top of Butterick's 120–145% band (≈1.45–1.6 on the web); short measures (sidebar, cards) can run tighter.
- **Headings are exempt from body leading:** multi-line display text at 1.5 falls apart visually; heading line-height should drop toward ~1.1–1.2 — Material's own ramp encodes this (e.g., Headline Large 32px has a 40px line = 1.25; Body 16px has 24px = 1.5), per the [M3 token values](https://material-web.dev/theming/typography/) and the [cheatsheet](https://medium.com/@vosarat1995/material-3-you-typography-cheatsheet-ffc58c540181). Condensed faces (our Avenir Next Condensed) tolerate the tight end.

**Verdict for our portal:** the center column should enforce measure with `max-width` on *text blocks*, not just the column: at 16px SF Pro, ~65ch ≈ 600–640px, which fits the current center column. Body/dek line-height 1.5–1.6; heds 1.1 (capa) to 1.3 (rows).

---

## 6. Vertical rhythm — space before vs. after headings

- **More space above a heading than below it**, so the heading binds to the content it introduces (Gestalt proximity). Rule of thumb from UI-typography practice: space *before* a heading ≈ **2× paragraph spacing**; space *after* ≈ same as normal paragraph spacing. Sources: [Imperavi — Vertical rhythm (UI Typography book)](https://imperavi.com/books/ui-typography/principles/vertical-rhythm/), [Imperavi — Vertical Rhythm in Typography](https://imperavi.com/blog/vertical-rhythm-in-typography/).
- Smaller subheads should sit *closer* to their text than big section heads do — proximity scales with rank ([BuninUX — Typography: Spacing](https://blog.prototypr.io/typography-spacing-6b33dd1992a2)).
- Build spacing from a **fixed spacing scale** (multiples of one unit) rather than per-element eyeballing; strict baseline-grid rhythm is of debatable value on the web, but a consistent spacing scale is not ([Zell Liew — Were we wrong about vertical rhythm?](https://zellwk.com/blog/wrong-about-vertical-rhythm/), [Imperavi](https://imperavi.com/blog/vertical-rhythm-in-typography/)).

**Verdict for our portal:** adopt an 8px spacing scale (4 for micro-gaps). Section h2: 40px above / 16 below. Card h3: 8 above kicker→hed, 4 hed→dek. Headline rows: 12px row padding — the row list reads as one block under its section head.

---

## 7. ALL-CAPS — the reading cost, and when caps are fine

- Continuous all-caps text reads **~10–20% slower** (Tinker's classic studies; modern replications find lowercase read >13% faster), because uniform rectangles destroy word shape and force letter-by-letter processing. Sources: [The Team W — Revisiting ALL UPPERCASE vs. mixed case](https://www.blog.theteamw.com/2017/11/03/revisiting-all-uppercase-letters-vs-upper-and-lower-case/), [Psych News Daily — All caps harder to read](https://psychnewsdaily.com/new-study-shows-all-caps-harder-to-read-2/).
- The cost is worse for dyslexic readers (13–18% additional slowdown) ([Brickfield — Why ALL CAPS creates reading barriers](https://brickfield.ie/2026/03/26/why-all-caps-text-creates-reading-barriers/)).
- **But**: for *glanceable single words/short labels*, NN/g found uppercase was actually **faster to glance-read** than lowercase — "fonts to support glancing at individual words should be uppercase." Source: [NN/g — Typography for Glanceable Reading](https://www.nngroup.com/articles/glanceable-fonts/). This is exactly the kicker/eyebrow/nav-label use case; eyebrow practice confirms uppercase as the label convention at 12–14px ([Socialectric](https://www.socialectric.com/insights/eyebrow-text-web-design)).
- Wide letter-spacing helps single-word recognition but hurts continuous reading rate ([PMC — Wider letter-spacing facilitates word processing but impairs reading rates](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7090332/)) — i.e., tracking belongs on caps *labels*, never on body.

**Verdict for our portal:** caps are allowed in exactly three places — kickers, section h2 labels, nav-rail items — always ≤2–3 words, 11–13px, +0.06–0.08em letter-spacing, 600 weight. Never caps a hed, dek, or button of sentence length.

---

## 8. `text-wrap: balance` for headlines

- `text-wrap: balance` makes the browser equalize line lengths in short blocks — recommended specifically for "headings, captions, and blockquotes"; it has a computation cost and a line-count limit (~6 lines in Chromium), so it must **not** be applied to body text. Source: [Chrome for Developers — CSS `text-wrap: balance`](https://developer.chrome.com/docs/css-ui/css-text-wrap-balance).
- Baseline-supported across modern browsers since 2023 (~88%+ support); `text-wrap: pretty` is the complementary option for paragraphs (avoids orphans). Sources: [MDN — text-wrap](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-wrap), [Stephanie Stimac — balance vs. pretty](https://blog.stephaniestimac.com/posts/2023/10/css-text-wrap/).

**Verdict for our portal:** `h1,h2,h3,h4,.hed { text-wrap: balance; }` and `.dek, .article-body p { text-wrap: pretty; }`. This single rule kills the ragged one-word-orphan capa headline problem at zero content cost — it degrades gracefully where unsupported ([MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-wrap)).

---

## 9. THE SPEC — concrete hierarchy for the rss-feed portal

Scale rationale: base 16px, utility steps on ~1.25 (Major Third — the "safer choice for complex UI," [Cieden](https://cieden.com/book/sub-atomic/typography/different-type-scale-types)): 12 → 13 → 16 → 20 → 25 → 31 → 39. The capa hed takes the top step (39px ≈ Material's Display-small territory, "used sparingly" for hero moments — [Material](https://m3.material.io/blog/design-material-theme-type)). Weight, not size, separates near-body ranks (Apple's Headline-vs-Body trick — [Apple HIG](https://developer.apple.com/design/human-interface-guidelines/typography)).

| Role (element) | HTML | Font (current stack) | Size | Weight | Line-height | Case / tracking | Space above / below |
|---|---|---|---|---|---|---|---|
| Portal title (masthead) | `h1` (capa only) | Avenir Next Condensed | 31px | 700 | 1.1 | Mixed case | 24 / 24px |
| Capa (lead) headline | `h2.hed-capa` | Avenir Next Condensed | 39px (`clamp(31px, 4vw, 39px)`) | 700 | 1.08 | Mixed; `text-wrap: balance` | 8 (below kicker) / 8px |
| Capa dek | `p.dek` | Iowan Old Style | 18px | 400 | 1.5 | Mixed; `text-wrap: pretty` | 8 / 8px |
| Section title ("Destaques", per-subject) | `h2.section-label` | SF Pro Text | 13px | 700 | 1.2 | UPPERCASE, +0.08em, section accent color + hairline rule | 40 / 16px |
| Card headline (section lead) | `h3.hed-card` | Avenir Next Condensed | 25px | 600 | 1.15 | Mixed; balance | 8 / 4px |
| Row headline (dense list) | `h4.hed-row` | SF Pro Text | 16px | 600 | 1.35 | Mixed | 12px row padding |
| Dek / summary (cards) | `p.dek` | Iowan Old Style | 16px | 400 | 1.5 | Mixed; pretty | 4 / 8px |
| Kicker / eyebrow | `p.kicker` (NOT a heading — [Greger](https://sebastiangreger.net/2022/01/how-to-correctly-mark-up-a-headline-kicker)) | SF Pro Text | 12px | 600 | 1.2 | UPPERCASE, +0.06em, section accent | 0 / 4px |
| Meta (byline · time · feed) | `p.meta` / `<time>` | SF Pro Text | 12px | 400 (feed name 500) | 1.35 | Mixed, muted ink | 8 / 0px |
| Body / lede (reader view) | `p` | SF Pro Text (or reading serif per font research) | 18px | 400 | 1.6 | Mixed; measure 60–66ch `max-width` | paragraph gap 16px (≥1.5× per [WCAG 1.4.8](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html)) |
| Interface body (sidebar, nav) | `p`, `li` | SF Pro Text | 14–16px (nav 14) | 400–500 | 1.45 | Nav labels may be caps 12px | 8px gaps |

Supporting rules, each traceable to §§1–8:

1. **Outline:** `h1` masthead → `h2` per section (capa included) → `h3` lead cards → `h4` rows; no skipped levels ([W3C WAI](https://www.w3.org/WAI/tutorials/page-structure/headings/), [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)). Article page: `h1` = story hed.
2. **Visual rank is a class, semantic rank is the tag** — the 13px `h2.section-label` outranks the 25px `h3` semantically, and that is correct ([W3C WAI](https://www.w3.org/WAI/tutorials/page-structure/headings/)).
3. **Measure:** center-column text blocks `max-width: 62ch` (≈ Baymard 50–75 / Bringhurst ideal 66 / WCAG ≤80 — [Baymard](https://baymard.com/blog/line-length-readability), [webtypography.net](http://webtypography.net/2.1.2), [W3C](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html)). The 320px sidebar (~38ch at 14px) runs tighter leading (1.45) per the length↔leading tradeoff (§5).
4. **Spacing scale:** 4/8/12/16/24/32/40; heading space-above ≈ 2× space-below ([Imperavi](https://imperavi.com/books/ui-typography/principles/vertical-rhythm/)).
5. **Caps whitelist:** kicker, section label, nav item — nothing else (§7; [NN/g](https://www.nngroup.com/articles/glanceable-fonts/), [Team W](https://www.blog.theteamw.com/2017/11/03/revisiting-all-uppercase-letters-vs-upper-and-lower-case/)).
6. **Single biggest fix:** body 15px → 16px interface / 18px reading view (§4; [Smashing](https://www.smashingmagazine.com/2011/10/16-pixels-body-copy-anything-less-costly-mistake/), [Apple HIG](https://developer.apple.com/design/human-interface-guidelines/typography), [iA](https://ia.net/topics/responsive-typography-the-basics)).

### Evidence-strength notes

- **Strong (multiple independent/controlled sources):** 16px+ body; 45–80 cpl measure; caps slowdown in continuous text; heading navigation dominance (WebAIM n≈1,500 survey); more-space-above-headings.
- **Convention, not experiment:** modular-scale ratios, the newspaper stack names, eyebrow styling norms — consistent across practice but aesthetic, not measured.
- **Weak / treat as directional only:** Medium's "21px raised reading time 40%" (secondhand, unpublished methodology — [DeveloperUX](https://developerux.com/2025/02/12/typography-in-ux-best-practices-guide/)); eyebrow "boosts clicks" claims ([UX Movement](https://uxmovement.com/content/increasing-headline-clicks-with-eyebrow-text/)) are practitioner assertions without published data.

---

## Sources

- https://www.w3.org/WAI/tutorials/page-structure/headings/
- https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements
- https://webaim.org/projects/screenreadersurvey10/
- https://www.rochester.edu/digital-accessibility/key-takeaways-from-webaims-screen-reader-user-survey-10/
- https://cieden.com/book/sub-atomic/typography/different-type-scale-types
- https://creativemarket.com/blog/typographic-scale
- https://baseline.is/tools/type-scale-generator
- https://designyourway.net/t/type-scale-generator/
- https://m3.material.io/blog/design-material-theme-type
- https://material-web.dev/theming/typography/
- https://medium.com/@vosarat1995/material-3-you-typography-cheatsheet-ffc58c540181
- https://developer.apple.com/design/human-interface-guidelines/typography
- https://www.learnui.design/blog/ios-font-size-guidelines.html
- https://www.poynter.org/reporting-editing/2025/journalism-words-reporting-terms-off-the-record/
- https://www.bmon.co.uk/2019/04/the-hed-the-dek-the-lede-and-the-nut-graf/
- https://blog.carlana.net/post/2020/article-bits/
- https://en.wikipedia.org/wiki/Kicker
- https://en.wikipedia.org/wiki/Lead_paragraph
- https://uxmovement.com/content/increasing-headline-clicks-with-eyebrow-text/
- https://www.socialectric.com/insights/eyebrow-text-web-design
- https://sebastiangreger.net/2022/01/how-to-correctly-mark-up-a-headline-kicker
- https://www.smashingmagazine.com/2011/10/16-pixels-body-copy-anything-less-costly-mistake/
- https://ia.net/topics/responsive-typography-the-basics
- https://developerux.com/2025/02/12/typography-in-ux-best-practices-guide/
- https://pielot.org/2016/01/optimal-font-size-for-web-pages/
- https://practicaltypography.com/line-spacing.html
- https://practicaltypography.com/line-length.html
- http://webtypography.net/2.1.2
- https://baymard.com/blog/line-length-readability
- https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html
- https://imperavi.com/books/ui-typography/principles/vertical-rhythm/
- https://imperavi.com/blog/vertical-rhythm-in-typography/
- https://blog.prototypr.io/typography-spacing-6b33dd1992a2
- https://zellwk.com/blog/wrong-about-vertical-rhythm/
- https://www.blog.theteamw.com/2017/11/03/revisiting-all-uppercase-letters-vs-upper-and-lower-case/
- https://psychnewsdaily.com/new-study-shows-all-caps-harder-to-read-2/
- https://brickfield.ie/2026/03/26/why-all-caps-text-creates-reading-barriers/
- https://www.nngroup.com/articles/glanceable-fonts/
- https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7090332/
- https://developer.chrome.com/docs/css-ui/css-text-wrap-balance
- https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-wrap
- https://blog.stephaniestimac.com/posts/2023/10/css-text-wrap/
