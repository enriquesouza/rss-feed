//! Admin page for the X post drafts — the publishing desk.
//!
//! Run with: `cargo run --bin admin` → http://localhost:8787
//!
//! Life cycle managed here: gerado → aprovado → postado (paste the X link) →
//! metrics snapshots over time (filled by hand — no paid X API). HTMX keeps
//! the front end in one file; every action swaps just the post card.
//! Installable as an app (PWA manifest + icons served from the binary).

use axum::Router;
use axum::extract::{Form, Path, Query, State};
use axum::response::Html;
use axum::routing::{get, post};
use rss_feed::fetching_x_metrics::fetch_post_stats::fetch_post_stats;
use rss_feed::storing_posts::save_and_update_posts::{
    MetricsRow, PostRow, PostsDb, StoryWithItems,
};
use rss_feed::storing_posts::split_posts_text::split_posts_text;
use rss_feed::writing_x_posts::write_x_posts_with_ai::XPostsWriter;
use serde::Deserialize;
use std::sync::Arc;

/// What every handler gets: the database plus one shared HTTP client.
#[derive(Clone)]
struct AppState {
    db: Arc<PostsDb>,
    http: reqwest::Client,
}

const ADMIN_PORT: u16 = 8787;
const POST_CHAR_BUDGET: usize = 260;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The post writer reads OMLX_* from the environment. Load the crate's own
    // .env so the admin works no matter which folder it was started from.
    let env_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenvy::from_path(&env_file)
        .or_else(|_| dotenvy::dotenv().map(|_| ()))
        .ok();

    let state = AppState {
        db: Arc::new(PostsDb::open_posts_db()?),
        http: reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(std::time::Duration::from_secs(25))
            .build()?,
    };

    let app = Router::new()
        .route("/", get(show_home))
        .route("/htmx.min.js", get(show_htmx_js))
        .route("/fonts/charter-regular.woff2", get(show_charter_regular))
        .route("/fonts/charter-bold.woff2", get(show_charter_bold))
        .route("/fonts/charter-italic.woff2", get(show_charter_italic))
        .route("/manifest.json", get(show_manifest))
        .route("/icon.svg", get(show_icon_svg))
        .route("/icon-512.png", get(show_icon_512))
        .route("/icon-180.png", get(show_icon_180))
        .route("/posts", get(show_posts_part))
        .route("/analytics", get(show_analytics_part))
        .route("/posts/{id}/aprovar", post(approve_post))
        .route("/posts/{id}/descartar", post(discard_post))
        .route("/posts/{id}/voltar", post(reopen_post))
        .route("/posts/{id}/editar", post(edit_post))
        .route("/posts/{id}/postado", post(mark_posted))
        .route("/posts/{id}/metricas", post(add_metrics))
        .route("/posts/{id}/atualizar-metricas", post(refresh_metrics))
        .route("/posts/{id}/verificar", post(check_published))
        .route("/noticias", get(show_news_part))
        .route("/noticias/{id}/sugerir-post", post(suggest_post_from_story))
        .with_state(state);

    let addr = format!("127.0.0.1:{ADMIN_PORT}");
    println!("Admin on http://localhost:{ADMIN_PORT}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ---------- static assets (served from the binary — works offline) ----------

async fn show_htmx_js() -> ([(&'static str, &'static str); 1], &'static str) {
    (
        [("content-type", "application/javascript")],
        include_str!("admin_assets/htmx.min.js"),
    )
}

/// Charter woff2, vendored like htmx: Apple devices use the preinstalled
/// Charter via local(); every other device downloads these from the binary.
async fn show_charter_regular() -> ([(&'static str, &'static str); 1], &'static [u8]) {
    (
        [("content-type", "font/woff2")],
        include_bytes!("admin_assets/fonts/charter_regular.woff2"),
    )
}

async fn show_charter_bold() -> ([(&'static str, &'static str); 1], &'static [u8]) {
    (
        [("content-type", "font/woff2")],
        include_bytes!("admin_assets/fonts/charter_bold.woff2"),
    )
}

async fn show_charter_italic() -> ([(&'static str, &'static str); 1], &'static [u8]) {
    (
        [("content-type", "font/woff2")],
        include_bytes!("admin_assets/fonts/charter_italic.woff2"),
    )
}

async fn show_manifest() -> ([(&'static str, &'static str); 1], &'static str) {
    (
        [("content-type", "application/manifest+json")],
        r##"{
  "name": "Posts Admin",
  "short_name": "Posts",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#070b0f",
  "theme_color": "#070b0f",
  "icons": [
    { "src": "/icon-512.png", "sizes": "512x512", "type": "image/png" },
    { "src": "/icon.svg", "sizes": "any", "type": "image/svg+xml" }
  ]
}"##,
    )
}

async fn show_icon_svg() -> ([(&'static str, &'static str); 1], &'static str) {
    (
        [("content-type", "image/svg+xml")],
        include_str!("admin_assets/icon.svg"),
    )
}

async fn show_icon_512() -> ([(&'static str, &'static str); 1], &'static [u8]) {
    (
        [("content-type", "image/png")],
        include_bytes!("admin_assets/icon-512.png"),
    )
}

async fn show_icon_180() -> ([(&'static str, &'static str); 1], &'static [u8]) {
    (
        [("content-type", "image/png")],
        include_bytes!("admin_assets/icon-180.png"),
    )
}

// ---------- pages ----------

async fn show_home(State(state): State<AppState>) -> Html<String> {
    let db = &state.db;
    let counts = db.count_posts_by_status().unwrap_or_default();
    let count_of = |status: &str| {
        counts
            .iter()
            .find(|(s, _)| s == status)
            .map(|(_, c)| *c)
            .unwrap_or(0)
    };

    let today = chrono::Local::now().format("%d %b %Y").to_string();

    // Sidebar snapshot: what the posts already in the air are doing.
    let posted_rows = db.list_posts(Some("postado")).unwrap_or_default();
    let mut views_total = 0i64;
    let mut eng_total = 0i64;
    for row in &posted_rows {
        if let Some(m) = db.list_post_metrics(row.id).unwrap_or_default().last() {
            views_total += m.views;
            eng_total += m.likes + m.reposts + m.replies + m.bookmarks;
        }
    }
    let er = if views_total > 0 {
        format!("{:.1}%", eng_total as f64 * 100.0 / views_total as f64)
    } else {
        "—".to_string()
    };

    let page = format!(
        r##"<!doctype html>
<html lang="pt-BR">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<title>Posts — mesa de publicação</title>
<link rel="manifest" href="/manifest.json">
<link rel="icon" href="/icon.svg" type="image/svg+xml">
<link rel="apple-touch-icon" href="/icon-180.png">
<meta name="theme-color" content="#070b0f">
<script>
  // Applied before first paint so the page never flashes the wrong theme.
  (function () {{
    const salvo = localStorage.getItem('tema');
    const claro = salvo ? salvo === 'claro'
      : window.matchMedia('(prefers-color-scheme: light)').matches;
    if (claro) document.documentElement.setAttribute('data-theme', 'light');
  }})();
</script>
<script src="/htmx.min.js"></script>
<style>{css}</style>
</head>
<body>
<aside class="rail">
  <div class="mark" aria-hidden="true">
    <svg viewBox="0 0 512 512" width="44" height="44">
      <defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1">
        <stop offset="0" stop-color="#58e7d6"/><stop offset="1" stop-color="#2db2a3"/>
      </linearGradient></defs>
      <circle cx="256" cy="238" r="118" fill="none" stroke="url(#g)" stroke-width="54"/>
      <rect x="150" y="404" width="212" height="26" rx="13" fill="#e8a33d"/>
    </svg>
  </div>
  <div class="brand">Mesa de publicação</div>

  <nav class="rail-nav" aria-label="Seções">
    <button class="side big" data-tab hx-get="/noticias" hx-target="#lista">notícias</button>
    <button class="side big" data-tab hx-get="/analytics" hx-target="#lista">análises</button>
  </nav>

  <div class="rail-group" aria-label="Posts">
    <span class="rail-label">posts</span>
    <button class="side" data-tab hx-get="/posts?status=gerado" hx-target="#lista">rascunhos <span class="cnt">{gerado}</span></button>
    <button class="side" data-tab hx-get="/posts?status=aprovado" hx-target="#lista">prontos <span class="cnt">{aprovado}</span></button>
    <button class="side" data-tab hx-get="/posts?status=postado" hx-target="#lista">no ar <span class="cnt">{postado}</span></button>
    <button class="side" data-tab hx-get="/posts?status=descartado" hx-target="#lista">descartados <span class="cnt">{descartado}</span></button>
    <button class="side" data-tab hx-get="/posts" hx-target="#lista">todos</button>
  </div>

  <button class="side theme-btn" onclick="trocarTema()" id="theme-btn">tema claro</button>
  <div class="rail-foot">@enriquesouza_<br>@AizzyAi</div>
</aside>

<main>
  <div class="center-head">
    <h1 id="view-title">Rascunhos</h1>
    <span class="head-date">{today}</span>
    <button class="ctx-toggle" onclick="trocarContexto()" id="ctx-btn" title="mostrar/ocultar painel de contexto">contexto</button>
  </div>
  <div id="lista" hx-get="/posts?status=gerado" hx-trigger="load"></div>
</main>

<aside class="context">
  <section class="ctx-card">
    <h2>Hoje</h2>
    <div class="ctx-nums">
      <div><b>{postado}</b><span>no ar</span></div>
      <div><b>{views_total}</b><span>views</span></div>
      <div><b>{er}</b><span>engajamento</span></div>
    </div>
  </section>

  <section class="ctx-card">
    <h2>Como o X pontua</h2>
    <ul class="ctx-list">
      <li><b>Responda toda reply.</b> Reply que você responde vale ~150× um like.</li>
      <li><b>Espace os posts.</b> O segundo seguido do mesmo autor vale metade.</li>
      <li><b>Feche com pergunta.</b> Reply pesa 27× mais que like.</li>
      <li><b>1ª hora é triagem.</b> Não apague post lento antes de 24h.</li>
    </ul>
    <a class="ctx-link" href="https://github.com/xai-org/x-algorithm" target="_blank">xai-org/x-algorithm ↗</a>
  </section>
</aside>

<script>
const TITULOS = {{
  '/posts?status=gerado': 'Rascunhos',
  '/posts?status=aprovado': 'Prontos para postar',
  '/posts?status=postado': 'No ar',
  '/posts?status=descartado': 'Descartados',
  '/posts': 'Todos os posts',
  '/analytics': 'Análises',
  '/noticias': 'Notícias',
}};
document.addEventListener('click', function (ev) {{
  const tab = ev.target.closest('[data-tab]');
  if (!tab) return;
  document.querySelectorAll('[data-tab]').forEach(b => b.classList.remove('active'));
  tab.classList.add('active');
  const alvo = tab.getAttribute('hx-get') || '';
  const titulo = TITULOS[alvo] || (alvo.startsWith('/noticias') ? 'Notícias' : 'Análises');
  document.getElementById('view-title').textContent = titulo;
  localStorage.setItem('aba', alvo);
  history.replaceState(null, '', '#' + alvo);
}});
// A refresh must come back to the view he was reading — notícias by default.
window.addEventListener('load', function () {{
  const alvo = decodeURIComponent((location.hash || '').slice(1)) || localStorage.getItem('aba') || '/noticias';
  const tab = Array.from(document.querySelectorAll('[data-tab]'))
    .find(b => b.getAttribute('hx-get') === alvo);
  if (tab) setTimeout(() => tab.click(), 0);
}});
function trocarContexto() {{
  const corpo = document.body;
  corpo.classList.toggle('sem-contexto');
  localStorage.setItem('contexto', corpo.classList.contains('sem-contexto') ? 'fechado' : 'aberto');
}}
// context panel starts hidden unless he opened it before (calm by default)
if (localStorage.getItem('contexto') !== 'aberto') {{ document.body.classList.add('sem-contexto'); }}
function trocarTema() {{
  const raiz = document.documentElement;
  const claro = raiz.getAttribute('data-theme') === 'light';
  if (claro) {{
    raiz.removeAttribute('data-theme');
    localStorage.setItem('tema', 'escuro');
  }} else {{
    raiz.setAttribute('data-theme', 'light');
    localStorage.setItem('tema', 'claro');
  }}
  marcarBotaoTema();
}}
function marcarBotaoTema() {{
  const claro = document.documentElement.getAttribute('data-theme') === 'light';
  const botao = document.getElementById('theme-btn');
  if (botao) botao.textContent = claro ? 'tema escuro' : 'tema claro';
}}
marcarBotaoTema();
function copiarTexto(botao) {{
  const texto = botao.closest('.post-card').querySelector('textarea[name=final_text]').value;
  navigator.clipboard.writeText(texto);
  botao.textContent = 'copiado ✓';
  setTimeout(() => botao.textContent = 'copiar', 1500);
}}
function ajustar(area) {{
  area.style.height = 'auto';
  area.style.height = (area.scrollHeight + 2) + 'px';
}}
function ajustarTodos() {{
  document.querySelectorAll('.post-card textarea').forEach(ajustar);
}}
document.addEventListener('htmx:afterSwap', ajustarTodos);
document.addEventListener('htmx:load', ajustarTodos);
function medir(area) {{
  ajustar(area);
  const n = [...area.value].length;
  const card = area.closest('.post-card');
  const label = card.querySelector('.meter-num');
  label.classList.toggle('warn', n > {budget} - 30 && n <= {budget});
  label.classList.toggle('over', n > {budget});
  label.textContent = n + '/{budget}';
}}
</script>
</body>
</html>"##,
        css = page_css(),
        gerado = count_of("gerado"),
        aprovado = count_of("aprovado"),
        postado = count_of("postado"),
        descartado = count_of("descartado"),
        today = today,
        views_total = views_total,
        er = er,
        budget = POST_CHAR_BUDGET,
    );
    Html(page)
}

