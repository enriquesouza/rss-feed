//! Read public numbers for one X post — no paid API, no login.
//!
//! X's syndication endpoint (the one the embed widget uses) answers with JSON
//! for any public post: likes, replies, when it was published, and the text.
//! Views, reposts and bookmarks are NOT in that payload — X only shows those
//! to a logged-in session — so those stay manual.

use serde::Deserialize;

const SYNDICATION_URL: &str = "https://cdn.syndication.twimg.com/tweet-result";
const BROWSER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0 Safari/537.36";

/// What the public endpoint can tell us about a post.
#[derive(Debug, Clone)]
pub struct PostStats {
    pub likes: i64,
    pub replies: i64,
    pub created_at: Option<String>,
    pub text: String,
    /// Who really owns the post X answered with.
    pub screen_name: Option<String>,
}

#[derive(Deserialize)]
struct SyndicationUser {
    #[serde(default)]
    screen_name: Option<String>,
}

#[derive(Deserialize)]
struct SyndicationAnswer {
    #[serde(default)]
    favorite_count: i64,
    #[serde(default)]
    conversation_count: i64,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    text: String,
    #[serde(default)]
    user: Option<SyndicationUser>,
}

/// Pull the numeric id out of an X/Twitter post link.
pub fn parse_post_id(post_url: &str) -> Option<String> {
    let after_status = post_url.split("/status/").nth(1)?;
    let id: String = after_status
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if id.is_empty() { None } else { Some(id) }
}

/// Ask X for the public numbers of one post.
///
/// Fails closed: a missing/private/deleted post is an error, never zeroes that
/// would look like a real measurement.
pub async fn fetch_post_stats(
    http_client: &reqwest::Client,
    post_url: &str,
) -> anyhow::Result<PostStats> {
    let post_id = parse_post_id(post_url)
        .ok_or_else(|| anyhow::anyhow!("link sem id de post: {post_url}"))?;

    let ask_url = format!("{SYNDICATION_URL}?id={post_id}&lang=en&token=a");
    let answer = http_client
        .get(&ask_url)
        .header("user-agent", BROWSER_AGENT)
        .timeout(std::time::Duration::from_secs(20))
        .send()
        .await?;

    if !answer.status().is_success() {
        anyhow::bail!("X respondeu {} para o post {post_id}", answer.status());
    }

    let body: SyndicationAnswer = answer.json().await?;

    let screen_name = body.user.and_then(|u| u.screen_name);

    // A typo in the link would otherwise import a stranger's numbers as ours.
    if let (Some(from_url), Some(from_x)) = (handle_from_url(post_url), screen_name.as_deref())
        && !from_url.eq_ignore_ascii_case(from_x)
    {
        anyhow::bail!("esse link é de um post de @{from_x}, não de @{from_url}");
    }

    Ok(PostStats {
        likes: body.favorite_count,
        replies: body.conversation_count,
        created_at: body.created_at,
        text: body.text,
        screen_name,
    })
}

/// The handle in an X post link: https://x.com/<handle>/status/<id>
pub fn handle_from_url(post_url: &str) -> Option<String> {
    let after_host = post_url
        .split("x.com/")
        .nth(1)
        .or_else(|| post_url.split("twitter.com/").nth(1))?;
    let handle = after_host.split('/').next()?.trim();
    if handle.is_empty() {
        None
    } else {
        Some(handle.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::parse_post_id;

    #[test]
    fn reads_handle_from_links() {
        use super::handle_from_url;
        assert_eq!(
            handle_from_url("https://x.com/enriquesouza_/status/123").as_deref(),
            Some("enriquesouza_")
        );
        assert_eq!(
            handle_from_url("https://twitter.com/AizzyAi/status/9").as_deref(),
            Some("AizzyAi")
        );
        assert!(handle_from_url("https://example.com/a/status/1").is_none());
    }

    #[test]
    fn reads_id_from_links() {
        assert_eq!(
            parse_post_id("https://x.com/enriquesouza_/status/2084443094356004914").as_deref(),
            Some("2084443094356004914")
        );
        assert_eq!(
            parse_post_id("https://twitter.com/a/status/123?s=20").as_deref(),
            Some("123")
        );
        assert!(parse_post_id("https://x.com/enriquesouza_").is_none());
        assert!(parse_post_id("https://x.com/a/status/abc").is_none());
    }
}
