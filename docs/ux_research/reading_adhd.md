# Reading and Focus with ADHD — Evidence Review for the RSS News Portal

**Date:** 2026-08-04
**Reader profile:** one user with astigmatism + myopia + ADHD, on Mac/Chrome (light theme today) and occasionally iPhone.
**Page today:** 3 columns (nav 240px | feed | context sidebar 320px), newspaper front page with ~40px lead headline, serif body bullets, per-section colors (teal/amber/red/blue), day chips, and counts everywhere ("8 histórias", "peso 370", "5 fontes"). User reports it is still hard to read and hard to concentrate.

---

## 1. TL;DR (BLUF)

The strongest, best-sourced levers for an ADHD reader are: **less simultaneous content, summaries first, chunked text with white space, user-controlled motion, and a bounded "done" state**. The page's biggest problems are not font choice — they are **three competing columns, dense headline rows, ubiquitous counts (attention traps), and color used as decoration instead of signal**. Bionic Reading, the most-marketed "ADHD reading hack," **failed its large test** and should not be adopted. A reading ruler, by contrast, has real supporting evidence. A "modo foco" (one story stack, one column, no counters) is directly supported by W3C COGA, GOV.UK "one thing per page," and single-tasking research.

---

## 2. Normative guidance: W3C COGA (the authoritative source)

W3C's *Making Content Usable for People with Cognitive and Learning Disabilities* is the normative reference for cognitive accessibility, and its Objective 5 is literally **"Help Users Focus."** Patterns directly relevant to this page ([W3C COGA](https://www.w3.org/TR/coga-usable/)):

- **Limit Interruptions (4.6.1)** — users with attention impairments need distractions removable/off by default.
- **Avoid Too Much Content (4.6.3)** — cognitive overload is the enemy; show less at once.
- **Use White Spacing (4.4.10)** — generous space between chunks reduces visual overwhelm.
- **Provide Summary of Long Documents (4.4.8)** — condensed versions let users get the point without sustained focus.
- **Break Media into Chunks (4.3.5)** — segment content into labeled, navigable sections.
- **Let Users Control When Content Moves or Changes (4.9.1)** — no autoplaying/auto-updating content.
- The **"Distractions" user story (3.5.1)** describes exactly this reader: needs task focus support and contextual re-anchoring after attention lapses.

**Verdict for this page:** COGA argues against the current everything-at-once, three-column, counts-everywhere layout.

## 3. UK Home Office posters — honest note

