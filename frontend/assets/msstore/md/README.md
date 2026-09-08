# `assets/msstore/md/` — md for Windows store pages

The two legal pages the Microsoft Store listing of **md for Windows** links
to — the Windows counterpart of `assets/appstore/md/` (App Store) and
`assets/play/md/` (Google Play). Both pages are static, edited by hand in
this folder, and follow the same HTML skeleton and stylesheet as the
App Store pair.

| URL                                            | Source                                                | Used by                                                       |
| ---------------------------------------------- | ----------------------------------------------------- | ------------------------------------------------------------- |
| `https://nettrash.me/msstore/md/privacy.html`  | `privacy.html`, edited by hand in this folder.        | Partner Center ▸ Privacy policy URL (mandatory, policy 10.5.1); the app's Help ▸ Privacy Policy. |
| `https://nettrash.me/msstore/md/support.html`  | `support.html`, edited by hand in this folder.        | Partner Center ▸ Support contact info / website; the app's Help ▸ md Help (F1). |

The source of truth for the facts on these pages is the `md.win` repo:
`PRIVACY.md` (the same policy in Markdown) and `store/` (the listing copy).
Change a fact there first, then here in the same sitting — the two must
never disagree, because the listing links to this page and App Review-style
metadata checks compare them.

## Wiring

1. **Trunk copy-dir — done.** `frontend/index.html` carries
   `<link data-trunk rel="copy-dir" href="assets/msstore" />` beside the
   `assets/appstore` and `assets/play` lines, so `trunk build --release`
   writes this folder to `dist/msstore/md/` and nginx serves it at
   `https://nettrash.me/msstore/md/`. Verified by building: `dist/msstore/md/`
   holds `privacy.html` and `support.html`. **Deployed and live** — both URLs
   answered 200 on 2026-09-08, so the mandatory privacy-policy URL is in place.
2. **Home-page card — done 2026-09-08, at nettrash's request.** The home page
   (`frontend/src/components/home.rs`) now carries a fifth tab, **Microsoft
   Store** (`HomeTab::MsStore` / `MsStoreTab`, session key `msstore`), with one
   md card built on the same Bootstrap skeleton as the App Store and Play cards.
   Its icon is `frontend/assets/md-windows-icon.png` — md.win's 300 × 300 store
   tile (`md.win/store/logos/app-tile-icon-300x300.png`), which is *different
   art* from the Apple `md-icon.png`; root icons are copied per file, so it has
   its own `copy-file` line in `frontend/index.html`.

   **The store link is a placeholder.** md is not in the public catalog yet — a
   Display Catalog lookup on the package family name `nttrsh.nettrash.md_hrycnkw7hr1b6`
   returned `TotalResultCount: 1` with no products on 2026-09-08 — so both the
   title link and the "Get it from Microsoft Store" button point at
   `https://apps.microsoft.com/detail/9NXXXXXXXXXX` and do **not** resolve.
   A `TODO(md.win)` comment above the card says so. Swap in the product ID
   Partner Center assigns once the listing goes live; nothing else changes.

   The card's copy follows `md.win/store/README.md` ▸ *Wording that must not
   drift back*: never "no third-party dependencies" (the engines are bundled),
   never "no network" (the preview fetches an image the document names), and
   Windows 11 only — never imply Windows 10.
