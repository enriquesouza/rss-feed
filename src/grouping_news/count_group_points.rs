use crate::app_data::rss_news::news_item::NewsItem;
use crate::app_data::settings::group_rule::GroupRule;
use crate::formatting_text::check_if_word_is_there::check_if_word_is_there;

const TITLE_POINTS: i32 = 10;
const SOURCE_POINTS: i32 = 8;
const BODY_POINTS: i32 = 3;
// Kept at 3 so two weak words (6) stay below MIN_POINTS_TO_JOIN (8) and cannot
// decide alone; three weak words in one title (9) still can, which is rare and fair.
const WEAK_TITLE_POINTS: i32 = 3;

// Same news item, ready to search: everything lowercase, done once per item.
pub struct LowerNewsText {
    pub title: String,
    pub source: String,
    pub body: String,
    pub source_and_link: String,
}

pub fn make_lower_news_text(item: &NewsItem) -> LowerNewsText {
    let source = item.source.to_lowercase();
    let link = item.link.to_lowercase();

    // links are matched only against the sources list, never against keywords —
    // URL slugs caused false hits (…/r/LocalLLaMA/… contains "llama").

    LowerNewsText {
        title: item.title.to_lowercase(),
        body: item.clean_description.to_lowercase(),
        source_and_link: format!("{source} {link}"),
        source,
    }
}

// How well this news item fits one topic group. More points = better fit.
pub fn count_group_points(text: &LowerNewsText, group: &GroupRule) -> i32 {
    let mut points = 0;

    for word in &group.keywords {
        if check_if_word_is_there(&text.title, word) {
            points += TITLE_POINTS;
        }
        if check_if_word_is_there(&text.source, word) {
            points += SOURCE_POINTS;
        }
        if check_if_word_is_there(&text.body, word) {
            points += BODY_POINTS;
        }
    }

    // Broad words only count when they are in the title.
    for word in &group.weak_keywords {
        if check_if_word_is_there(&text.title, word) {
            points += WEAK_TITLE_POINTS;
        }
    }

    // A feed that always talks about this topic.
    for feed in &group.sources {
        if text.source_and_link.contains(feed) {
            points += SOURCE_POINTS;
        }
    }

    points
}