There is **no** "Designing for users with ADHD" poster. The official Home Office set covers anxiety, autism, deafness, dyslexia, low vision, motor disabilities, and screen readers ([ukhomeoffice.github.io/accessibility-posters](https://ukhomeoffice.github.io/accessibility-posters/), [GDS blog](https://accessibility.blog.gov.uk/2016/09/02/dos-and-donts-on-designing-for-accessibility/)). Any "Home Office ADHD poster" circulating online is a community derivative, not the government artifact — do not cite it as official. The closest official guidance (dyslexia + autism + anxiety posters) still converges on the same advice: simple layout, plain language, short chunks, no dense walls of text, minimal distraction.

## 4. What ADHD does to reading (the deficit to design around)

- Reading comprehension difficulties are common in adolescents/adults with ADHD; ADHD readers show **shallower text processing and weaker reinstatement of central ideas** while reading (think-aloud study: [Yeari & Lavie 2021](https://journals.sagepub.com/doi/abs/10.1111/ldrp.12237); profile study: [Miller et al., ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0891422214005125)).
- A 2022 scoping review finds ADHD comprehension deficits are **task-dependent** — longer, denser, less-structured tasks hurt most ([Parks et al. 2022, SAGE](https://journals.sagepub.com/doi/10.1177/10870547211068047)).
- Sustained attention interacts with **display medium and text spacing** in ADHD adolescents ([Stern & Shalev 2013, ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0891422212002272)) — spacing manipulations measurably change comprehension efficiency for ADHD readers.
- Strategy interventions that impose structure (e.g., TWA: think before/while/after reading, with written summarization) significantly improved expository text recall in ADHD students ([review context: Parks et al. 2022](https://journals.sagepub.com/doi/10.1177/10870547211068047)).

**Design translation:** the page must do the structuring the reader's executive function won't: pre-digested central ideas (summaries), explicit hierarchy, short chunks, generous spacing.

## 5. Techniques that WORK (with evidence)

### 5.1 Summary-first / BLUF / inverted pyramid
NN/g's long-standing research recommendation: put the conclusion first; even a reader who lasts one paragraph gets the point ([NN/g inverted pyramid](https://www.nngroup.com/articles/inverted-pyramid/), [NN/g 1996 original](https://www.nngroup.com/articles/inverted-pyramids-in-cyberspace/), [BLUF, Wikipedia](https://en.wikipedia.org/wiki/BLUF_(communication))). Matches COGA 4.4.8. For an ADHD reader who cannot sustain attention to the end, the first line must carry the payload.

### 5.2 Progressive disclosure
Show only primary content by default; defer the rest to interaction. Nielsen's technique reduces cognitive load and error rates; NN/g-cited research reports 20–40% faster task completion with better comprehension ([NN/g video](https://www.nngroup.com/videos/progressive-disclosure/), [UXPin overview](https://www.uxpin.com/studio/blog/what-is-progressive-disclosure/)). For a feed: collapsed sections, "show more" per subject, details behind one click.

### 5.3 One thing per page / single-task screens
GOV.UK's researched pattern: splitting content into one-question/one-task pages helps low-confidence users, works on mobile, and handles focus better ([GDS design notes](https://designnotes.blog.gov.uk/2015/07/03/one-thing-per-page/), [GOV.UK question pages pattern](https://design-system.service.gov.uk/patterns/question-pages/)). Directly transferable to a "modo foco": one story (or one subject) per screen.

### 5.4 White space, chunking, line length, spacing
- COGA 4.4.10 (white spacing) — see §2.
- WCAG 1.4.8 Visual Presentation: line spacing ≥1.5 within paragraphs, ≤80 characters per line, no full justification ([W3C Understanding 1.4.8](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html)).
- A century of line-length research converges on ~50–75 characters per line for comfortable reading ([Visible Language literature review](https://journals.uc.edu/index.php/vl/article/view/5765)); Baymard found text wider than 80 cpl was skipped 41% more often ([Baymard](https://baymard.com/blog/line-length-readability)).
- Spacing manipulations measurably affect ADHD reading efficiency ([Stern & Shalev 2013](https://www.sciencedirect.com/science/article/abs/pii/S0891422212002272)).

### 5.5 Reading rulers (line focus) — positive evidence
The CHI 2023 "Digital Reading Rulers" study (91 dyslexic + 86 non-dyslexic readers) found rulers **increased reading speed for dyslexic readers with no comprehension cost**, and many non-dyslexic readers benefited too; designs tested: Grey Bar, Lightbox, Shade, Underline ([ACM CHI 2023](https://dl.acm.org/doi/10.1145/3544548.3581367), [Readability Matters summary](https://readabilitymatters.org/articles/research-highlight-digital-reading-rulers)). Evidence is for dyslexia, not ADHD specifically — extrapolation to ADHD is plausible (visual anchoring) but not proven. Low-cost, opt-in feature worth shipping.

### 5.6 Reduced motion, no auto-updating content
Ambient motion is a cognitive tax; people with ADHD "might be so distracted by animated elements that they forget why they went to the site" ([web.dev motion guide](https://web.dev/learn/accessibility/motion), [Pope Tech](https://blog.pope.tech/2025/12/08/design-accessible-animation-and-movement/)). WCAG 2.2.2 requires pause/stop/hide for anything moving >5s; 2.3.3 (AAA) says interaction-triggered animation must be disableable ([W3C Understanding 2.3.3](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html)). `prefers-reduced-motion` must be honored, but for this reader the right default is **near-zero motion regardless of the OS setting** — reduced motion here is a focus feature, not just an aesthetic or vestibular one.

### 5.7 Progress + completion (dopamine done right)
The goal-gradient effect: effort intensifies as people approach a visible endpoint (café stamp cards, song-rating study — [Kivetz, Urminsky & Zheng 2006, JMR](https://journals.sagepub.com/doi/abs/10.1509/jmkr.43.1.39), [PDF](https://www.columbia.edu/~rk566/Session4/Goal-Gradient_Illusionary_Goal_Progress.pdf)). A **bounded session with visible progress toward "done"** ("3 of 8 stories read") harnesses this; an endless feed wastes it. Research on "purposefully finite" feeds — content split into fixed-size stacks with an explicit end state — shows reduced compulsive scrolling ([Teachable Feeds, arXiv](https://arxiv.org/pdf/2401.14000), [Design Frictions on Social Media, arXiv](https://arxiv.org/html/2407.18803v3)). This is the "inbox zero for news" pattern: a finite daily stack that can be *finished*.

## 6. Techniques that FAIL or are attention traps (report honestly)

### 6.1 Bionic Reading — does not work
- Readwise's large test (1,916 analyzed of 2,074 participants, paired design): **no significant effect on speed or comprehension**; bionic text was actually 2.6 wpm *slower* on average (~0.8%, not significant), comprehension identical at 88% ([Readwise study](https://blog.readwise.io/bionic-reading-results/)).
- Peer-reviewed follow-up: *"No, Bionic Reading does not work"*, Acta Psychologica 2024 ([ScienceDirect](https://www.sciencedirect.com/science/article/pii/S0001691824001811), [ResearchGate](https://www.researchgate.net/publication/380485238_No_Bionic_Reading_does_not_work)) — eye-tracking evidence, same conclusion.
- One small study reported 8–12% gains ([summarized at FocusWord](https://focus-word.com/blog/what-is-bionic-reading)), but the weight of evidence, including the only large pre-registered-style test, is null. **Do not build bold-prefix rendering into this page.** If the user ever wants it, it is a browser-extension experiment, not a product feature.

### 6.2 Counts and badges — engineered attention traps
Badge counts exploit **salience and urgency biases** and measurably increase checking/clicking behavior ([PLOS One badge study](https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0270888)); unread counters act as Zeigarnik-effect "open loops" demanding closure ([Slim, psychology of unread notifications](https://www.slim.am/en/blog/the-psychology-of-the-unread-notification)); notifications-on correlates with higher inattention and hyperactivity symptoms ([Beyond the Buzz, Media Psychology 2024](https://www.tandfonline.com/doi/full/10.1080/15213269.2024.2334025)). This page's "8 histórias", "peso 370", "5 fontes" on every block are exactly this pattern — a dozen open loops per screen, each a small pull on an ADHD reader's attention. Counts that serve ranking (weight) are backend signals, not reader content.

### 6.3 Infinite/unbounded feeds
Infinite scroll promotes normative dissociation (absorption with reduced self-awareness), compulsion, and lowered wellbeing ([Infinite Scrolling, Finite Satisfaction, arXiv](https://arxiv.org/pdf/2408.09601), [Design Frictions, arXiv](https://arxiv.org/html/2407.18803v3)). The antidote is a finite stack with an explicit end (§5.7).

### 6.4 Dark mode for this specific reader — caution
For astigmatism (and often myopia), light-on-dark triggers **halation** — bright text bleeding/glowing on dark backgrounds — and dark displays dilate the pupil, worsening focus for astigmatic eyes; dark text on light background is generally the sharper choice for dense reading with astigmatism ([BOIA](https://www.boia.org/blog/dark-mode-can-improve-text-readability-but-not-for-everyone), [Stéphanie Walter, dark-mode myths](https://stephaniewalter.design/blog/dark-mode-accessibility-myth-debunked/), [Level Access astigmatism guide](https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/)). The user is on light theme today — that is the right default for him. Keep dark mode available but consider softening it (dark-gray background, off-white text, heavier font weight) rather than pure black/white, per the same sources.

### 6.5 Colored overlays / tinted backgrounds — weak evidence
Colored overlays are popular for reading difficulties but the mainstream evidence base is weak/contested; the CHI reading-ruler work deliberately tested *rulers*, not tints, and rulers are the variant with measured benefit ([ACM CHI 2023](https://dl.acm.org/doi/10.1145/3544548.3581367)). Do not invest in tint themes as a focus intervention.

---

## 7. What this means for THIS page

### 7.1 The 3-column layout fights focus
Three simultaneous regions (nav | feed | context sidebar) is three competing claims on attention — the opposite of COGA's "Avoid Too Much Content" and "Limit Interruptions" (§2) and of GOV.UK's one-thing-per-page evidence (§5.3). Concretely:

- **Collapse the left nav to icons or a hidden drawer** while reading. It is wayfinding, not content; it does not need 240px of persistent text.
- **Make the context sidebar on-demand** (open per-story, or a keystroke), not permanent. A permanently visible "destaques" list is a standing invitation to abandon the current story mid-sentence — precisely the ADHD failure mode documented in §4 (weak reinstatement of central ideas after interruptions).
- Result: the feed column can hold a proper measure. At body 16–17px, cap the text column near **65–70ch** (§5.4); today's multi-column squeeze plus dense headline rows produces short choppy lines in some blocks and dense walls in others.

### 7.2 Density: fewer items, bigger gaps, summaries first
- Per-section blocks should show **lead + 3 headlines max** by default, with "mostrar mais" (progressive disclosure, §5.2). Dense 8-row headline lists are COGA anti-pattern 4.6.3.
- Every story card leads with a **1–2 sentence BLUF summary** (§5.1) — the RSS pipeline already runs AI summarization; surface it as the first line, not the headline soup.
- Increase inter-block white space and keep line-height ≥1.5 (§5.4, WCAG 1.4.8). White space is not wasted space for this reader; it is the chunking his executive function needs (§4).

### 7.3 What earns color: signal, not decoration
Four persistent section hues (teal/amber/red/blue) plus teal/amber accents means color no longer means anything — and every saturated element is a salience magnet (§6.2). Rule: **color only for the one thing that should pull the eye** — the current section's identity (a thin left border or small chip is enough) and unread state. Everything else: neutral ink on paper. This follows the same COGA/Home Office convergence on simple, low-noise layouts (§2–3).

### 7.4 Kill or demote the counts
"8 histórias", "peso 370", "5 fontes" are Zeigarnik open loops (§6.2). Actions:
- Remove weights and source counts from the reading surface entirely (keep in a debug/admin view).
- Replace story counts with **progress**: "3 de 8 lidas" only makes sense inside a bounded session (§5.7) — a count that closes a loop instead of opening one.

### 7.5 "Modo foco" — evidence-backed spec
A focus mode is not a gimmick for this reader; it is the COGA distraction user story implemented (§2). Spec, each element traceable to evidence above:

1. **One column, ~66ch, 17–18px, line-height ≥1.5** (§5.4) — nav and sidebar hidden.
2. **Finite daily stack**: today's N stories as cards, read one at a time, with "story 3 of 8" progress and an explicit **"Tudo lido" end screen** (§5.7, §6.3 — goal-gradient + finite-feed evidence).
3. **BLUF first**: summary sentence, then bullets, expand-for-detail (§5.1–5.2).
4. **Optional reading ruler** (Grey Bar or Shade variant from CHI 2023, §5.5), off by default, one keystroke to toggle.
5. **Zero motion, zero auto-refresh, zero badges** while in focus mode (§5.6, §6.2). New arrivals queue silently for the next session.
6. Light theme default; dark theme softened (dark-gray, not black) for the astigmatism case (§6.4).
7. **Not Bionic Reading** (§6.1).

### 7.6 Keep (these already align with evidence)
- Light theme as current default (§6.4).
- Strong single lead headline — one clear entry point per screen is good hierarchy; the problem is what surrounds it, not the lead itself.
- Quiet underlined action links — low-salience actions are correct (§7.3); do not "improve" them into buttons.

---

## Sources

- W3C COGA, *Making Content Usable for People with Cognitive and Learning Disabilities* — https://www.w3.org/TR/coga-usable/
- UK Home Office accessibility posters (official set; no ADHD poster exists) — https://ukhomeoffice.github.io/accessibility-posters/
- GDS, "Dos and don'ts on designing for accessibility" — https://accessibility.blog.gov.uk/2016/09/02/dos-and-donts-on-designing-for-accessibility/
- Yeari & Lavie (2021), centrality deficit in ADHD text comprehension — https://journals.sagepub.com/doi/abs/10.1111/ldrp.12237
- Parks et al. (2022), scoping review of ADHD reading comprehension — https://journals.sagepub.com/doi/10.1177/10870547211068047
- Stern & Shalev (2013), sustained attention, spacing, and display medium in ADHD — https://www.sciencedirect.com/science/article/abs/pii/S0891422212002272
- ADHD poor-comprehender profile study — https://www.sciencedirect.com/science/article/abs/pii/S0891422214005125
- NN/g, "Inverted Pyramid: Writing for Comprehension" — https://www.nngroup.com/articles/inverted-pyramid/
- NN/g, "Inverted Pyramids in Cyberspace" — https://www.nngroup.com/articles/inverted-pyramids-in-cyberspace/
- BLUF (communication) — https://en.wikipedia.org/wiki/BLUF_(communication)
- NN/g, Progressive Disclosure — https://www.nngroup.com/videos/progressive-disclosure/
- UXPin, progressive disclosure overview — https://www.uxpin.com/studio/blog/what-is-progressive-disclosure/
- GDS Design Notes, "One thing per page" — https://designnotes.blog.gov.uk/2015/07/03/one-thing-per-page/
- GOV.UK Design System, question pages — https://design-system.service.gov.uk/patterns/question-pages/
- WCAG, Understanding SC 1.4.8 Visual Presentation — https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html
- Nanavati & Bias, "Optimal Line Length in Reading — A Literature Review" — https://journals.uc.edu/index.php/vl/article/view/5765
- Baymard Institute, line length readability — https://baymard.com/blog/line-length-readability
- CHI 2023, "Digital Reading Rulers" — https://dl.acm.org/doi/10.1145/3544548.3581367
- Readability Matters, reading-rulers research highlight — https://readabilitymatters.org/articles/research-highlight-digital-reading-rulers
- web.dev, animation and motion accessibility — https://web.dev/learn/accessibility/motion
- Pope Tech, accessible animation — https://blog.pope.tech/2025/12/08/design-accessible-animation-and-movement/
- WCAG, Understanding SC 2.3.3 Animation from Interactions — https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html
- Kivetz, Urminsky & Zheng (2006), goal-gradient hypothesis — https://journals.sagepub.com/doi/abs/10.1509/jmkr.43.1.39 (PDF: https://www.columbia.edu/~rk566/Session4/Goal-Gradient_Illusionary_Goal_Progress.pdf)
- Teachable social media feeds ("purposefully finite" stacks) — https://arxiv.org/pdf/2401.14000
- Design Frictions on Social Media — https://arxiv.org/html/2407.18803v3
- Infinite Scrolling, Finite Satisfaction — https://arxiv.org/pdf/2408.09601
- Readwise Bionic Reading test (n≈2,000, null result) — https://blog.readwise.io/bionic-reading-results/
- "No, Bionic Reading does not work," Acta Psychologica (2024) — https://www.sciencedirect.com/science/article/pii/S0001691824001811 (also https://www.researchgate.net/publication/380485238_No_Bionic_Reading_does_not_work)
- FocusWord, Bionic Reading overview incl. the one contrary small study — https://focus-word.com/blog/what-is-bionic-reading
- PLOS One, badge notifications drive checking behavior — https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0270888
- Media Psychology (2024), "Beyond the Buzz" notification-disabling intervention — https://www.tandfonline.com/doi/full/10.1080/15213269.2024.2334025
- Slim, psychology of unread notifications (Zeigarnik effect) — https://www.slim.am/en/blog/the-psychology-of-the-unread-notification
- BOIA, dark mode readability caveats — https://www.boia.org/blog/dark-mode-can-improve-text-readability-but-not-for-everyone
- Stéphanie Walter, dark mode accessibility myths — https://stephaniewalter.design/blog/dark-mode-accessibility-myth-debunked/
- Level Access, designing for astigmatism — https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/
