//! Live GitHub repository metadata fetcher with sessionStorage caching.
//!
//! The `GitHubTab` on the Home page used to hard-code stars, forks,
//! and version-tag strings per repository. This module replaces those
//! constants with a live fetch from the GitHub public REST API on first
//! render of the tab, caches the result in `sessionStorage` with a
//! 1-hour TTL so subsequent renders within the session are free, and
//! lets the call site keep its hardcoded fallback for offline /
//! rate-limited / first-paint scenarios.
//!
//! Why sessionStorage and not localStorage: rest of the site already
//! treats `sessionStorage` as the persistence layer for ephemeral UI
//! state; doing the same here keeps cache invalidation predictable —
//! quitting the browser tab clears it, which is the right thing for
//! "data that's stale within an hour anyway."
//!
//! Why GitHub's REST API and not GraphQL: REST endpoints serve
//! `Access-Control-Allow-Origin: *` for unauthenticated public data so
//! the request goes browser-to-GitHub directly, no CORS proxy needed.
//! GraphQL requires auth and we don't want to bake a token into a
//! WASM bundle that ships to every visitor.
//!
//! Rate limiting: 60 requests / hour / IP for unauthenticated calls.
//! Each repo costs 1–2 requests (one for the repo metadata, one for
//! the latest tag if the repo has Releases configured), so the five
//! cards on the GitHub tab consume up to 10 requests on a fresh
//! session — well within the hourly budget for a personal site. The
//! cache means re-renders within the session add zero requests.

use chrono::Utc;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::storage;

/// One hour, in seconds. Tunable; pick anything between ~10 minutes
/// (snappier "I just released" feedback for the maintainer) and a
/// day (cheap on the rate budget). One hour is the sweet spot for a
/// site that's typically read more than re-released to.
const CACHE_TTL_SECS: i64 = 60 * 60;

/// Subset of GitHub repo metadata that the GitHub tab actually
/// renders. We don't fetch description or language because the
/// hardcoded copy in `home.rs` is intentionally richer than the
/// repo's GitHub description, and the framework badges
/// ("Swift / SwiftUI", "Kotlin / Jetpack Compose") don't match the
/// `language` API field's primary-language signal.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GhRepoInfo {
    pub stars: u32,
    pub forks: u32,
    /// Latest tag name as published by GitHub. Comes from the
    /// `/releases/latest` endpoint when the repo cuts formal Releases,
    /// otherwise from the first entry of `/tags` (which lists every
    /// git tag, in reverse-chronological order).
    pub latest_tag: Option<String>,
}

/// Cache envelope: the live data plus the wall-clock timestamp it
/// was fetched. Stored as JSON in sessionStorage under
/// `gh_repo:{owner}/{repo}`.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Cached {
    fetched_at: i64,
    data: GhRepoInfo,
}

fn cache_key(owner: &str, repo: &str) -> String {
    format!("gh_repo:{}/{}", owner, repo)
}

fn load_cached(owner: &str, repo: &str) -> Option<GhRepoInfo> {
    let raw = storage::get(&cache_key(owner, repo))?;
    let cached: Cached = serde_json::from_str(&raw).ok()?;
    let age = Utc::now().timestamp() - cached.fetched_at;
    // `age >= 0` guards against a clock that's jumped backwards — if
    // the cache says it was fetched in the future, treat it as stale
    // and re-fetch rather than serving content with a nonsensical age.
    if age >= 0 && age < CACHE_TTL_SECS {
        Some(cached.data)
    } else {
        None
    }
}

fn store_cached(owner: &str, repo: &str, data: &GhRepoInfo) {
    let cached = Cached {
        fetched_at: Utc::now().timestamp(),
        data: data.clone(),
    };
    if let Ok(json) = serde_json::to_string(&cached) {
        storage::set(&cache_key(owner, repo), &json);
    }
}

// ---------------------------------------------------------------------------
// GitHub API response shapes — only the fields we read are declared.
// `serde(deny_unknown_fields)` is intentionally NOT set: the API ships
// many keys we don't care about, and we want to ignore them.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ApiRepo {
    stargazers_count: u32,
    forks_count: u32,
}

#[derive(Debug, Deserialize)]
struct ApiRelease {
    tag_name: String,
}

#[derive(Debug, Deserialize)]
struct ApiTag {
    name: String,
}

/// Fetch metadata for one repo. Honours the 1-hour sessionStorage
/// cache; on cache miss / staleness, hits the GitHub API for the
/// repo and the latest tag, then writes the result back to cache.
///
/// Returns `None` on any error — caller falls back to hardcoded
/// values defined per-project in `home.rs`. Errors aren't surfaced
/// to the UI: a rate-limited / offline visitor sees the same
/// rendering they'd have seen before this module existed.
pub async fn fetch_repo_info(owner: &str, repo: &str) -> Option<GhRepoInfo> {
    if let Some(cached) = load_cached(owner, repo) {
        return Some(cached);
    }

    let repo_url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    let api: ApiRepo = Request::get(&repo_url)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    let latest_tag = fetch_latest_tag(owner, repo).await;

    let info = GhRepoInfo {
        stars: api.stargazers_count,
        forks: api.forks_count,
        latest_tag,
    };
    store_cached(owner, repo, &info);
    Some(info)
}

/// Two-tier latest-tag lookup. GitHub Releases is opt-in — repos
/// that just `git tag` without going through the Releases UI have an
/// empty `/releases/latest` response (HTTP 404), but their tags are
/// still listed under `/tags`. Try the canonical Releases endpoint
/// first, fall back to the bare tag list if that comes up empty.
async fn fetch_latest_tag(owner: &str, repo: &str) -> Option<String> {
    let release_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );
    if let Ok(resp) = Request::get(&release_url).send().await {
        if resp.ok() {
            if let Ok(release) = resp.json::<ApiRelease>().await {
                return Some(release.tag_name);
            }
        }
    }

    let tags_url = format!("https://api.github.com/repos/{}/{}/tags", owner, repo);
    let tags: Vec<ApiTag> = Request::get(&tags_url)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    tags.into_iter().next().map(|t| t.name)
}