#[derive(Deserialize)]
struct PostsQuery {
    status: Option<String>,
    assunto: Option<String>,
}

async fn show_posts_part(
    State(state): State<AppState>,
    Query(query): Query<PostsQuery>,
) -> Html<String> {
    let db = &state.db;
    let posts = db.list_posts(query.status.as_deref()).unwrap_or_default();

    if posts.is_empty() {
        let msg = match query.status.as_deref() {
            Some("gerado") => {
                "Nenhum rascunho esperando. O gerador roda a cada 3 horas — os próximos chegam sozinhos."
            }
            Some("aprovado") => {
                "Nada aprovado ainda. Aprove um rascunho para ele aparecer aqui, pronto para postar."
            }
            Some("postado") => {
                "Nenhum post publicado ainda. Depois de postar no X, cole o link no card aprovado."
            }
            Some("descartado") => "Nada descartado. Bom sinal.",
            _ => "Nenhum post no banco ainda. Rode o rss-feed para gerar os primeiros rascunhos.",
        };
        return Html(format!("<p class='empty'>{msg}</p>"));
    }

    let mut html = String::new();
    let filtered = if query.status.is_some() {
        " filtrado"
    } else {
        ""
    };
    html.push_str(&format!("<div class='cards{filtered}'>"));
    for post_row in &posts {
        let metrics = db.list_post_metrics(post_row.id).unwrap_or_default();
        html.push_str(&post_card(post_row, &metrics));
    }
    html.push_str("</div>");
    Html(html)
}

async fn show_analytics_part(
    State(state): State<AppState>,
    Query(query): Query<PostsQuery>,
) -> Html<String> {
    let db = &state.db;
    let posted: Vec<PostRow> = match query.assunto.as_deref() {
        Some(subject) => db
            .list_posts_of_subject(subject)
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.status == "postado")
            .collect(),
        None => db.list_posts(Some("postado")).unwrap_or_default(),
    };

    if posted.is_empty() {
        let extra = match query.assunto.as_deref() {
            Some(subject) => format!(
                " Nenhum post de <b>{}</b> foi publicado ainda — gere um a partir de uma notícia desse assunto.",
                esc(subject)
            ),
            None => String::new(),
        };
        return Html(format!(
            "<p class='empty'>Sem posts publicados ainda. Publique um post, cole o link e as métricas chegam sozinhas.{extra}</p>"
        ));
    }

    let mut total_views = 0i64;
    let mut total_eng = 0i64;
    let mut table_rows = String::new();

    for post_row in &posted {
        let series = db.list_post_metrics(post_row.id).unwrap_or_default();
        let (views, likes, reposts, replies, bookmarks) = match series.last() {
            Some(m) => (m.views, m.likes, m.reposts, m.replies, m.bookmarks),
            None => (0, 0, 0, 0, 0),
        };
        let eng = likes + reposts + replies + bookmarks;
        total_views += views;
        total_eng += eng;

        let er = if views > 0 {
            format!("{:.1}%", eng as f64 * 100.0 / views as f64)
        } else {
            "—".to_string()
        };
        let views_series: Vec<i64> = series.iter().map(|m| m.views).collect();
        let spark = sparkline_svg(&views_series, 96.0, 26.0);
        let views_per_day = views_per_day_text(post_row, views);
        let x_link = post_row
            .x_url
            .as_deref()
            .map(|u| format!("<a href='{}' target='_blank'>abrir ↗</a>", esc(u)))
            .unwrap_or_else(|| "—".to_string());

        table_rows.push_str(&format!(
            "<tr><td class='mono'>#{id}</td><td>{topic}</td><td class='sparkcell'>{spark}</td><td class='mono'>{views}</td><td class='mono'>{eng}</td><td class='mono'>{er}</td><td class='mono'>{vpd}</td><td>{link}</td></tr>",
            id = post_row.id,
            topic = esc(&post_row.topic),
            spark = spark,
            views = views,
            eng = eng,
            er = er,
            vpd = views_per_day,
            link = x_link,
        ));
    }

    let er_total = if total_views > 0 {
        format!("{:.1}%", total_eng as f64 * 100.0 / total_views as f64)
    } else {
        "—".to_string()
    };

    Html(format!(
        r##"<div class="analytics-box">
<div class="kpis">
  <div class="kpi"><span class="kpi-num">{views}</span><span class="kpi-lbl">views totais</span></div>
  <div class="kpi"><span class="kpi-num">{eng}</span><span class="kpi-lbl">engajamentos</span></div>
  <div class="kpi teal"><span class="kpi-num">{er}</span><span class="kpi-lbl">taxa de engajamento</span></div>
  <div class="kpi"><span class="kpi-num">{n}</span><span class="kpi-lbl">posts no ar</span></div>
</div>
<table>
<thead><tr><th>#</th><th>tema</th><th>views ao longo do tempo</th><th>views</th><th>eng.</th><th>ER</th><th>views/dia</th><th>X</th></tr></thead>
<tbody>{rows}</tbody>
</table>
<p class="hint">ER = (likes + reposts + replies + bookmarks) ÷ views. Cada registro de métricas vira um ponto na linha — registre de tempos em tempos para ver a curva.</p>
</div>"##,
        n = posted.len(),
        views = total_views,
        eng = total_eng,
        er = er_total,
        rows = table_rows,
    ))
}

// ---------- actions ----------

async fn approve_post(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let db = &state.db;
    let _ = db.set_post_status(id, "aprovado");
    refreshed_card(db, id)
}

