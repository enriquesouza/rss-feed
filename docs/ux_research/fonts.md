# Fonts — evidence-ranked choice for the news portal

> Produced 2026-08-04 by a 4-agent deep research run (legibility science, reading-product
> survey, zero-network availability audit, synthesis judge). Rule: no claim without a URL.
> Applied to `src/bin/admin.rs` — see the Recommendation section for the exact values.

# TYPOGRAPHY DECISION — Synthesis of Reports A/B/C

## 1. RANKED TABLE — Top 10 body/summary fonts for this app

| # | Font | Serif/Sans | Evidence for readability | Availability (zero-network) | PT-BR accents | Risk / caveat |
|---|------|-----------|--------------------------|------------------------------|---------------|----------------|
| 1 | **Charter** (Matthew Carter, 1987) | Serif | Medium's article body since 2015, chosen for low-res sturdiness, open forms, big x-height ([medium.design](https://medium.design/cast-of-characters-17eaa82755cf), [fontsinuse.com](https://fontsinuse.com/uses/12025/medium-com-2015)); on Pocket's designer shortlist ([medium.com/pocket-design](https://medium.com/pocket-design/reading-types-deserve-the-best-type-for-reading-c348753b070b)) | **Both**: preinstalled "system font" on iOS AND macOS ([developer.apple.com/fonts/system-fonts](https://developer.apple.com/fonts/system-fonts/), [support.apple.com/122869](https://support.apple.com/en-us/122869)) AND vendorable ~44 KB woff2, Bitstream free license ([practicaltypography.com/charter.html](https://practicaltypography.com/charter.html)) | Verified by cmap inspection (Report C) | Only reading serif with this dual path; no direct peer-reviewed speed study of Charter itself |
| 2 | **Literata** (TypeTogether) | Serif | Google Play Books default since 2015, commissioned for "outstanding reading across devices" ([type-together.com/literata-book](https://www.type-together.com/literata-book), [fastcompany.com](https://www.fastcompany.com/3046511/how-google-made-an-e-book-font-designed-for-any-screen)); added by Instapaper 10 in 2026 ([blog.instapaper.com](https://blog.instapaper.com/2026/07/28/instapaper-10/)) | Vendor only: OFL, VF ~59 KB (upright 400–700 in one file + italic; [github.com/googlefonts/literata](https://github.com/googlefonts/literata)) | Verified by cmap inspection (Report C) | Never renders as a native system font; always costs bytes |
| 3 | **Georgia** (Matthew Carter) | Serif | Lineage/fallback of NYT and Guardian body stacks ([github.com/guardian/guss-typography](https://github.com/guardian/guss-typography/blob/master/README.md), [fontsinuse.com/typefaces/112761](https://fontsinuse.com/typefaces/112761/nyt-imperial)); WaPo's historic web body; on Instapaper's menu; repeatedly cited screen-optimized for x-height/open forms ([legible-typography.com](https://legible-typography.com/en/5-overview-of-research-type)) | System font iOS + macOS + Windows ([developer.apple.com/fonts/system-fonts](https://developer.apple.com/fonts/system-fonts/)) — 0 KB everywhere that matters | Not explicitly verified in reports (Apple/Microsoft system Latin font) | Safe, not distinctive; the news giants that used it moved to custom faces built on the same formula |
| 4 | **New York** (Apple, 2019) | Serif | Apple Books/Apple News reading face, variable optical sizes ([en.wikipedia.org/wiki/New_York_(2019_typeface)](https://en.wikipedia.org/wiki/New_York_(2019_typeface))); added by Instapaper 10 ([blog.instapaper.com](https://blog.instapaper.com/2026/07/28/instapaper-10/)) | System, but reachable only via `ui-serif` — **Safari/iOS only; Chrome does not resolve it** ([caniuse.com/extended-system-fonts](https://caniuse.com/extended-system-fonts)); **cannot be vendored** (Apple license, [developer.apple.com/fonts](https://developer.apple.com/fonts/)) | Not explicitly verified in reports (Apple system font) | User reads in Chrome on macOS → primary browser gets the fallback, not the font |
| 5 | **Atkinson Hyperlegible** | Sans | Added by BOTH Instapaper 10 and Readwise Reader as accessibility reading option ([blog.instapaper.com](https://blog.instapaper.com/2026/07/28/instapaper-10/), [docs.readwise.io](https://docs.readwise.io/reader/docs/faqs/appearance)); design implements the Il1 letter-differentiation principle from Report A ([dotcentric.co.uk](https://www.dotcentric.co.uk/thinking/accessible-typeface-guide/), [brailleinstitute.org/freefont](https://www.brailleinstitute.org/freefont/)) | Vendor: OFL, ~34 KB static (smallest option; [github.com/googlefonts/atkinson-hyperlegible](https://github.com/googlefonts/atkinson-hyperlegible)) | Verified by cmap inspection (Report C) | **No peer-reviewed efficacy study exists** — evidence is design-process testing + Braille Institute claims (Report C, honest-status note) |
| 6 | **Inter** | Sans | The UI-sans consensus: Linear's UI face; designed by Rasmus Andersson specifically for small-size screen UI, tall x-height ([en.wikipedia.org/wiki/Inter_(typeface)](https://en.wikipedia.org/wiki/Inter_(typeface)), [typ.io/s/2jmp](https://typ.io/s/2jmp)) | Vendor: OFL, VF ~72 KB — largest of the measured set ([github.com/rsms/inter](https://github.com/rsms/inter)) | Verified by cmap inspection (Report C) | UI evidence, not long-form reading evidence; heaviest file; duplicates what SF already gives on Apple devices |
| 7 | **Garamond** | Serif | **Strongest lab result**: fastest average in Wallace et al. 2022 (312 WPM, 16 fonts, 352 participants) and one of 3 fonts favoring readers 35+ ([nngroup.com](https://www.nngroup.com/articles/best-font-for-online-reading/), [PDF mirror](https://thereadabilityconsortium.org/wp-content/uploads/2023/07/Readability__TOCHI-1.pdf)) | **No zero-network path verified in any report**: absent from Apple's system lists (Report C) and no OFL build was measured | Not verified (no file inspected) | Average-only win — same study shows 35% individual variance and NN/g's verdict is "no single answer"; fails ranking criterion (b) outright |
| 8 | **Verdana** | Sans | Beat Times New Roman at threshold-legibility sizes, "primarily due to its larger x-height" — the single most predictive legibility feature ([legible-typography.com](https://legible-typography.com/en/5-overview-of-research-type), [fontfabric.com](https://www.fontfabric.com/blog/typography-knowledge-legible-fonts/)) | System font iOS + macOS + Windows ([developer.apple.com/fonts/system-fonts](https://developer.apple.com/fonts/system-fonts/)) — 0 KB | Not explicitly verified in reports | Evidence is small-size/threshold legibility; no reading product in Report B uses it as body today |
| 9 | **SF Pro via `system-ui`** (incumbent) | Sans | GitHub Primer and Wikipedia deliberately ship the raw system stack ([github.com/primer/css](https://github.com/primer/css/blob/main/src/support/variables/typography.scss), [mediawiki.org](https://www.mediawiki.org/wiki/Typography_refresh)); Report B's verdict: "what you already have for UI is defensible" | 0 KB, `-apple-system`/`system-ui`; cannot be vendored (Apple license, Report C); non-Apple devices get their own system font | Not explicitly verified in reports (system font) | Per Report A the science-backed culprit is the **15px size**, not SF itself — keep the face, fix the size |
| 10 | **Palatino** | Serif | On Instapaper's expert-curated system-font menu (with Georgia/Hoefler Text/Baskerville) ([marco.org](https://marco.org/2012/03/16/instapaper-4-1-released)) | System font iOS + macOS ([developer.apple.com/fonts/system-fonts](https://developer.apple.com/fonts/system-fonts/)) — 0 KB | Not explicitly verified in reports | No study evidence; it's already his fallback — a floor, not a destination |

**Dropped from ranking, with reason**: **Iowan Old Style** (current dek font) — officially a legacy **"document-support" font** on macOS 15, macOS 26 and iOS ([support.apple.com/122869](https://support.apple.com/en-us/122869), [support.apple.com/120414](https://support.apple.com/en-us/120414), [developer.apple.com/fonts/system-fonts](https://developer.apple.com/fonts/system-fonts/)), and no reading product in Report B uses it. It resolves today only because CSS requests by name.

## 2. HEADLINES mini-ranking (top 3)

| # | Font | Serif/Sans | Evidence | Availability | PT-BR | Risk / caveat |
|---|------|-----------|----------|--------------|-------|----------------|
| 1 | **Avenir Next** (Demi/Bold, non-condensed) | Sans | Keeps the current family voice while restoring the open apertures/counters that Report A's feature research identifies as legibility drivers ([legible-typography.com](https://legible-typography.com/en/5-overview-of-research-type), [fontfabric.com](https://www.fontfabric.com/blog/typography-knowledge-legible-fonts/)) | System font iOS + macOS, all weights ([support.apple.com/122869](https://support.apple.com/en-us/122869)) — 0 KB | Not explicitly verified in reports (Apple system font) | No headline-specific study exists for it (none was found for any font) |
| 2 | **Literata 700** (same vendored VF) | Serif | Unifies headline + dek in one purpose-built reading family; Report C verified the single upright VF file covers 400–700 (Google serves the identical file for both weights) ([github.com/googlefonts/literata](https://github.com/googlefonts/literata), [type-together.com](https://www.type-together.com/literata-book)) | **0 extra KB** if Literata is already vendored for deks | Verified (Report C cmap) | Serif headlines are a bigger visual change; evidence is body-reading, not display |
| 3 | **Avenir Next Condensed 700** (status quo) | Sans | Partial defense exists: Oswald — a condensed sans — was #2 fastest in Wallace et al., only 6% behind Garamond ([nngroup.com](https://www.nngroup.com/articles/best-font-for-online-reading/)), so condensed is not inherently slow | System font iOS + macOS incl. Condensed Bold ([support.apple.com/122869](https://support.apple.com/en-us/122869)) — 0 KB | Not explicitly verified in reports | Report A flags condensed faces as conflicting with the open-aperture/large-counter findings (explicitly an inference, no direct study); keep only at large display sizes, never for labels or deks |

**Verdict on Avenir Next Condensed**: defensible for large headlines only (Oswald precedent), but #1 is the safer evidence-aligned move at zero cost — same family, so the redesign is invisible except where it helps.

## 3. THE RECOMMENDATION

```css
:root {
  /* UI + body — keep system stack (GitHub Primer pattern), fix the SIZE not the face */
  --body: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans",
          Helvetica, Arial, sans-serif;

  /* Deks / summaries / digests — Charter: native on Apple, vendored elsewhere */
  --serif: Charter, "Bitstream Charter", Georgia, "Times New Roman", serif;

  /* Headlines — non-condensed Avenir Next; graceful degradation off-Apple */
  --display: "Avenir Next", Avenir, -apple-system, "Segoe UI",
             "Helvetica Neue", Arial, sans-serif;  /* weight 600–700 */
}

/* Non-Apple guarantee for --serif: local() makes Apple devices use the
   preinstalled Charter (0 bytes); everyone else downloads from your binary */
@font-face { font-family: Charter; font-style: normal; font-weight: 400;
  src: local("Charter"), url(/fonts/charter-regular.woff2) format("woff2"); }
@font-face { font-family: Charter; font-style: normal; font-weight: 700;
  src: local("Charter Bold"), url(/fonts/charter-bold.woff2) format("woff2"); }
@font-face { font-family: Charter; font-style: italic; font-weight: 400;
  src: local("Charter Italic"), url(/fonts/charter-italic.woff2) format("woff2"); }
```

**Evidence-backed numbers**:
- **Body/UI font-size: 16px** (up from 15px). 16px is the accessible baseline ([a11y-collective.com](https://www.a11y-collective.com/blog/wcag-minimum-font-size/)); Butterick's range is 15–25px with 15 at the floor ([practicaltypography.com](https://practicaltypography.com/summary-of-key-rules.html)). Report A: the 15px UI text is "the most likely science-backed culprit, not the serif choice."
- **Deks/summaries: 18px** (up from 16.5px). Long-form guidance is 18–20px ([learnui.design](https://www.learnui.design/blog/mobile-desktop-website-font-size-guidelines.html)); Readwise Reader defaults to 20px ([docs.readwise.io](https://docs.readwise.io/reader/docs/faqs/appearance)).
- **Line-height: 1.5** for both (down from 1.62 on deks). WCAG 1.4.8 minimum is 1.5 ([w3.org](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html)); Butterick's ceiling is 145% ([practicaltypography.com](https://practicaltypography.com/summary-of-key-rules.html)) — 1.5 is the intersection; current 1.62 exceeds both.
- **Max measure: `max-width: 65ch`** on the summary column. Sweet spot 50–75 cpl ([medium.com/@wblekhoa](https://medium.com/@wblekhoa/talk-aboutthe-optimal-length-of-text-in-ux-ui-525e689f0b71)), WCAG hard max 80, Butterick 45–90 ([practicaltypography.com](https://practicaltypography.com/summary-of-key-rules.html), [w3.org](https://www.w3.org/WAI/WCAG21/Understanding/visual-presentation.html)).
- No full justification on dek paragraphs (WCAG 1.4.8, same source). Side note for the dark theme: glance-legibility research favors dark-text-on-light ([Dobres et al. 2016](https://www.tandfonline.com/doi/full/10.1080/00140139.2015.1137637), [nngroup.com/articles/dark-mode](https://www.nngroup.com/articles/dark-mode/)) — keep light mode the default for long reading sessions.

**WHY Charter beats #2 (Literata)**: identical product pedigree (Medium's body face vs Google Play Books'), but Charter is the only reading serif that is simultaneously a preinstalled system font on both of his target platforms (0 KB, native rendering) AND freely vendorable for every other device — Literata always costs 59 KB and is never native (Reports B + C).

## 4. Vendoring manifest

- **Files**: `charter-regular.woff2` (14.6 KB) + `charter-bold.woff2` (15.0 KB) + `charter-italic.woff2` (15.4 KB) — optional `charter-bold-italic.woff2` (16.1 KB). **Total: ~45 KB (61 KB with bold-italic), full charset, no subsetting needed.**
- **Source**: Butterick's Practical Typography distribution, zip `Charter 210112.zip` — https://practicaltypography.com/charter.html
- **License**: Bitstream free license — bundled `license.txt` grants "permission … to use, copy, modify, sublicense, sell, and redistribute" with the notice kept intact (Report C). Ship the notice file alongside the fonts.
- **PT-BR coverage** (ã õ ç é ê á à í ó ô ú ü â): verified present via fontTools cmap inspection (Report C).
- Files already downloaded at `/private/tmp/claude-501/-Users-enriquesouza/97cf29e5-4eae-4187-8fae-0d8e2250bc67/scratchpad/fonts/` and `/private/tmp/claude-501/-Users-enriquesouza/97cf29e5-4eae-4187-8fae-0d8e2250bc67/scratchpad/charterdist/` — note these are session-scratch paths; copy into the app's vendored-assets directory before the session ends.
- **Optional upgrade path** (deferred, not required): Literata VF (~59 KB, OFL, [github.com/googlefonts/literata](https://github.com/googlefonts/literata)) if he later wants headline/dek unification under one reading family.

**Unverifiable-claims carried over**: PT-BR accent coverage of the Apple system fonts (Georgia, Palatino, Verdana, Avenir Next, New York, SF) was not cmap-verified by any report; Wallace et al. full text was confirmed only via mirrors ([thereadabilityconsortium.org PDF](https://thereadabilityconsortium.org/wp-content/uploads/2023/07/Readability__TOCHI-1.pdf), [nngroup.com](https://www.nngroup.com/articles/best-font-for-online-reading/)); no peer-reviewed study exists for Iowan Old Style, Avenir Next Condensed, SF Pro, Charter, or Atkinson Hyperlegible specifically (Reports A + C).