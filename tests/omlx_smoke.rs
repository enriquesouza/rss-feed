// Live smoke test against the local oMLX server (needs it running + .env).
// Ignored by default. Run with:
//   cargo test --test omlx_smoke -- --ignored --nocapture

#[tokio::test]
#[ignore]
async fn writes_english_posts_with_bonsai() {
    dotenvy::dotenv().ok();
    let client = reqwest::Client::new();
    let writer = rss_feed::writing_x_posts::write_x_posts_with_ai::XPostsWriter::new(&client);

    let story = "Source: Hugging Face blog\n\
        Title: Qwen team releases Qwen3.6-VL 32B open weights\n\
        Summary: The new vision-language model matches closed frontier models on \
        document understanding benchmarks while running on a single consumer GPU. \
        Weights are on Hugging Face under Apache 2.0. Link: https://huggingface.co/blog/qwen36-vl";

    let out = writer
        .write_x_posts(story.to_string())
        .await
        .expect("omlx call failed")
        .expect("empty answer");
    println!("=== RAW ===\n{out}\n");

    let posts = rss_feed::storing_posts::split_posts_text::split_posts_text(&out);
    println!("=== PARSED: {} posts ===", posts.len());
    for p in &posts {
        println!("- [{}] {}", p.topic, p.text_en);
    }
    assert!(!posts.is_empty(), "no posts parsed from writer output");
}