async fn discard_post(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let db = &state.db;
    let _ = db.set_post_status(id, "descartado");
    refreshed_card(db, id)
}

async fn reopen_post(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let db = &state.db;
    let _ = db.set_post_status(id, "gerado");
    refreshed_card(db, id)
}

#[derive(Deserialize)]
struct EditForm {
    final_text: String,
}

async fn edit_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<EditForm>,
) -> Html<String> {
    let db = &state.db;
    let _ = db.set_final_text(id, form.final_text.trim());
    refreshed_card(db, id)
}

#[derive(Deserialize)]
struct PostedForm {
    x_url: String,
}

async fn mark_posted(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<PostedForm>,
) -> Html<String> {
    let db = &state.db;
    let url = form.x_url.trim();
    if url.starts_with("https://x.com/") || url.starts_with("https://twitter.com/") {
        let _ = db.mark_post_posted(id, url);
    }
    refreshed_card(db, id)
}

#[derive(Deserialize)]
struct MetricsForm {
    views: Option<i64>,
    likes: Option<i64>,
    reposts: Option<i64>,
    replies: Option<i64>,
    bookmarks: Option<i64>,
    followers_total: Option<i64>,
}

async fn add_metrics(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<MetricsForm>,
) -> Html<String> {
    let db = &state.db;
    // A partial form (only "views", say) keeps every other number as it was —
    // an empty field means "unchanged", never "zero".
    let previous = db.list_post_metrics(id).unwrap_or_default();
    let last = previous.last();
    let keep = |typed: Option<i64>, old: Option<i64>| typed.or(old).unwrap_or(0);
    let _ = db.add_post_metrics(
        id,
        keep(form.views, last.map(|m| m.views)),
        keep(form.likes, last.map(|m| m.likes)),
        keep(form.reposts, last.map(|m| m.reposts)),
        keep(form.replies, last.map(|m| m.replies)),
        keep(form.bookmarks, last.map(|m| m.bookmarks)),
        form.followers_total
            .or(last.and_then(|m| m.followers_total)),
    );
    refreshed_card(db, id)
}

/// Read the public numbers straight from X and store them — no typing.
/// X only exposes likes and replies without a login, so views/reposts/
/// bookmarks stay at whatever the last manual snapshot had.
async fn refresh_metrics(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let db = &state.db;
    let Ok(Some(post_row)) = db.get_post(id) else {
        return Html("<p class='empty'>Post não encontrado.</p>".to_string());
    };
    let Some(post_url) = post_row.x_url.as_deref() else {
        return refreshed_card(db, id);
    };

    let card_now = |db: &PostsDb| match db.get_post(id) {
        Ok(Some(row)) => post_card(&row, &db.list_post_metrics(id).unwrap_or_default()),
        _ => String::new(),
    };

    match fetch_post_stats(&state.http, post_url).await {
        Ok(stats) => {
            let previous = db.list_post_metrics(id).unwrap_or_default();
            let last = previous.last();

            // Nothing new? Say so instead of piling identical snapshots.
            if let Some(last) = last
                && last.likes == stats.likes
                && last.replies == stats.replies
            {
                {
                    return Html(format!(
                        "{card}<p class='flash'>Sem mudança desde {when} — ainda {likes} likes e {replies} replies. <span class='mut'>Views não entram aqui: o X só mostra para quem está logado.</span></p>",
                        card = card_now(db),
                        when = esc(&last.taken_at),
                        likes = stats.likes,
                        replies = stats.replies,
                    ));
                }
            }

            // Carry the manual-only numbers forward so a refresh never zeroes them.
            let _ = db.add_post_metrics(
                id,
                last.map(|m| m.views).unwrap_or(0),
                stats.likes,
                last.map(|m| m.reposts).unwrap_or(0),
                stats.replies,
                last.map(|m| m.bookmarks).unwrap_or(0),
                last.and_then(|m| m.followers_total),
            );
            Html(format!(
                "{card}<p class='flash good'>Atualizado: {likes} likes · {replies} replies. <span class='mut'>Views seguem manuais — o X não as expõe fora do login.</span></p>",
                card = card_now(db),
                likes = stats.likes,
                replies = stats.replies,
            ))
        }
        Err(error) => Html(format!(
            "{card}<p class='flash bad'>Não deu para ler o post no X: {err}</p>",
            card = card_now(db),
            err = esc(&error.to_string()),
        )),
    }
}

/// Confirm a post really is live on X and stamp the real publish time.
async fn check_published(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let db = &state.db;
    let Ok(Some(post_row)) = db.get_post(id) else {
        return Html("<p class='empty'>Post não encontrado.</p>".to_string());
    };
    let Some(post_url) = post_row.x_url.as_deref() else {
        return refreshed_card(db, id);
    };

    match fetch_post_stats(&state.http, post_url).await {
        Ok(stats) => {
            let when = stats
                .created_at
                .as_deref()
                .map(|c| c.replace('T', " ").replace(".000Z", ""))
                .unwrap_or_else(|| "sem data".to_string());
            let card = match db.get_post(id) {
                Ok(Some(row)) => post_card(&row, &db.list_post_metrics(id).unwrap_or_default()),
                _ => String::new(),
            };
            Html(format!(
                "{card}<p class='flash good'>No ar desde {when} · {n} caracteres publicados</p>",
                n = stats.text.chars().count(),
            ))
        }
        Err(error) => Html(format!(
            "{card}<p class='flash bad'>X não devolveu esse post: {err}</p>",
            card = match db.get_post(id) {
                Ok(Some(row)) => post_card(&row, &db.list_post_metrics(id).unwrap_or_default()),
                _ => String::new(),
            },
            err = esc(&error.to_string()),
        )),
    }
}

#[derive(Deserialize)]
struct NewsQuery {
    dia: Option<String>,
}

/// The reading room as a front page: a capa with the day's strongest
/// stories, then one section per subject — a lead with its summary plus a
/// dense list of headlines, like a paper, not a wall of equal cards.
async fn show_news_part(
    State(state): State<AppState>,
    Query(query): Query<NewsQuery>,
) -> Html<String> {
    let db = &state.db;
    let days = db.list_story_days().unwrap_or_default();
    let stories = db
        .list_stories_of_day(query.dia.as_deref())
        .unwrap_or_default();
    // Repeated fetch rounds in one day re-save the same stories; the reader
    // should see each headline once — rows come newest-run first, keep that one.
    let mut seen_headlines: Vec<String> = Vec::new();
    let stories: Vec<StoryWithItems> = stories
        .into_iter()
        .filter(|entry| {
            let key = entry.story.headline.to_lowercase();
            if seen_headlines.contains(&key) {
                false
            } else {
                seen_headlines.push(key);
                true
            }
        })
        .collect();
    let post_counts = db.count_posts_by_story().unwrap_or_default();

    if days.is_empty() {
        return Html("<p class='empty'>Nenhuma notícia guardada ainda. Rode `cargo run --bin coletar` para buscar uma rodada agora, ou deixe o rss-feed rodando.</p>".to_string());
    }

    let chosen_day = query
        .dia
        .clone()
        .unwrap_or_else(|| days.first().map(|(d, _)| d.clone()).unwrap_or_default());

    let mut calendar = String::from("<div class='calendar'>");
    for (day, _count) in &days {
        let is_on = if *day == chosen_day { " on" } else { "" };
        let label = day.get(8..10).unwrap_or(day).to_string();
        let month = day.get(5..7).unwrap_or("").to_string();
        calendar.push_str(&format!(
            "<button class='day{is_on}' hx-get='/noticias?dia={day}' hx-target='#lista'><span class='d'>{label}/{month}</span></button>"
        ));
    }
    calendar.push_str("</div>");

    // Order subjects the way he reads, and stories by strength inside each.
    let mut by_subject: Vec<(String, Vec<&StoryWithItems>)> = Vec::new();
    for story in &stories {
        match by_subject
            .iter_mut()
            .find(|(name, _)| *name == story.story.subject)
        {
            Some((_, list)) => list.push(story),
            None => by_subject.push((story.story.subject.clone(), vec![story])),
        }
    }
    by_subject.sort_by_key(|(name, _)| subject_order(name));
    for (_, list) in &mut by_subject {
        list.sort_by_key(|e| std::cmp::Reverse(e.story.score));
    }

    let top_score = stories
        .iter()
        .map(|s| s.story.score)
        .max()
        .unwrap_or(1)
        .max(1);

    // Sections strip: where to jump, with counts.
    let mut strip = String::from("<nav class='sections-strip'>");
    for (subject, _list) in &by_subject {
        strip.push_str(&format!(
            "<a class='strip-link {cls}' href='#assunto-{slug}'>{sub}</a>",
            cls = subject_class(subject),
            slug = subject_slug(subject),
            sub = esc(subject),
        ));
    }
    strip.push_str("</nav>");

    // Capa: the day's strongest story, plus up to three runners-up.
    let mut ranked: Vec<&StoryWithItems> = stories.iter().collect();
    ranked.sort_by_key(|e| std::cmp::Reverse(e.story.score));
    let lead = ranked.first().copied();
    let secundarias: Vec<&StoryWithItems> = ranked.iter().skip(1).take(3).copied().collect();
    let capa_ids: Vec<i64> = lead
        .iter()
        .map(|e| e.story.id)
        .chain(secundarias.iter().map(|e| e.story.id))
        .collect();

    let mut capa = String::new();
    if let Some(entry) = lead {
        capa.push_str("<section class='capa'>");
        capa.push_str(&capa_lead_html(entry, top_score, &post_counts));
        if !secundarias.is_empty() {
            capa.push_str("<div class='capa-side'><span class='capa-side-title'>destaques</span>");
            for entry in &secundarias {
                capa.push_str(&capa_sec_html(entry, top_score));
            }
            capa.push_str("</div>");
        }
        capa.push_str("</section>");
    }

    // Sections: a lead with its summary + a dense headline list.
    let mut body = String::new();
    for (subject, list) in &by_subject {
        let rest: Vec<&&StoryWithItems> = list
            .iter()
            .filter(|e| !capa_ids.contains(&e.story.id))
            .collect();
        if rest.is_empty() {
            continue;
        }
        body.push_str(&format!(
            "<section class='subject {cls}' id='assunto-{slug}'><h2><span class='sub-name'>{sub}</span></h2>",
            cls = subject_class(subject),
            slug = subject_slug(subject),
            sub = esc(subject),
        ));
        body.push_str("<div class='section-grid'>");
        body.push_str(&section_lead_html(rest[0], top_score, &post_counts));
        if rest.len() > 1 {
            body.push_str("<div class='headline-list'>");
            for entry in rest.iter().skip(1).take(3) {
                body.push_str(&headline_row_html(entry, top_score, &post_counts));
            }
            if rest.len() > 4 {
                body.push_str("<details class='mais'><summary>mostrar mais</summary>");
                for entry in rest.iter().skip(4) {
                    body.push_str(&headline_row_html(entry, top_score, &post_counts));
                }
                body.push_str("</details>");
            }
            body.push_str("</div>");
        }
        body.push_str("</div></section>");
    }

    Html(format!(
        "<div class='news-page'><div class='edition'>{calendar}{strip}</div>{capa}{body}</div>",
    ))
}

