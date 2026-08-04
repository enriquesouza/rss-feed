# Reading With Astigmatism + Myopia — Evidence Review

**Scope:** low-vision typography science applied to one reader (astigmatism + myopia + ADHD) using a personal news portal on a Mac (Chrome) and iPhone.
**Date:** 2026-08-04.
**Rule of this document:** every claim carries a source. Where the popular design advice is unsupported, weakly supported, or was *contradicted* by study, it is flagged as such — those negative findings are the most useful part of this file.

---

## 1. Contrast polarity: the strongest evidence in the whole file

### 1.1 The positive-polarity advantage is real and replicated

- Piepenbrock, Mayr & Buchner measured pupil size and proofreading performance across polarities: **pupils were smaller and proofreading performance was better with positive polarity (dark text on light background)**, and the authors attribute the effect to display luminance — brighter display → smaller pupil → sharper retinal image → better detail perception. ([PubMed 25135324](https://pubmed.ncbi.nlm.nih.gov/25135324/))
- The same group found the positive-polarity advantage **for both younger (18–33) and older (60–85) adults**, in visual-acuity (Landolt C) *and* proofreading tasks, in participants with normal or corrected-to-normal vision and no eye disease. ([HHU Düsseldorf PDF](https://www.psychologie.hhu.de/fileadmin/redaktion/Oeffentliche_Medien/Fakultaeten/Mathematisch-Naturwissenschaftliche_Fakultaet/Psychologie/AAP/Publikationen/2013/Piepenbrock-2013-Positive_display_polarity_is_.pdf))
- Critically for a dense news page: **the positive-polarity advantage increases linearly as character size decreases** — i.e. the smaller the type, the more light mode wins. ([Piepenbrock, Mayr & Buchner, *Human Factors* 2014, "Positive Display Polarity Is Particularly Advantageous for Small Character Sizes"](https://journals.sagepub.com/doi/abs/10.1177/0018720813515509))
- The effect is not an artifact of room lighting or colour: polarity affected performance **irrespective of ambient illumination and colour contrast**. ([Buchner & Baumgartner, *Ergonomics*](https://www.researchgate.net/publication/6321309_Text_-_Background_polarity_affects_performance_irrespective_of_ambient_illumination_and_colour_contrast))
- NN/g's synthesis: light mode outperformed dark mode across visual-acuity and reading tasks for people with normal vision, and the advantage grows with smaller fonts — but they still recommend *offering* a dark mode rather than forcing either. ([NN/g, "Dark Mode vs. Light Mode"](https://www.nngroup.com/articles/dark-mode/))

### 1.2 The counter-evidence — do not overstate the case

- **Negative finding.** A 2024 study with 104 participants (52 under 60, 52 aged 60+) doing 24 chart-reading trials (bar, line, scatter) found **neither polarity consistently outperformed the other** for accuracy or response time; e.g. for response time 44.2% of older and 53.8% of younger adults did better in positive polarity, the rest better in negative. Conclusion: "negative and positive contrast polarity are similarly capable of improving user performance," and the recommendation is to **offer both**. Note the task was *visualization reading*, not continuous text. ([arXiv 2409.10841](https://arxiv.org/html/2409.10841v2))
- The same study found **preference did not predict performance** — people often preferred the mode that made them slower. Practical consequence: "which one feels nicer" is not evidence for this page; only a timed comparison is. ([arXiv 2409.10841](https://arxiv.org/html/2409.10841v2))
- **Dark mode genuinely wins for some eyes.** A subset of people with low vision read **10% to 50% faster with bright letters on a black background**, because light scatter in diseased/cloudy ocular media degrades a bright-background image more. ([Legge & Chung, "Reading Digital with Low Vision", PMC5726769](https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/); [NN/g](https://www.nngroup.com/articles/dark-mode/))
- NN/g also notes dark mode benefits users with **cloudy ocular media (cataracts)**, while people with **central-vision impairments were not significantly affected by polarity**. ([NN/g](https://www.nngroup.com/articles/dark-mode/))

**Reading of the evidence:** for a corrected astigmat/myope reading small dense text, positive polarity is the better default, and the advantage is *largest exactly where this page is weakest* (small headline rows, 17–18px bullets, meta counts). But the case is "strong default", not "dark mode is forbidden".

---

## 2. Astigmatism, halation and the ~50% claim — verify before quoting

### 2.1 Prevalence: the "50%" figure is roughly defensible but definition-dependent

- Systematic review + meta-analysis: astigmatism is the **most common refractive error**, pooled adult prevalence **40% (95% CI 34–47%)**, ranging **8–62%** across populations and higher in people 70+. ([Zhang et al., *Optometry and Vision Science* 2023, "Epidemiology and Burden of Astigmatism"](https://doi.org/10.1097/OPX.0000000000001998); [full text PDF](https://www.researchgate.net/publication/368334523_Epidemiology_and_Burden_of_Astigmatism_A_Systematic_Literature_Review/fulltext/645bdcbafbaf5b27a4ba1068/Epidemiology-and-Burden-of-Astigmatism-A-Systematic-Literature-Review.pdf))
- The accessibility-industry framing: nearly half the population has **≥0.5 D**, ~10% have **≥1 D**, ~8% have **≥1.5 D**; a UK study of 11,000+ spectacle wearers found **47.4% with ≥0.75 D in at least one eye**, 24.1% in both. ([Level Access, "Astigmatism and Web Accessibility"](https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/))
- **Caveat to state plainly:** "50% of people have astigmatism" is true only at a very low diopter cut-off. Clinically meaningful astigmatism (>1.00 D) is more like **15–20%**. ([Pacific Eye Surgeons summary](https://www.paceyemd.com/blog/what-percentage-of-people-have-astigmatism-understanding-its-impact/)) The design conclusion does not depend on the headline number — it depends on *this* reader having it.

### 2.2 Halation: mechanistically plausible, but the evidence is practitioner-level, not experimental

- The mechanism as described in accessibility literature: bright glyphs on a dark field are a high-contrast edge; in dark surroundings the **pupil dilates**, light enters through a wider corneal/lens aperture, and an irregularly curved (astigmatic) cornea refracts peripheral rays differently — so the bright stroke appears to bleed into the dark background ("halation"), letters look thicker, fuzzier, shimmering, and reading slows. ([techealthinfo, "Dark Mode and Astigmatism"](https://techealthinfo.com/dark-mode-and-astigmatism-5-tweaks-to-reduce-eye-strain/); [Luke Harris, "Astigmatism and dark mode"](https://www.lkhrs.com/blog/astigmatism-and-dark-mode/))
- Level Access explicitly names it as a design failure automated contrast checkers miss: avoid **pure white text on pure black**, because it "creates a visual fuzzing effect for people with astigmatism called 'halation'". ([Level Access](https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/))
- H Locke's widely-cited accessibility piece makes the same argument — that dark mode introduces accessibility problems for astigmatic readers rather than solving them. ([Medium, "Why dark mode causes more accessibility issues than it solves"](https://medium.com/@h_locke/why-dark-mode-causes-more-accessibility-issues-than-it-solves-54cddf6466f5))
- **Honest limitation:** the sources above are blogs, optometry-practice posts and accessibility consultancies, not controlled trials on astigmatic readers. The *peer-reviewed* leg of this argument is the pupil-size mechanism from Piepenbrock — smaller pupil under a bright display gives a sharper retinal image ([PubMed 25135324](https://pubmed.ncbi.nlm.nih.gov/25135324/)) — which is exactly the mechanism that would relieve an astigmatic eye. Treat "halation" as *well-motivated but not directly measured in a published astigmatism trial*.

---

## 3. Font weight: the "make it bolder" instinct is largely unsupported

- **Negative finding, direct.** Ten participants with central vision loss (7 AMD, 2 Stargardt, 1 toxoplasmic chorioretinitis) read RSVP sentences in Courier at six stroke weights (0.27×, 0.72×, 1×, 1.48×, 1.89×, 3.04× standard). **Reading speeds were essentially identical from 0.72× to 1.89×**; only the thinnest (0.27×) and the heaviest (3.04×) were significantly slower. **No weight beat the standard.** The paper's title is the finding: "Bolder print does not increase reading speed in people with central vision loss." ([PMC6287928](https://pmc.ncbi.nlm.nih.gov/articles/PMC6287928/))
- The same pattern in normal vision: the **lightest (0.27×) and boldest (3.04×) weights reduced reading speed at the fovea**; at 10° eccentricity the two boldest weights (1.89× and 3.04×) also hurt. ([PMC3642228, "The Effect of Letter-stroke Boldness on Reading Speed in Central and Peripheral Vision"](https://pmc.ncbi.nlm.nih.gov/articles/PMC3642228/))
- Over-bolding has a specific failure mode: adding blackness uniformly across the stroke distorts the black/white area distribution and **impairs letter recognition** — high stroke contrast is worse for bold fonts. ([*Applied Ergonomics*, "High letter stroke contrast impairs letter recognition of bold fonts"](https://www.sciencedirect.com/science/article/pii/S0003687021001460))
- What *did* help low-vision readers in a controlled reading-acuity experiment (55 participants: 15 blurred vision, 11 central vision loss, 15 peripheral vision loss, 14 controls; six variants of the Zed typeface): **letter width**, not weight. Width 140 was optimal for the central- and peripheral-loss groups, width 160 for blurred-vision and normal readers, with performance **plateauing** beyond that. Weight and tracking did not produce clear independent benefits. ([Typotheque, "Designing fonts with low-vision readers in mind"](https://www.typotheque.com/research/designing-fonts-with-low-vision-readers-in-mind))

**Takeaway:** the actionable rule is **eliminate thin/light weights** (300 and below, and hairline serifs), not "go bold everywhere". Regular/medium (400–500) sits in the flat part of the curve; wider letterforms buy more than heavier ones.

---

## 4. Letter, word and line spacing: mostly a myth, with one WCAG-shaped exception

- **Negative finding.** Increasing letter spacing beyond standard — which does reduce crowding and does improve *letter identification* — **does not increase reading speed** in central or peripheral vision, and spacing beyond normal text separations actually **slows reading**, because the extra spacing pushes text further into lower-acuity peripheral vision. ([Chung, *IOVS* 2002 / PubMed 11923275](https://pubmed.ncbi.nlm.nih.gov/11923275/); [IOVS article](https://iovs.arvojournals.org/article.aspx?articleid=2200181))
- Replicated in people with actual central vision loss (14 observers): **extra letter spacing does not improve reading speed**. ([PMC3429790 / PubMed 22842309](https://pmc.ncbi.nlm.nih.gov/articles/PMC3429790/))
- Line spacing: the effect is **statistically significant but small** — average reading speed rose by only **7.1 words/min** going from standard to double interline spacing in low-vision patients with central field loss. ([PubMed 19834038](https://pubmed.ncbi.nlm.nih.gov/19834038/))
- Nevertheless, WCAG 2.2 SC 1.4.12 requires that content survive user-applied spacing of **line-height ≥ 1.5×**, **paragraph spacing ≥ 2×**, **letter-spacing ≥ 0.12×** and **word-spacing ≥ 0.16×** the font size — derived from the McLeish study (tested 0.04–0.25 em). ([W3C, Understanding SC 1.4.12 Text Spacing](https://www.w3.org/WAI/WCAG22/Understanding/text-spacing.html))

**Takeaway:** don't buy readability with tracking. Generous **line-height (~1.5)** and **paragraph separation** are cheap and mildly positive; **letter-spacing tweaks on body text are not a fix** and can backfire. The one place tracking is defensible is all-caps micro-labels, which is a legibility repair, not a reading-speed gain.

---

## 5. Size, magnification and the "resize" contract

- Normally sighted readers have a **10-fold "fluent range"** of print sizes (x-height 0.2°–2°) over which speed is maximal; low vision means the required print size sits above that range and must be magnified into it. ([PMC5726769](https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/))
- **Acuity reserve rule:** most people read at maximum rate when print is **at least 2× their threshold print size** (critical print size, CPS, from MNRead-type charts). If fluent reading is the goal, the required magnification roughly doubles versus a spot-reading prescription. ([Optometry Times, "Approach patients with low vision on an individual basis"](https://www.optometrytimes.com/view/approach-patients-with-low-vision-on-an-individual-basis); [Xiong, Legge et al., "Reading Acuity as a Predictor of Low-Vision Reading Performance", PMC6181187](https://pmc.ncbi.nlm.nih.gov/articles/PMC6181187/))
- **18–20 pt is cited as a practical minimum** for low-vision accessibility, and 20/60 acuity needs print ~3× standard. ([PMC5726769](https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/))
- Halving viewing distance (40 cm → 20 cm) doubles apparent size — free magnification — but requires accommodation most adults eventually cannot supply. ([PMC5726769](https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/))
- The hard constraint on a mobile portal: **print size, characters per line, line separation and font interact** — on small screens you cannot simultaneously get enough magnification and enough characters per line, and reading collapses. ([PMC5726769](https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/); [Technical Report: "Digital Reading with Low Vision: Principles for Selecting Display Size", PMC9357187](https://ncbi.nlm.nih.gov/pmc/articles/PMC9357187))
- WCAG 2.2 SC 1.4.4 requires text to resize to **200%** without loss of content or functionality; beyond 200%, full page zoom is more effective than text-only resize, and layouts must not truncate or overlap. ([W3C, Understanding SC 1.4.4 Resize Text](https://www.w3.org/WAI/WCAG22/Understanding/resize-text.html))

---

## 6. Contrast targets: WCAG 2 ratios are the wrong tool for dark mode

- APCA reports a perceptually uniform lightness contrast (Lc) and explicitly states WCAG 2.x **"far overstates contrast for dark colors"** and **"cannot provide useful guidance when designing dark mode."** ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html); [Why APCA](https://git.apcacontrast.com/documentation/WhyAPCA))
- APCA reference levels: **Lc 90 preferred for body-text columns** (min 14px/400), **Lc 75 minimum for body columns** (min 18px/400), **Lc 60 minimum for content text** (24px/400 or 16px/700), **Lc 45 for headlines** (36px/400 or 24px/700), **Lc 30** for placeholder/disabled, **Lc 15** the practical point of invisibility. ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html); [APCA Readability Criterion](https://www.readtech.org/ARC/tests/visual-readability-contrast/))
- The size/weight ladder at Lc 60 is: **48px/200, 36px/300, 24px/400, 21px/500, 18px/600, 16px/700** (reference font Helvetica) — i.e. **the thinner the weight, the larger the type must be**, because thin small strokes lower perceived contrast and must be compensated with more lightness difference. ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html))
- APCA uses signed values for dark mode (e.g. Lc −60) and gives a **preliminary dark-mode maximum around Lc −90 for large fonts** — an explicit acknowledgement that *maximum* contrast is not optimal on dark backgrounds. ([APCA Easy Intro](https://git.apcacontrast.com/documentation/APCAeasyIntro.html))

---

## 7. Blue text and "blue light" — the popular claim is weaker than advertised

- The common claim: blue text is harder to read because of **low S-cone density plus longitudinal chromatic aberration** — the lens fails to focus short wavelengths on the retina, so blue letter edges look fuzzy. ([kidspattern, "Why is blue text harder to read?"](https://kidspattern.com/learn/why-is-blue-text-harder-to-read/))
- The classic UX rule: **avoid dark or highly saturated blue for text or fine lines on black or other dark backgrounds**. ([UXmatters, "Applying Color Theory to Digital Displays"](https://www.uxmatters.com/mt/archives/2007/01/applying-color-theory-to-digital-displays.php))
- **Negative finding — this is the important one.** "Vision is protected against blue defocus": the eye's own **monochromatic aberrations mitigate the impact of longitudinal chromatic aberration on S-cone contrast**, and observers judged blue images defocused by the chromatic-difference equivalent as **sharper** than green or grayscale images defocused by the same amount. Optical depth-of-focus plus neural tolerance mean **blue detail is not the acuity catastrophe the folklore implies**. ([PMC7801416](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7801416/))
- Text/background colour combinations do measurably perturb the **accommodative response** dynamics, so colour is not neutral — but the effect is about focusing dynamics, not gross illegibility. ([*Vision Research*, "Short-term effects of text-background color combinations on the dynamics of the accommodative response"](https://www.sciencedirect.com/science/article/pii/S0042698919302123))

**Takeaway:** do not rewrite the palette on a "blue light / chromatic aberration" theory. The defensible reason to avoid saturated blue *text* is **luminance contrast**: a saturated blue is dark, so blue-on-white body copy is dim and blue-on-dark is a thin glowing stroke. Keep blue for **section identity**, not for reading surfaces.

---

## 8. Glare, pure white, and the environment

- **Measured evidence exists for glare.** EEG + eye-tracking with 18 participants reading on **glossy (glare) vs matte (non-glare)** monitors: significantly **longer fixations** on glare monitors; with **black backgrounds** — where reflections stand out most — participants **looked away from the text more often to look at their own reflection**, and beta power (15–20 Hz) decreased less, indicating **less task engagement**. Subjective illegibility ratings and reading time showed **no significant difference**, i.e. the cost was real but invisible to self-report. ([PMC10506577](https://pmc.ncbi.nlm.nih.gov/articles/PMC10506577/))
  - Note the direction: this is **evidence against black backgrounds on a glossy display** — a MacBook/iPhone screen is glossy. For an ADHD reader, "looked away from the sentences more frequently" is precisely the failure mode to avoid.
- **Weak evidence.** The advice to use **off-white instead of pure white** (and near-black instead of pure black) is widespread in design writing and optometry-practice blogs — the reasoning being lower total emitted light, less glare, and less pupil re-adjustment on scroll — but it rests on practitioner opinion, not controlled trials. ([Design for Ducks, "Color's effect on readability and vision fatigue"](https://designforducks.com/colors-effect-on-readability-and-vision-fatigue/); [Stoney Creek Optometry](https://stoneycreekoptometry.com/is-dark-mode-better-for-your-eyes/); [W3C WAI-IG list post on white-background glare reducing perceived character sharpness](https://lists.w3.org/Archives/Public/w3c-wai-ig/2003JanMar/0883.html))
  - Tension to respect: dimming the background **reduces the very luminance that produces the pupil constriction** that Piepenbrock credits for the positive-polarity advantage ([PubMed 25135324](https://pubmed.ncbi.nlm.nih.gov/25135324/)). So go **slightly** off-white, not beige-dim.
- Ambient conditions matter at least as much as palette: low-intensity ambient light matched to screen intensity is recommended over fighting glare with UI colour. ([Stoney Creek Optometry](https://stoneycreekoptometry.com/is-dark-mode-better-for-your-eyes/))

---

## 9. Typeface family: stop optimising this variable

- Across studies, **serif vs sans-serif shows no consistent significant difference** in reading speed or comprehension when text is well laid out; **layout, lighting and contrast are more influential than typeface choice**. ([Legible Typography, "Overview of research: Type"](https://legible-typography.com/en/5-overview-of-research-type); [Quantitative analysis of serif and sans serif on reading speed in print and digital](https://bshuva.net/journals/quantitative-paper-fonts-0/))
- A broad review of typeface features and legibility reaches the same place — feature-level effects exist but are modest relative to size and contrast. ([*Vision Research*, "Typeface features and legibility research"](https://www.sciencedirect.com/science/article/pii/S0042698919301087))
- One low-vision nuance: **fixed-width fonts (Courier) outperform proportional fonts near the acuity limit**, though overall font effects remain modest. ([PMC5726769](https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/))

**Takeaway:** the Avenir Next / Charter pairing is not the problem. Mixing two families at 17–18px in dense rows is a *hierarchy* problem, not a legibility one.

---

## 10. Breaks and eye strain: the popular rule is not supported

- **Negative finding.** A recent study gives **little or no support** for 20-second breaks relieving digital eye strain; 20 seconds of distance viewing is likely too short for accommodation and vergence responses to dissipate. ([Review of Optometry, "20-20-20 Still Not Enough to Alleviate Eye Strain"](https://www.reviewofoptometry.com/news/article/202020-still-not-enough-to-alleviate-eye-strain); [Optometry Advisor](https://www.optometryadvisor.com/features/digital-eye-strain-may-not-be-solved-by-the-20-20-20-rule/))
- There is **no human clinical data** that the 20-20-20 rule combats myopia, and animal models suggest 20-second breaks from myopiagenic activity are ineffective. ([Myopia Profile, "Is the 20-20 rule effective advice in myopia management?"](https://www.myopiaprofile.com/articles/is-the-20-20-rule-effective-myopia))
- Longer or differently-structured breaks, with specified fixation targets, may be needed. ([Review of Optometry](https://www.reviewofoptometry.com/news/article/202020-still-not-enough-to-alleviate-eye-strain))

Do not ship a "take a break" nudge as an accessibility feature; it is not evidence-backed. Fix the page instead.

---

## 11. What this means for THIS page

Each recommendation traces to a section above.

### 11.1 Default theme → **light, and make it the default explicitly**
Positive polarity wins on proofreading and acuity for corrected-normal eyes, wins *more* as characters get smaller, and the win is not an artifact of ambient light (§1.1). The page is a dense small-type newspaper layout — the worst case for negative polarity. Keep dark mode available (§1.2: NN/g, and the arXiv null result), but ship light as the default and do not treat dark as the "premium" look.
**Do not** ship a pure-black dark theme: black on a glossy Mac/iPhone panel measurably increased look-aways and reduced task engagement (§8, PMC10506577), and pure-white-on-pure-black is the exact combination accessibility guidance flags for halation (§2.2).

### 11.2 Background tone → **near-white, not pure white; dark theme on charcoal, not black**
Light theme: something in the #FAFAF8–#F7F6F3 range. Off-white advice is weakly sourced (§8) but costs nothing; going much darker is actively counterproductive because display luminance is the mechanism behind the polarity advantage (§1.1). Do **not** go warm-beige "paper".
Dark theme: charcoal (~#16181C–#1E2126) with off-white text (~#E6E4E0), never #FFF on #000 (§2.2, §6 — APCA caps dark-mode contrast around Lc −90 rather than maximising it).

### 11.3 Text weights → **kill every weight below 400; stop reaching for bold**
Thin strokes are the one weight regime that reliably slowed readers (§3, 0.27× stroke width), and the heaviest weights also slowed them; everything from 0.72× to 1.89× performed the same. So: body and headline rows at **400–500**; use **600** only for the lead headline and section labels; **no 200/300 anywhere**, including "quiet" metadata (`8 histórias`, `peso 370`, `5 fontes`) and day chips — those are currently the most likely offenders. Prefer a **wider** cut over a heavier one where the family offers it (§3, Typotheque: width helped, weight did not).

### 11.4 Sizes → **raise the floor, compress the range**
- Body/serif bullets: **18–19px minimum** (currently 17–18px), line-height **1.5** (§4 WCAG 1.4.12; §5).
- Dense headline rows: **17px minimum at weight 400+** — these rows are where the small-character polarity penalty bites hardest (§1.1).
- **Metadata is the real failure**: counts, chips, "5 fontes" are almost certainly ~11–13px at a light weight and low-contrast grey. APCA's Lc 60 ladder says 16px needs weight 700, and 24px is the minimum at weight 400 (§6). Either raise metadata to **14–15px at weight 500 with Lc ≥ 60**, or delete it. Fewer counts is also the ADHD-correct answer.
- The ~40px lead headline is fine; the problem is not the top of the hierarchy, it is the bottom.
- Verify the layout survives **200% zoom** without truncation or overlap (§5, SC 1.4.4). At 200%, the 240px nav + 320px sidebar three-column grid will fight the feed — plan a collapse of both rails, not a horizontal scroll.

### 11.5 Contrast → **target APCA, not the 4.5:1 checkbox**
Body text at **Lc ≥ 75, ideally Lc ≥ 90** (§6). Every grey currently used for de-emphasis should be measured in APCA; WCAG 2 ratios will pass greys that are unreadable at 13px. In dark theme, do not simply invert — recompute, since WCAG 2 overstates contrast for dark colours (§6).

### 11.6 Accents → **colour is for identity, never for reading**
- Per-section colours (IA teal, Rust amber, Hacking red, Crypto blue) should live in **rules, dots, small caps labels and backgrounds** — not in headline or body text. Saturated blue as text is dim on white and glowing-thin on dark; note the *reason* is luminance, not chromatic aberration, which the eye largely compensates for (§7).
- Amber on near-white is the worst offender for luminance contrast — amber must be a **background/underline/marker**, never text.
- On the dark theme, **no glowing accents**: bright saturated strokes on dark are precisely the halation case (§2.2) and the reflection-magnet case on glossy screens (§8). Desaturate accents ~15–25% in dark mode and let them carry Lc, not neon.
- Quiet underlined action links: keep the underline (it is a non-colour cue) but bring the link colour to the same Lc band as body text (§6).

### 11.7 What NOT to do (things that look like fixes and are not)
- **Do not add letter-spacing to body text** to "help" — beyond normal separation it does not increase reading speed and can reduce it (§4).
- **Do not bold everything** — no weight above standard beat standard for impaired readers (§3).
- **Do not switch typeface families** hoping for a legibility gain — serif vs sans shows no consistent difference (§9).
- **Do not add a blue-light filter or a 20-20-20 nudge** as an accessibility feature — unsupported (§7, §10).
- **Do not max out dark-mode contrast** — APCA explicitly caps it (§6).

### 11.8 The single highest-leverage change
Reduce the number of simultaneously-legible elements. Every source in §5 says print size, characters per line and line spacing trade against each other in a fixed budget; a 3-column grid with a 240px nav, a 320px sidebar, per-section leads, dense rows, day chips and counts spends that budget on chrome. Raising type sizes **without** removing elements will just trigger the mobile failure mode (§5, PMC9357187) on desktop too.

---

## Sources

1. Piepenbrock C., Mayr S., Buchner A. — *Smaller pupil size and better proofreading performance with positive than with negative polarity displays* — https://pubmed.ncbi.nlm.nih.gov/25135324/
2. Piepenbrock C., Mayr S., Buchner A. — *Positive display polarity is advantageous for both younger and older adults* (PDF) — https://www.psychologie.hhu.de/fileadmin/redaktion/Oeffentliche_Medien/Fakultaeten/Mathematisch-Naturwissenschaftliche_Fakultaet/Psychologie/AAP/Publikationen/2013/Piepenbrock-2013-Positive_display_polarity_is_.pdf
3. Piepenbrock C., Mayr S., Buchner A. — *Positive Display Polarity Is Particularly Advantageous for Small Character Sizes*, Human Factors — https://journals.sagepub.com/doi/abs/10.1177/0018720813515509
4. Buchner A., Baumgartner N. — *Text–background polarity affects performance irrespective of ambient illumination and colour contrast* — https://www.researchgate.net/publication/6321309_Text_-_Background_polarity_affects_performance_irrespective_of_ambient_illumination_and_colour_contrast
5. Nielsen Norman Group — *Dark Mode vs. Light Mode: Which Is Better?* — https://www.nngroup.com/articles/dark-mode/
6. *Dark Mode or Light Mode? Exploring the Impact of Contrast Polarity on Visualization Performance Between Age Groups*, arXiv 2409.10841 — https://arxiv.org/html/2409.10841v2
7. Zhang J. et al. — *Epidemiology and Burden of Astigmatism: A Systematic Literature Review*, Optometry and Vision Science 2023 — https://doi.org/10.1097/OPX.0000000000001998 (full text PDF: https://www.researchgate.net/publication/368334523_Epidemiology_and_Burden_of_Astigmatism_A_Systematic_Literature_Review/fulltext/645bdcbafbaf5b27a4ba1068/Epidemiology-and-Burden-of-Astigmatism-A-Systematic-Literature-Review.pdf)
8. Pacific Eye Surgeons — *What Percentage of People Have Astigmatism?* — https://www.paceyemd.com/blog/what-percentage-of-people-have-astigmatism-understanding-its-impact/
9. Level Access — *Astigmatism and Web Accessibility: Design Guide* — https://www.levelaccess.com/blog/accessibility-for-people-with-astigmatism/
10. H Locke — *Why dark mode causes more accessibility issues than it solves* — https://medium.com/@h_locke/why-dark-mode-causes-more-accessibility-issues-than-it-solves-54cddf6466f5
11. Luke Harris — *Astigmatism and dark mode* — https://www.lkhrs.com/blog/astigmatism-and-dark-mode/
12. techealthinfo — *Dark Mode and Astigmatism: 5 Tweaks to Reduce Eye Strain* — https://techealthinfo.com/dark-mode-and-astigmatism-5-tweaks-to-reduce-eye-strain/
13. Xiong Y.-Z. et al. — *Bolder print does not increase reading speed in people with central vision loss* — https://pmc.ncbi.nlm.nih.gov/articles/PMC6287928/
14. *The Effect of Letter-stroke Boldness on Reading Speed in Central and Peripheral Vision* — https://pmc.ncbi.nlm.nih.gov/articles/PMC3642228/
15. *High letter stroke contrast impairs letter recognition of bold fonts*, Applied Ergonomics — https://www.sciencedirect.com/science/article/pii/S0003687021001460
16. Typotheque — *Designing fonts with low-vision readers in mind: A reading acuity experiment* — https://www.typotheque.com/research/designing-fonts-with-low-vision-readers-in-mind
17. Chung S.T.L. — *The effect of letter spacing on reading speed in central and peripheral vision* — https://pubmed.ncbi.nlm.nih.gov/11923275/ and https://iovs.arvojournals.org/article.aspx?articleid=2200181
18. *Dependence of Reading Speed on Letter Spacing in Central Vision Loss* — https://pmc.ncbi.nlm.nih.gov/articles/PMC3429790/
19. *Small effect of interline spacing on maximal reading speed in low-vision patients with central field loss* — https://pubmed.ncbi.nlm.nih.gov/19834038/
20. W3C — *Understanding SC 1.4.12: Text Spacing* — https://www.w3.org/WAI/WCAG22/Understanding/text-spacing.html
21. W3C — *Understanding SC 1.4.4: Resize Text* — https://www.w3.org/WAI/WCAG22/Understanding/resize-text.html
22. Legge G.E., Chung S.T.L. — *Reading Digital with Low Vision* — https://pmc.ncbi.nlm.nih.gov/articles/PMC5726769/
23. *Technical Report: Digital Reading with Low Vision: Principles for Selecting Display Size* — https://ncbi.nlm.nih.gov/pmc/articles/PMC9357187 (abstract: https://pubmed.ncbi.nlm.nih.gov/35731508/)
24. Xiong Y.-Z., Legge G.E. et al. — *Reading Acuity as a Predictor of Low-Vision Reading Performance* — https://pmc.ncbi.nlm.nih.gov/articles/PMC6181187/
25. Optometry Times — *Approach patients with low vision on an individual basis* (acuity reserve / critical print size) — https://www.optometrytimes.com/view/approach-patients-with-low-vision-on-an-individual-basis
26. APCA — *The Easy Intro to the APCA Contrast Method* — https://git.apcacontrast.com/documentation/APCAeasyIntro.html
27. APCA — *Why APCA as a New Contrast Method?* — https://git.apcacontrast.com/documentation/WhyAPCA
28. Readtech — *APCA Readability Criterion • Contrast* — https://www.readtech.org/ARC/tests/visual-readability-contrast/
29. *Vision is protected against blue defocus* — https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7801416/
30. UXmatters — *Applying Color Theory to Digital Displays* — https://www.uxmatters.com/mt/archives/2007/01/applying-color-theory-to-digital-displays.php
31. kidspattern — *Why is blue text harder to read?* — https://kidspattern.com/learn/why-is-blue-text-harder-to-read/
32. *Short-term effects of text-background color combinations on the dynamics of the accommodative response*, Vision Research — https://www.sciencedirect.com/science/article/pii/S0042698919302123
33. *Examination of distraction and discomfort caused by using glare monitors: a simultaneous EEG and eye-tracking study* — https://pmc.ncbi.nlm.nih.gov/articles/PMC10506577/
34. Design for Ducks — *Color's effect on readability and vision fatigue* — https://designforducks.com/colors-effect-on-readability-and-vision-fatigue/
35. Stoney Creek Optometry — *Is Dark Mode Better for Your Eyes?* — https://stoneycreekoptometry.com/is-dark-mode-better-for-your-eyes/
36. W3C WAI-IG mailing list — glare on white backgrounds and perceived character sharpness — https://lists.w3.org/Archives/Public/w3c-wai-ig/2003JanMar/0883.html
37. Legible Typography — *Overview of research: Type* (serif vs sans-serif) — https://legible-typography.com/en/5-overview-of-research-type
38. *Quantitative Analysis of typefaces serif and sans serif on reading speed in Print and Digital medium* — https://bshuva.net/journals/quantitative-paper-fonts-0/
39. *Typeface features and legibility research*, Vision Research — https://www.sciencedirect.com/science/article/pii/S0042698919301087
40. Review of Optometry — *20-20-20 Still Not Enough to Alleviate Eye Strain* — https://www.reviewofoptometry.com/news/article/202020-still-not-enough-to-alleviate-eye-strain
41. Optometry Advisor — *Digital Eye Strain: Another Look at the 20-20-20 Rule* — https://www.optometryadvisor.com/features/digital-eye-strain-may-not-be-solved-by-the-20-20-20-rule/
42. Myopia Profile — *Is the 20-20 rule effective advice in myopia management?* — https://www.myopiaprofile.com/articles/is-the-20-20-rule-effective-myopia
