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
   holds `privacy.html` and `support.html`. It still has to be **deployed** —
   the two URLs must answer 200 before the Partner Center submission is sent,
   or the mandatory privacy-policy URL fails certification.
2. **Home-page card — nettrash's call, deliberately not done.** No Microsoft
   Store card has been added to the home page (`frontend/src/…/home.rs`) and no
   app icon has been copied for one. That is a decision about the site's shape,
   not a prerequisite for the submission, and it waits until the listing is live
   and its Store URL is known. Do not add it unasked.