/// Kicker line: SUBJECT · grade · day/time (+ draft chip when it exists).
fn kicker_html(entry: &StoryWithItems, top_score: i64, post_counts: &[(i64, i64)]) -> String {
    let story = &entry.story;
    let grade = importance_grade(story.score, top_score);
    let when = match (story.run_at.get(5..10), story.run_at.get(11..16)) {
        (Some(day), Some(clock)) => {
            let day = day.split('-').rev().collect::<Vec<_>>().join("/");
            format!("{day} · {clock}")
        }
        _ => story.run_at.clone(),
    };
    let drafts = post_counts
        .iter()
        .find(|(sid, _)| *sid == story.id)
        .map(|(_, n)| *n)
        .unwrap_or(0);
    let chip = if drafts > 0 {
        format!("<span class='chip done'>{drafts} rascunho(s)</span>")
    } else {
        String::new()
    };
    format!(
        "<span class='kicker'><b>{sub}</b> · {grade} · {when}</span>{chip}",
        sub = esc(&story.subject),
    )
}

/// The quiet action row every story carries.
fn acts_html(entry: &StoryWithItems) -> String {
    let story = &entry.story;
    let mut links = String::new();
    for item in &entry.items {
        let source = item.source.as_deref().unwrap_or("");
        match item.link.as_deref() {
            Some(link) => links.push_str(&format!(
                "<li><a href='{}' target='_blank'>{}</a> <span class='src'>{}</span></li>",
                esc(link),
                esc(&item.title),
                esc(source)
            )),
            None => links.push_str(&format!(
                "<li>{} <span class='src'>{}</span></li>",
                esc(&item.title),
                esc(source)
            )),
        }
    }
    format!(
        r##"<div class="acts">
  <button class="act warm" hx-post="/noticias/{id}/sugerir-post" hx-target="#flash-{id}" hx-indicator="#flash-{id}" hx-disabled-elt="this">sugerir post</button>
  <button class="act" hx-get="/analytics?assunto={subject}" hx-target="#lista">análises</button>
  <details class="links"><summary>{fontes}</summary><ul>{links}</ul></details>
</div>
<div id="flash-{id}" class="flash-slot"></div>"##,
        id = story.id,
        subject = esc(&story.subject),
        fontes = "fontes",
    )
}

/// Front-page lead: the biggest headline of the day, summary in serif.
fn capa_lead_html(entry: &StoryWithItems, top_score: i64, post_counts: &[(i64, i64)]) -> String {
    let story = &entry.story;
    // No AI summary yet? The group's own headlines make an honest dek.
    let digest = match story.digest.as_deref() {
        Some(d) => format!("<p class='dek'>{}</p>", esc(d)),
        None => {
            let bullets: String = entry
                .items
                .iter()
                .skip(1) // the first item IS the headline above
                .take(4)
                .map(|item| format!("<li>{}</li>", esc(&item.title)))
                .collect();
            format!("<ul class='dek-list'>{bullets}</ul>")
        }
    };
    format!(
        r##"<article class="capa-lead {cls}">
  <div class="kicker-row">{kicker}</div>
  <h3>{headline}</h3>
  {digest}
  {acts}
</article>"##,
        cls = subject_class(&story.subject),
        kicker = kicker_html(entry, top_score, post_counts),
        headline = esc(&story.headline),
        acts = acts_html(entry),
    )
}

/// Front-page runner-up: kicker + headline, nothing else.
fn capa_sec_html(entry: &StoryWithItems, top_score: i64) -> String {
    let story = &entry.story;
    let grade = importance_grade(story.score, top_score);
    let clock = story.run_at.get(11..16).unwrap_or("").to_string();
    format!(
        r##"<a class="capa-sec {cls}" href="#hist-{id}">
  <span class="kicker"><b>{sub}</b> · {grade} · {clock}</span>
  <h4>{headline}</h4>
</a>"##,
        cls = subject_class(&story.subject),
        id = story.id,
        sub = esc(&story.subject),
        headline = esc(&story.headline),
    )
}

/// Section lead: card with the summary, importance bar and actions.
fn section_lead_html(entry: &StoryWithItems, top_score: i64, post_counts: &[(i64, i64)]) -> String {
    let story = &entry.story;
    let digest = story
        .digest
        .as_deref()
        .map(|d| format!("<p class='dek short'>{}</p>", esc(d)))
        .unwrap_or_default();
    format!(
        r##"<article class="story {cls}" id="hist-{id}">
  <div class="kicker-row">{kicker}</div>
  <h3>{headline}</h3>
  {digest}
  {acts}
</article>"##,
        cls = subject_class(&story.subject),
        id = story.id,
        kicker = kicker_html(entry, top_score, post_counts),
        headline = esc(&story.headline),
        acts = acts_html(entry),
    )
}

/// Dense list row: headline + meta, actions on one quiet line.
fn headline_row_html(entry: &StoryWithItems, top_score: i64, post_counts: &[(i64, i64)]) -> String {
    let story = &entry.story;
    format!(
        r##"<article class="headline-row {cls}" id="hist-{id}">
  <div class="kicker-row">{kicker}</div>
  <h4>{headline}</h4>
  {acts}
</article>"##,
        cls = subject_class(&story.subject),
        id = story.id,
        kicker = kicker_html(entry, top_score, post_counts),
        headline = esc(&story.headline),
        acts = acts_html(entry),
    )
}

/// One accent class per subject — the paper's section colors.
fn subject_class(subject: &str) -> &'static str {
    match subject {
        "IA" => "sub-ia",
        "Rust" => "sub-rust",
        "Hacking" => "sub-hacking",
        "Crypto" => "sub-crypto",
        "Macro" => "sub-macro",
        _ => "sub-geral",
    }
}

/// Turn one news story into a post draft, using the same writer the daily
/// generator uses. The draft lands in "gerados" like any other.
async fn suggest_post_from_story(
    State(state): State<AppState>,
    Path(story_id): Path<i64>,
) -> Html<String> {
    let db = &state.db;
    let Ok(Some(entry)) = db.get_story(story_id) else {
        return Html("<p class='flash bad'>Notícia não encontrada.</p>".to_string());
    };

    // Feed the writer the English titles and sources — never the PT digest,
    // so the post comes out in English like the rest of the pipeline.
    let mut story_text = format!("Story: {}\n", entry.story.headline);
    for item in &entry.items {
        story_text.push_str(&format!(
            "- {} ({})\n",
            item.title,
            item.source.as_deref().unwrap_or("")
        ));
    }

    let writer = XPostsWriter::new(&state.http);
    match writer.write_x_posts(story_text).await {
        Ok(Some(answer)) => {
            let drafts = split_posts_text(&answer);
            match drafts.first() {
                Some(draft) => {
                    match db.save_post_from_story(
                        story_id,
                        &entry.story.subject,
                        &draft.topic,
                        &draft.text_en,
                    ) {
                        Ok(_) => Html(format!(
                            "<p class='flash good'>Rascunho criado a partir desta notícia. Ele está em <b>gerados</b>.<br><span class='mono'>{}</span></p>",
                            esc(&draft.text_en)
                        )),
                        Err(error) => Html(format!(
                            "<p class='flash bad'>Não deu para salvar o rascunho: {}</p>",
                            esc(&error.to_string())
                        )),
                    }
                }
                None => Html(
                    "<p class='flash bad'>O modelo não devolveu um post utilizável. Tente de novo.</p>"
                        .to_string(),
                ),
            }
        }
        Ok(None) => {
            Html("<p class='flash bad'>O modelo respondeu vazio. Tente de novo.</p>".to_string())
        }
        Err(error) => Html(format!(
            "<p class='flash bad'>O oMLX não respondeu: {}</p>",
            esc(&error.to_string())
        )),
    }
}

/// Turn the pipeline score into a word, relative to the strongest story
/// of the same day — importance only means anything next to its peers.
/// Anchor-friendly id for a subject name.
fn subject_slug(subject: &str) -> String {
    subject
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

fn importance_grade(score: i64, top_score: i64) -> &'static str {
    let share = score as f64 / top_score.max(1) as f64;
    if share >= 0.75 {
        "alta"
    } else if share >= 0.4 {
        "media"
    } else {
        "baixa"
    }
}

/// Keep the reading order stable and meaningful: what he builds with first.
fn subject_order(subject: &str) -> u8 {
    match subject {
        "IA" => 0,
        "Rust" => 1,
        "Hacking" => 2,
        "Crypto" => 3,
        "Macro" => 4,
        _ => 5,
    }
}

// ---------- rendering ----------

fn refreshed_card(db: &PostsDb, id: i64) -> Html<String> {
    match db.get_post(id) {
        Ok(Some(post_row)) => {
            let metrics = db.list_post_metrics(id).unwrap_or_default();
            Html(post_card(&post_row, &metrics))
        }
        _ => Html("<p class='empty'>Post não encontrado.</p>".to_string()),
    }
}

fn post_card(post_row: &PostRow, metrics: &[MetricsRow]) -> String {
    let id = post_row.id;
    let text_shown = post_row.final_text.as_deref().unwrap_or(&post_row.text_en);
    let chars = text_shown.chars().count();
    let meter_class = if chars > POST_CHAR_BUDGET {
        "over"
    } else if chars > POST_CHAR_BUDGET - 30 {
        "warn"
    } else {
        ""
    };

    let quote_html = post_row
        .quote_url
        .as_deref()
        .map(|u| {
            format!(
                "<p class='meta'><span class='meta-k'>quotar</span> <a href='{0}' target='_blank'>{0}</a></p>",
                esc(u)
            )
        })
        .unwrap_or_default();

    let aizzy_html = post_row
        .aizzy_text
        .as_deref()
        .map(|t| {
            format!(
                "<p class='meta aizzy'><span class='meta-k'>@AizzyAi</span> {}</p>",
                esc(t)
            )
        })
        .unwrap_or_default();

    let (card_state, badge_text) = match post_row.status.as_str() {
        "gerado" => ("state-gerado", "rascunho"),
        "aprovado" => ("state-aprovado", "pronto para postar"),
        "postado" => ("state-postado", "no ar"),
        _ => ("state-descartado", "descartado"),
    };

    let actions = match post_row.status.as_str() {
        "gerado" => format!(
            r#"<button class="btn warm" hx-post="/posts/{id}/aprovar" hx-target="closest .post-card" hx-swap="outerHTML">aprovar</button>
               <button class="btn ghost" hx-post="/posts/{id}/descartar" hx-target="closest .post-card" hx-swap="outerHTML">descartar</button>"#
        ),
        "aprovado" => format!(
            r#"<form class="inline" hx-post="/posts/{id}/postado" hx-target="closest .post-card" hx-swap="outerHTML">
                 <input name="x_url" placeholder="link do post no X" required>
                 <button class="btn warm">salvar link</button>
               </form>
               <button class="btn ghost" hx-post="/posts/{id}/voltar" hx-target="closest .post-card" hx-swap="outerHTML">voltar</button>"#
        ),
        "postado" => {
            let link = post_row
                .x_url
                .as_deref()
                .map(|u| {
                    format!(
                        "<a class='xlink' href='{u}' target='_blank'>ver no X ↗</a>",
                        u = esc(u)
                    )
                })
                .unwrap_or_default();
            let views_series: Vec<i64> = metrics.iter().map(|m| m.views).collect();
            let spark = sparkline_svg(&views_series, 120.0, 30.0);
            let last = metrics
                .last()
                .map(|m| {
                    format!(
                        "<span class='nums'><b class='auto'>{likes}</b> likes · <b class='auto'>{replies}</b> replies <span class='tag'>do X</span> · {views} views · {reposts} reposts · {bookmarks} bookmarks <span class='tag'>à mão</span></span>",
                        likes = m.likes,
                        replies = m.replies,
                        views = manual_number(m.views),
                        reposts = manual_number(m.reposts),
                        bookmarks = manual_number(m.bookmarks),
                    )
                })
                .unwrap_or_else(|| {
                    "<span class='mut'>sem leitura ainda — clique em atualizar métricas</span>"
                        .to_string()
                });
            let views_now = metrics
                .last()
                .map(|m| m.views)
                .filter(|v| *v > 0)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "só logado no X".to_string());
            format!(
                r#"<div class="posted-row">{link}<span class="sparkwrap">{spark}</span>{last}</div>
                <div class="auto-row">
                  <button class="btn warm" hx-post="/posts/{id}/atualizar-metricas" hx-target="closest .post-card" hx-swap="outerHTML" hx-indicator="closest .post-card" title="Lê likes e replies no X. Views só aparecem para quem está logado — preencha ao lado.">buscar likes e replies</button>
                  <button class="btn" hx-post="/posts/{id}/verificar" hx-target="closest .post-card" hx-swap="outerHTML" hx-indicator="closest .post-card">verificar publicação</button>
                  <form class="inline views-form" hx-post="/posts/{id}/metricas" hx-target="closest .post-card" hx-swap="outerHTML">
                    <label title="Abra o post no X (logado) e copie o número de views">views do X <input name="views" type="number" min="0" placeholder="{views_now}"></label>
                    <button class="btn ghost">salvar</button>
                  </form>
                </div>
                <details class="manual"><summary>completar à mão (views, reposts, bookmarks)</summary>
                <form class="inline metrics" hx-post="/posts/{id}/metricas" hx-target="closest .post-card" hx-swap="outerHTML">
                  <input name="views" type="number" placeholder="views" min="0">
                  <input name="likes" type="number" placeholder="likes" min="0">
                  <input name="reposts" type="number" placeholder="reposts" min="0">
                  <input name="replies" type="number" placeholder="replies" min="0">
                  <input name="bookmarks" type="number" placeholder="bookmarks" min="0">
                  <input name="followers_total" type="number" placeholder="seguidores" min="0">
                  <button class="btn">registrar</button>
                </form></details>"#
            )
        }
        _ => format!(
            r#"<button class="btn ghost" hx-post="/posts/{id}/voltar" hx-target="closest .post-card" hx-swap="outerHTML">restaurar</button>"#
        ),
    };

    format!(
        r#"<article class="post-card {card_state}">
  <div class="card-top">
    <span class="badge">{badge_text}</span>
    <span class="topic">{topic}</span>
    <span class="date mono">#{id} · {made_short}</span>
  </div>
  <form hx-post="/posts/{id}/editar" hx-target="closest .post-card" hx-swap="outerHTML">
    <textarea name="final_text" rows="2" oninput="medir(this)" aria-label="Texto do post">{text}</textarea>
    <div class="card-tools">
      <span class="meter-num mono {meter_class}">{chars}/{budget}</span>
      <button type="button" class="btn ghost" onclick="copiarTexto(this)">copiar</button>
      <button class="btn ghost">salvar edição</button>
    </div>
  </form>
  {quote_html}
  {aizzy_html}
  <div class="card-actions">{actions}</div>
</article>"#,
        topic = esc(&post_row.topic),
        made_short = {
            let raw = &post_row.made_at;
            match (raw.get(5..10), raw.get(11..16)) {
                (Some(day), Some(clock)) => {
                    let day = day.split('-').rev().collect::<Vec<_>>().join("/");
                    format!("{day} · {clock}")
                }
                _ => raw.clone(),
            }
        },
        text = esc(text_shown),
        budget = POST_CHAR_BUDGET,
    )
}

/// Single-series micro line chart: views over time. One hue, dot on the last
/// point, no axes — the exact values live in the mono text right beside it.
fn sparkline_svg(values: &[i64], width: f64, height: f64) -> String {
    if values.len() < 2 {
        return String::new();
    }
    let max = *values.iter().max().unwrap_or(&1) as f64;
    let min = *values.iter().min().unwrap_or(&0) as f64;
    let span = (max - min).max(1.0);
    let pad = 3.0;
    let step = (width - pad * 2.0) / (values.len() as f64 - 1.0);

    let points: Vec<String> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let x = pad + i as f64 * step;
            let y = pad + (height - pad * 2.0) * (1.0 - ((*v as f64 - min) / span));
            format!("{x:.1},{y:.1}")
        })
        .collect();

    let last = points.last().cloned().unwrap_or_default();
    let (lx, ly) = last.split_once(',').unwrap_or(("0", "0"));

    format!(
        r##"<svg class="spark" viewBox="0 0 {width:.0} {height:.0}" width="{width:.0}" height="{height:.0}" role="img" aria-label="views ao longo do tempo"><polyline fill="none" stroke="#58e7d6" stroke-width="2" stroke-linejoin="round" stroke-linecap="round" points="{pts}"/><circle cx="{lx}" cy="{ly}" r="3" fill="#58e7d6"/></svg>"##,
        pts = points.join(" "),
    )
}

fn views_per_day_text(post_row: &PostRow, views: i64) -> String {
    use chrono::NaiveDateTime;
    let Some(posted_at) = post_row.posted_at.as_deref() else {
        return "—".to_string();
    };
    let Ok(posted) = NaiveDateTime::parse_from_str(posted_at, "%Y-%m-%d %H:%M:%S") else {
        return "—".to_string();
    };
    let days = (chrono::Local::now().naive_local() - posted)
        .num_days()
        .max(1);
    format!("{}", views / days)
}

/// Numbers X only shows to a logged-in session: an unset one is a dash,
/// never a zero that would read like a measurement.
fn manual_number(value: i64) -> String {
    if value == 0 {
        "—".to_string()
    } else {
        value.to_string()
    }
}

fn esc(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn page_css() -> &'static str {
    r##"
@font-face { font-family: Charter; font-style: normal; font-weight: 400;
  src: local("Charter"), url(/fonts/charter-regular.woff2) format("woff2"); }
@font-face { font-family: Charter; font-style: normal; font-weight: 700;
  src: local("Charter Bold"), url(/fonts/charter-bold.woff2) format("woff2"); }
@font-face { font-family: Charter; font-style: italic; font-weight: 400;
  src: local("Charter Italic"), url(/fonts/charter-italic.woff2) format("woff2"); }
:root {
  color-scheme: dark;
  --ink: #070b0f;
  --amber-fill: #e8a33d;
  --prose: #cddae4;
  --panel: #0e151c;
  --panel-2: #121b24;
  --line: #1c2833;
  --teal: #2db2a3;
  --teal-bright: #58e7d6;
  --amber: #f0b25a;
  --amber-soft: #e8a33d33;
  --red: #e05252;
  --text: #dde3ea;
  --head: #e8e8e8;
  --mut: #aeb8c6;
  --sans: -apple-system, "SF Pro Text", "Helvetica Neue", sans-serif;
  --display: "Avenir Next", Avenir, -apple-system, "Segoe UI", "Helvetica Neue", Arial, sans-serif;
  --serif: Charter, "Bitstream Charter", Georgia, "Times New Roman", serif;
  --body: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif;
  --mono: ui-monospace, "SF Mono", Menlo, monospace;
}

/* Light theme: same variable names, values tuned for contrast on paper. */
:root[data-theme="light"] {
  color-scheme: light;
  --ink: #fafaf8;
  --panel: #ffffff;
  --panel-2: #eef2f6;
  --line: #d8e0e8;
  --teal: #17897d;
  --teal-bright: #0f766e;
  --amber: #92600a;
  --amber-fill: #e8a33d;
  --prose: #cddae4;
  --amber-soft: #f6e3c4;
  --red: #b32d2d;
  --text: #24292f;
  --head: #1c2128;
  --mut: #57606a;
  --prose: #2b3a46;
}
:root[data-theme="light"] body { background: var(--ink); }
:root[data-theme="light"] .rail { background: var(--ink); }
:root[data-theme="light"] .center-head { background: rgba(250, 250, 248, .9); }
:root[data-theme="light"] textarea { background: #fbfdff; }
:root[data-theme="light"] form.inline input { background: #fbfdff; }
:root[data-theme="light"] .state-postado .badge { background: #dff3f0; color: #0f7166; }
:root[data-theme="light"] .chip.done { background: #dff3f0; color: #0f7166; }
:root[data-theme="light"] .flash.good { background: #e6f6f3; border-color: var(--teal); color: #0d5f56; }
:root[data-theme="light"] .flash.bad { background: #fdecec; border-color: #e3a6a6; color: #8c2020; }
* { box-sizing: border-box; margin: 0; }
html { height: 100%; }
body {
  min-height: 100%;
  background: var(--ink);
  color: var(--text);
  font: 16px/1.5 var(--body);
  display: grid;
  /* left = where to go · center = the work · right = on-demand context */
  grid-template-columns: 240px minmax(0, 1fr) 320px;
  align-content: start;
}
.ctx-toggle {
  margin-left: 14px; background: none; border: 1px solid var(--line); border-radius: 999px;
  color: var(--mut); font: 500 12.5px var(--body); padding: 4px 12px; cursor: pointer;
}
.ctx-toggle:hover { color: var(--text); }
body.sem-contexto { grid-template-columns: 240px minmax(0, 1fr); }
body.sem-contexto .context { display: none; }

/* ---------- rail: calm flat navigation ---------- */
.rail {
  position: sticky; top: 0; align-self: start;
  height: 100dvh;
  display: flex; flex-direction: column;
  padding: 24px 16px;
  border-right: 1px solid var(--line);
  background: var(--ink);
}
.mark { margin-bottom: 8px; }
.mark svg { width: 34px; height: 34px; }
.brand {
  font: 600 14px/1.3 var(--body);
  color: var(--mut);
  margin-bottom: 28px;
}
.rail-nav { display: flex; flex-direction: column; gap: 2px; }
.rail-group { margin-top: 26px; display: flex; flex-direction: column; gap: 2px; }
.rail-label {
  font: 700 11px var(--body); letter-spacing: .08em; text-transform: uppercase;
  color: var(--mut); opacity: .75; padding: 0 10px 6px;
}
.side {
  display: flex; align-items: center; justify-content: space-between; gap: 8px;
  background: none; border: none; border-radius: 8px; cursor: pointer;
  color: var(--mut); font: 500 14px var(--body); text-align: left; padding: 8px 10px;
}
.side.big { font-size: 15px; font-weight: 600; color: var(--text); }
.side:hover { color: var(--text); background: var(--panel); }
.side.active { color: var(--text); background: var(--panel); box-shadow: inset 2px 0 0 var(--teal); }
.side .cnt { font: 500 12px var(--mono); color: var(--mut); opacity: .8; }
.theme-btn { margin-top: auto; justify-content: flex-start; }
.rail-foot { margin-top: 12px; color: var(--mut); font: 500 11.5px var(--mono); line-height: 1.7; opacity: .7; }

/* ---------- main column ---------- *//* ---------- main column ---------- */
main { min-width: 0; border-right: 1px solid var(--line); padding-bottom: 90px; }
.center-head {
  position: sticky; top: 0; z-index: 5;
  display: flex; align-items: baseline; gap: 12px;
  padding: 16px 26px 13px;
  background: rgba(7, 11, 15, .82);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--line);
}
.center-head h1 {
  font-family: var(--display); font-size: 21px; text-transform: uppercase;
  letter-spacing: .04em; margin: 0;
}
.head-date { margin-left: auto; color: var(--mut); font: 11.5px var(--mono); }
#lista { padding: 18px 26px 0; }
.top { margin-bottom: 24px; }
.eyebrow {
  font: 700 12px var(--display); text-transform: uppercase; letter-spacing: .22em;
  color: var(--teal-bright);
}
h1 {
  font-family: var(--display); font-weight: 700; text-transform: uppercase;
  font-size: clamp(28px, 4.4vw, 40px); line-height: 1.0; letter-spacing: .015em;
  margin: 6px 0 8px;
}
.top .mut { max-width: 62ch; }
.mut { color: var(--mut); }
.mono { font-family: var(--mono); font-variant-numeric: tabular-nums; }

/* ---------- context sidebar ---------- */
.context { padding: 16px 20px 60px; display: flex; flex-direction: column; gap: 12px;
  position: sticky; top: 0; align-self: start; max-height: 100dvh; overflow-y: auto; }
.ctx-card { background: var(--panel); border: 1px solid var(--line); border-radius: 14px; padding: 14px 16px; }
.ctx-card h2 {
  font: 700 10.5px var(--display); text-transform: uppercase; letter-spacing: .18em;
  color: var(--teal-bright); margin-bottom: 10px;
}
.ctx-nums { display: flex; gap: 14px; flex-wrap: wrap; }
.ctx-nums div { display: flex; flex-direction: column; }
.ctx-nums b { font: 700 22px/1.1 var(--display); font-variant-numeric: tabular-nums; }
.ctx-nums span { color: var(--mut); font-size: 11px; }
.ctx-list { list-style: none; padding: 0; margin: 0; display: grid; gap: 9px; }
.ctx-list li { font-size: 12.5px; line-height: 1.5; color: var(--mut); padding-left: 11px; position: relative; }
.ctx-list li::before { content: ''; position: absolute; left: 0; top: 8px; width: 4px; height: 4px; border-radius: 50%; background: var(--teal); }
.ctx-list b { color: var(--text); font-weight: 650; }
.ctx-link { display: inline-block; margin-top: 10px; color: var(--teal-bright); font: 11.5px var(--mono); text-decoration: none; }
.ctx-link:hover { text-decoration: underline; }

/* ---------- cards ---------- */
.post-card {
  background: var(--panel);
  border: 1px solid var(--line); border-left-width: 3px; border-radius: 14px;
  padding: 15px 17px; margin-bottom: 12px; max-width: 780px;
}
.cards { max-width: 820px; margin-inline: auto; }

/* On a filtered page every card has the same state — repeating it per card
   is noise. The badge and colored border only earn their ink in "todos". */
.filtrado .badge { display: none; }
.filtrado .post-card { border-left-width: 1px; border-left-color: var(--line); box-shadow: none; }
.state-gerado    { border-left-color: var(--amber); }
.state-aprovado  { border-left-color: var(--amber); box-shadow: 0 0 0 1px var(--amber-soft); }
.state-postado   { border-left-color: var(--teal); }
.state-descartado { border-left-color: var(--line); opacity: .55; }

.card-top { display: flex; gap: 12px; align-items: baseline; margin-bottom: 10px; }
.badge {
  font: 700 10.5px var(--display); text-transform: uppercase; letter-spacing: .14em;
  padding: 3px 9px; border-radius: 999px; white-space: nowrap;
}
.state-gerado .badge    { background: var(--amber-soft); color: var(--amber); }
.state-aprovado .badge  { background: var(--amber-fill); color: #140d02; }
.state-postado .badge   { background: #0e2f2b; color: var(--teal-bright); }
.state-descartado .badge { background: var(--panel-2); color: var(--mut); }
.topic { font-weight: 650; font-size: 14.5px; }
.date { margin-left: auto; color: var(--mut); font-size: 11.5px; }

textarea {
  width: 100%; background: var(--ink); color: var(--text);
  border: 1px solid var(--line); border-radius: 10px; padding: 12px 14px;
  font: 16px/1.5 var(--body); resize: vertical; max-width: 68ch;
}
textarea:focus { outline: none; border-color: var(--teal); }
.meter-num.warn { color: var(--amber); font-weight: 600; }
.meter-num.over { color: var(--red); font-weight: 700; }
.card-tools { display: flex; gap: 8px; align-items: center; margin-top: 8px; }
.meter-num { color: var(--mut); font-size: 12px; margin-right: auto; }

.meta { color: var(--mut); font-size: 13px; margin-top: 7px; word-break: break-all; }
.meta a { color: var(--teal-bright); text-decoration: none; }
.meta a:hover { text-decoration: underline; }
.meta-k {
  font: 700 10px var(--display); text-transform: uppercase; letter-spacing: .14em;
  color: var(--mut); margin-right: 6px;
}
.meta.aizzy .meta-k { color: var(--teal); }

.card-actions { margin-top: 12px; display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
.btn {
  background: var(--panel-2); color: var(--text);
  border: 1px solid var(--line); border-radius: 9px;
  padding: 8px 15px; font: 600 13px var(--body); cursor: pointer;
}
.btn:hover { border-color: var(--teal); }
.btn.warm { background: var(--amber-fill); border-color: var(--amber-fill); color: #140d02; }
.btn.warm:hover { filter: brightness(1.08); border-color: var(--amber); }
.btn.ghost { background: transparent; color: var(--mut); }
.btn.ghost:hover { color: var(--text); }
:focus-visible { outline: 2px solid var(--teal-bright); outline-offset: 2px; border-radius: 6px; }

form.inline { display: flex; gap: 7px; flex-wrap: wrap; align-items: center; }
form.inline input {
  background: var(--ink); color: var(--text); border: 1px solid var(--line);
  border-radius: 9px; padding: 8px 11px; font: 14px var(--body);
}
form.inline input:focus { outline: none; border-color: var(--teal); }
form.inline input[name="x_url"] { min-width: 250px; }
form.metrics input { width: 104px; }
.posted-row { display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-bottom: 9px; }
.xlink { color: var(--teal-bright); font-weight: 650; font-size: 13px; text-decoration: none; }
.xlink:hover { text-decoration: underline; }
.sparkwrap { display: inline-flex; align-items: center; }
.spark { display: block; }

.empty { color: var(--mut); padding: 44px 10px; text-align: center; max-width: 46ch; margin: 0 auto; }

/* ---------- analytics ---------- */
.kpis { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; margin-bottom: 20px; }
.kpi {
  background: linear-gradient(180deg, var(--panel-2) 0%, var(--panel) 100%);
  border: 1px solid var(--line); border-radius: 14px; padding: 14px 16px;
}
.kpi.teal { border-color: var(--teal); }
.kpi-num { display: block; font: 700 30px/1.1 var(--display); letter-spacing: .01em; font-variant-numeric: tabular-nums; }
.kpi-lbl { color: var(--mut); font-size: 12px; }
table { width: 100%; border-collapse: collapse; font-size: 13px; }
th {
  text-align: left; padding: 7px 9px; border-bottom: 1px solid var(--line);
  color: var(--mut); font: 700 10.5px var(--display); text-transform: uppercase; letter-spacing: .12em;
}
td { padding: 8px 9px; border-bottom: 1px solid var(--panel-2); vertical-align: middle; }
td a { color: var(--teal-bright); text-decoration: none; }
td a:hover { text-decoration: underline; }
.sparkcell { width: 104px; }
.hint { color: var(--mut); font-size: 12px; margin-top: 12px; }

/* ---------- automatic metrics row + flashes ---------- */
.auto-row { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 8px; }
.manual > summary { color: var(--mut); font-size: 12px; cursor: pointer; margin-bottom: 8px; }
.manual > summary:hover { color: var(--text); }
.flash {
  margin: 10px 0 0; padding: 9px 13px; border-radius: 10px;
  font-size: 13px; line-height: 1.5; border: 1px solid var(--line);
}
/* only a flash that follows a post card hugs it */
#lista > .flash { margin: -6px 0 14px; }
.flash.good { background: #0e2f2b; border-color: var(--teal); color: var(--teal-bright); }
.flash.bad { background: #2a1414; border-color: #5c2b2b; color: #f0a8a8; }
.post-card.htmx-request { opacity: .55; transition: opacity .15s ease; }

/* ---------- news reading room ---------- */
.calendar { display: flex; gap: 6px; overflow-x: auto; padding-bottom: 10px; margin-bottom: 6px; }
.day {
  flex: none; display: flex; flex-direction: column; align-items: center; gap: 1px;
  background: var(--panel); border: 1px solid var(--line); border-radius: 10px;
  padding: 7px 11px; cursor: pointer; color: var(--mut);
}
.day .d { font: 700 12px var(--mono); }
.day .c { font-size: 10px; opacity: .75; }
.day:hover { color: var(--text); border-color: var(--teal); }
.day.on { background: var(--panel-2); border-color: var(--teal); color: var(--teal-bright); }
.subject { margin-top: 22px; }
.subject h2 {
  font: 700 13px var(--display); text-transform: uppercase; letter-spacing: .2em;
  color: var(--teal-bright); padding-bottom: 7px; border-bottom: 1px solid var(--line);
}
.run { margin-top: 12px; }
.run-when {
  font: 700 10.5px var(--display); text-transform: uppercase; letter-spacing: .14em;
  color: var(--amber);
}
.run ul { list-style: none; padding: 0; margin: 5px 0 0; }
.run li { padding: 5px 0; border-bottom: 1px solid var(--panel-2); font-size: 13.5px; }
.run li a { color: var(--text); text-decoration: none; }
.run li a:hover { color: var(--teal-bright); }
.run .src { color: var(--mut); font: 11px var(--mono); margin-left: 6px; }

/* ---------- news portal: consolidated stories ---------- */
/* subject index — counts + jump links */
.subject-index { display: grid; grid-template-columns: repeat(auto-fit, minmax(112px, 1fr)); gap: 8px; margin: 12px 0 4px; }
.ix {
  background: var(--panel); border: 1px solid var(--line); border-radius: 12px;
  padding: 10px 12px; display: flex; flex-direction: column; gap: 1px; text-decoration: none;
}
.ix:hover { border-color: var(--teal); }
.ix-sub { font: 700 10.5px var(--display); text-transform: uppercase; letter-spacing: .16em; color: var(--teal-bright); }
.ix-n { font: 700 24px/1.1 var(--display); font-variant-numeric: tabular-nums; color: var(--text); }
.ix-lbl { color: var(--mut); font-size: 11px; }
.subject h2 { display: flex; align-items: baseline; gap: 10px; }
.sub-count { margin-left: auto; color: var(--mut); font: 500 11px var(--mono); letter-spacing: 0; text-transform: none; }
/* newspaper grid: lead story full width, the rest in columns */
.story-grid {
  display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 12px; margin-top: 10px;
  align-items: start; /* each card ends where its content ends */
}
.story {
  background: var(--panel); border: 1px solid var(--line); border-left-width: 3px;
  border-radius: 12px; padding: 13px 15px; margin-top: 10px;
  display: flex; flex-direction: column;
}
.story-grid .story { margin-top: 0; }
.story.lead {
  background: linear-gradient(180deg, var(--panel-2) 0%, var(--panel) 100%);
  padding: 20px 22px; border-left-width: 4px; margin: 16px 0 4px;
}
.story.lead h3 { font-size: clamp(20px, 2.6vw, 27px); font-family: var(--display); text-transform: none; line-height: 1.15; letter-spacing: .005em; }
.story.lead .digest { font-size: 14.5px; }
.digest.short {
  display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical;
  overflow: hidden; font-size: 13px;
}
.story-actions { display: flex; gap: 7px; flex-wrap: wrap; align-items: center; margin-top: 12px; }
.story-actions .btn { padding: 6px 12px; font-size: 12px; }
.chip { font: 700 9.5px var(--display); text-transform: uppercase; letter-spacing: .12em; padding: 2px 7px; border-radius: 999px; }
.chip.done { background: #0e2f2b; color: var(--teal-bright); }
.flash-slot:empty { display: none; }
.flash-slot { max-width: 100%; overflow-wrap: anywhere; }
.flash-slot.htmx-request { display: block; }
.flash { background: var(--panel-2); color: var(--mut); }
.nums { font: 12.5px var(--mono); color: var(--mut); }
.nums b.auto { color: var(--text); font-weight: 700; }
.views-form { margin-left: 4px; }
.views-form label { color: var(--mut); font: 12px var(--mono); display: flex; align-items: center; gap: 5px; }
.views-form input { width: 120px; }
.tag {
  font: 700 9px var(--display); text-transform: uppercase; letter-spacing: .12em;
  padding: 1px 6px; border-radius: 999px; background: var(--line); color: var(--mut);
  margin: 0 2px;
}
.flash-slot.htmx-request::after {
  content: 'escrevendo o post no modelo local… leva cerca de 1 minuto';
  display: block; margin-top: 10px; padding: 9px 13px; border-radius: 10px;
  background: var(--amber-soft); color: var(--amber); font-size: 12.5px;
}
/* a button mid-request says so, instead of looking ignored */
.btn:disabled { opacity: .55; cursor: progress; }
.story.grade-alta { border-left-color: var(--amber); }
.story.grade-media { border-left-color: var(--teal); }
.story.grade-baixa { border-left-color: var(--line); }
.story-top { display: flex; align-items: center; gap: 9px; margin-bottom: 7px; }
.imp { flex: 1; height: 4px; background: var(--line); border-radius: 2px; overflow: hidden; max-width: 190px; }
.imp-bar { display: block; height: 100%; border-radius: 2px; background: var(--teal-bright); }
.grade-alta .imp-bar { background: var(--amber-fill); }
.grade-baixa .imp-bar { background: var(--mut); opacity: .6; }
.imp-txt { font: 700 10px var(--display); text-transform: uppercase; letter-spacing: .16em; color: var(--mut); }
.grade-alta .imp-txt { color: var(--amber); }
.story-when { margin-left: auto; color: var(--mut); font: 11px var(--mono); }
.story h3 { font-size: 15px; font-weight: 650; line-height: 1.35; }
.story-meta { color: var(--mut); font: 11.5px var(--mono); margin-top: 3px; }
.digest { margin-top: 9px; font-size: 13.5px; line-height: 1.6; color: var(--prose); white-space: pre-wrap; max-width: 82ch; }
.links > summary { margin-top: 9px; color: var(--mut); font-size: 12px; cursor: pointer; }
.links > summary:hover { color: var(--text); }
.links ul { list-style: none; padding: 0; margin: 7px 0 0; }
.links li { padding: 4px 0; border-bottom: 1px solid var(--panel-2); font-size: 13px; }
.links li a { color: var(--teal-bright); text-decoration: none; }
.links li a:hover { text-decoration: underline; text-underline-offset: 2px; }
.links li a::after { content: ' ↗'; color: var(--mut); font-size: 10px; }

/* ---------- the paper: front page + sections ---------- */
.news-page { display: flex; flex-direction: column; gap: 18px; }
.edition {
  display: flex; align-items: center; justify-content: space-between;
  gap: 14px; flex-wrap: wrap;
  border-bottom: 3px double var(--line); padding-bottom: 12px;
}
.sections-strip { display: flex; gap: 4px; flex-wrap: wrap; }
.strip-link {
  display: inline-flex; align-items: center; gap: 7px;
  padding: 5px 11px; border-radius: 999px;
  font-family: var(--display); font-size: 13px; letter-spacing: .09em;
  text-transform: uppercase; text-decoration: none;
  color: var(--mut); border: 1px solid transparent;
}
.strip-link b { color: var(--acc, var(--mut)); font-size: 12px; }
.strip-link:hover { border-color: var(--line); color: var(--text); }

/* section accents — the paper's editorial colors */
.sub-ia { --acc: #58e7d6; }
.sub-rust { --acc: #e8a33d; }
.sub-hacking { --acc: #ef6363; }
.sub-crypto { --acc: #6ea8fe; }
.sub-macro { --acc: #b58cf0; }
.sub-geral { --acc: #9aa8b5; }
:root[data-theme="light"] .sub-ia { --acc: #0d7268; }
:root[data-theme="light"] .sub-rust { --acc: #8a5a10; }
:root[data-theme="light"] .sub-hacking { --acc: #b32d2d; }
:root[data-theme="light"] .sub-crypto { --acc: #2c5eb0; }
:root[data-theme="light"] .sub-macro { --acc: #6b3fb5; }
:root[data-theme="light"] .sub-geral { --acc: #5b6874; }

.kicker-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.kicker {
  font-family: var(--display); font-size: 12.5px; letter-spacing: .1em;
  text-transform: uppercase; color: var(--mut);
}
.kicker b { color: var(--acc, var(--amber)); font-weight: 700; }

/* capa — the day's mosaic */
.capa {
  display: grid; grid-template-columns: minmax(0, 1.7fr) minmax(280px, 420px);
  gap: 0; align-items: start;
  border-bottom: 1px solid var(--line); padding-bottom: 20px;
}
.capa-lead { padding-right: 26px; display: flex; flex-direction: column; gap: 12px; }
.capa-lead h3 {
  margin: 0; font-family: var(--display);
  font-size: clamp(26px, 3.2vw, 40px); line-height: 1.08;
  letter-spacing: -0.015em; font-weight: 700; color: var(--head, var(--text));
  text-wrap: balance;
}
.capa-side {
  border-left: 1px solid var(--line); padding-left: 26px;
  display: flex; flex-direction: column; gap: 0; min-width: 0;
}
.capa-side-title {
  font-family: var(--display); font-size: 11px; letter-spacing: .22em;
  text-transform: uppercase; color: var(--mut); padding-bottom: 10px;
}
.capa-sec {
  display: flex; flex-direction: column; gap: 6px;
  padding: 12px 0; border-top: 1px solid var(--line);
  text-decoration: none;
}
.capa-sec h4 {
  margin: 0; font-family: var(--display); font-size: 17px;
  line-height: 1.25; font-weight: 600; color: var(--head, var(--text)); text-wrap: balance;
}
.capa-sec:hover h4 { color: var(--acc, var(--amber)); }

/* serif deks — the newspaper's voice */
.dek {
  margin: 0; font-family: var(--serif); text-wrap: pretty;
  font-size: 18px; line-height: 1.5; color: var(--prose);
  max-width: 65ch;
}
.dek-list {
  margin: 0; padding-left: 20px;
  font-family: var(--serif);
  font-size: 17px; line-height: 1.5; color: var(--prose);
  display: flex; flex-direction: column; gap: 7px; max-width: 62ch;
}
.dek-list li::marker { color: var(--acc, var(--mut)); }
.dek.short {
  font-size: 16px; line-height: 1.5;
  display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;
  overflow: hidden;
}
.meta-min { margin: 0; font-size: 12.5px; color: var(--mut); }

/* sections */
.subject h2 {
  display: flex; align-items: baseline; gap: 14px; margin: 28px 0 14px;
  font-family: var(--display); font-size: 15px; letter-spacing: .16em;
  text-transform: uppercase; color: var(--acc, var(--text));
}
.subject h2::after { content: ""; flex: 1; border-top: 1px solid var(--line); align-self: center; }
.subject h2 .sub-count {
  font-size: 11.5px; letter-spacing: .08em; color: var(--mut);
  text-transform: none; font-family: var(--sans); order: 3;
}
.section-grid {
  display: grid; grid-template-columns: minmax(0, 1.25fr) minmax(0, 1fr);
  gap: 22px; align-items: start;
}
.section-grid:has(> .story:only-child) { grid-template-columns: minmax(0, 1fr); }
.headline-list { display: flex; flex-direction: column; min-width: 0; }
.headline-row {
  display: flex; flex-direction: column; gap: 6px;
  padding: 13px 0; border-top: 1px solid var(--line);
}
.headline-row:first-child { border-top: 0; padding-top: 2px; }
.headline-row h4 {
  margin: 0; font-family: var(--display); font-size: 17px;
  line-height: 1.3; font-weight: 600; color: var(--head, var(--text)); text-wrap: balance;
}

/* the section lead deserves a real headline, not a card title */
.section-grid .story h3 {
  font-family: var(--display); font-size: 22px; line-height: 1.18;
  font-weight: 700; letter-spacing: -0.01em; text-wrap: balance;
}

.mais summary {
  cursor: pointer; list-style: none; padding: 12px 0 4px;
  color: var(--mut); font: 500 13.5px var(--body);
  text-decoration: underline; text-underline-offset: 3px;
  text-decoration-color: color-mix(in srgb, currentColor 35%, transparent);
}
.mais summary:hover { color: var(--text); }
.mais summary::after { content: " ▾"; font-size: 10px; }
.mais[open] summary::after { content: " ▴"; }

/* quiet action row */
.acts { display: flex; align-items: center; gap: 14px; flex-wrap: wrap; }
.act {
  background: none; border: 0; padding: 0; cursor: pointer;
  font-family: var(--sans); font-size: 12.5px; color: var(--mut);
  text-decoration: underline; text-underline-offset: 3px;
  text-decoration-color: color-mix(in srgb, currentColor 35%, transparent);
}
.act:hover { color: var(--text); }
.act.warm { color: var(--amber); font-weight: 600; }
.act.warm:hover { color: var(--text); }
.act:disabled { opacity: .45; cursor: wait; }
.acts .links { font-size: 12.5px; }
.acts .links summary {
  cursor: pointer; color: var(--mut); list-style: none;
}
.acts .links summary::after { content: " ▾"; font-size: 10px; }
.acts .links[open] summary::after { content: " ▴"; }
.acts .links summary:hover { color: var(--text); }
.acts .links ul { margin: 8px 0 2px; padding-left: 16px; display: flex; flex-direction: column; gap: 5px; }

/* ---------- htmx settle ---------- */
#lista.htmx-swapping { opacity: 0; transition: opacity .12s ease; }
#lista { opacity: 1; transition: opacity .16s ease; }

/* ---------- mobile: rail becomes a top strip ---------- */
@media (max-width: 1180px) {
  body { grid-template-columns: 240px minmax(0, 1fr); }
  .context { display: none; }
}
@media (max-width: 900px) {
  .capa { grid-template-columns: minmax(0, 1fr); }
  .capa-lead { padding-right: 0; }
  .capa-side { border-left: 0; padding-left: 0; margin-top: 16px; border-top: 3px double var(--line); padding-top: 14px; }
  .section-grid { grid-template-columns: minmax(0, 1fr); }
}
@media (max-width: 760px) {
  body { grid-template-columns: 1fr; }
  main { border-right: none; }
  .center-head { padding: 12px 16px 10px; }
  #lista { padding: 14px 16px 0; }
  .rail {
    position: static; height: auto; flex-direction: row; align-items: center;
    flex-wrap: wrap; gap: 4px 10px; padding: 14px 16px;
    border-right: none; border-bottom: 1px solid var(--line);
  }
  .mark svg { width: 30px; height: 30px; }
  .brand { margin: 0 8px 0 0; }
  .rail-nav { flex-direction: row; }
  .rail-group { flex-direction: row; margin: 0 0 0 auto; align-items: center; }
  .rail-label { display: none; }
  .side { padding: 6px 8px; }
  .theme-btn { margin-top: 0; }
  .rail-foot { display: none; }
  main { padding: 20px 14px 70px; }
  form.metrics input { width: 30%; min-width: 90px; flex: 1; }
}

@media (prefers-reduced-motion: reduce) {
  * { animation: none !important; transition: none !important; }
}
"##
}
