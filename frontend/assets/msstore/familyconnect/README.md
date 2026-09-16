# `assets/msstore/familyconnect/` — Family Connect for Windows store pages

The two legal pages the Microsoft Store listing of **Family Connect for Windows** links to — the Windows counterpart of
`assets/appstore/familyconnect/` (App Store) and `assets/play/familyconnect/` (Google Play). Both are static, edited by
hand in this folder, and use the same HTML skeleton and stylesheet as those pairs and as `assets/msstore/md/`.

| URL                                                     | Source                                         | Used by                                                                 |
| ------------------------------------------------------- | ---------------------------------------------- | ----------------------------------------------------------------------- |
| `https://nettrash.me/msstore/familyconnect/privacy.html` | `privacy.html`, edited by hand in this folder. | Partner Center ▸ Privacy policy URL (mandatory for a full-trust app, policy 10.5.1). |
| `https://nettrash.me/msstore/familyconnect/support.html` | `support.html`, edited by hand in this folder. | Partner Center ▸ Support contact info.                                  |

The source of truth for the facts is the `family.connect` repo: `win/store/listing.md` (the listing copy, and its
*Privacy policy — what the page must add for Windows* section), `win/README.md`, and the Windows client itself
(`win/src/FamilyConnect.App/Package.appxmanifest` for the capabilities). The server-side sections — plaintext storage,
retention, reports, account deletion, the assistant — are the same service as the App Store page and must say the same
thing; change a fact there and here in the same sitting.

## What is particular to Windows (keep true)

- **No push service.** A Windows client registers no push device; notifications come from the app's own socket while it
  runs, and say who wrote, never what. Never write "APNs/FCM token" for Windows.
- **Maps are OpenStreetMap** tiles (switch: *Map Previews*), cached on disk for a month; *Open in Maps* opens Apple Maps'
  website in the browser, only on click.
- **Calls run in WebView2**, so the SmartScreen notice Microsoft requires stays on the privacy page.
- **The session token is in the Windows Credential Locker**; history, attachments and the outbox are in `cache.db`,
  `blobs` and `outgoing` under the app's local folder; `diagnostics.log` never holds message text or tokens.
- **Capabilities:** `runFullTrust`, `microphone`, `webcam` (video calls only — no camera capture in chats), `location`
  (one fix, on demand).
- Windows 11 only (21H2, build 22000, is the package's `MinVersion`) — never imply Windows 10.

## Wiring

1. **Trunk copy-dir — already in place.** `frontend/index.html` carries
   `<link data-trunk rel="copy-dir" href="assets/msstore" />` (added for md), so `trunk build --release` writes this folder
   to `dist/msstore/familyconnect/` and nginx serves it at `https://nettrash.me/msstore/familyconnect/`. Nothing else to
   add.
2. **Partner Center.** `win/store/listing.md` still names the App Store pages as the privacy and support URLs; point both
   at the two URLs above once they are deployed and answer 200.
3. **Home-page card — not built.** The Microsoft Store tab (`MsStoreTab` in `frontend/src/components/home.rs`) has only
   md's card. Add Family Connect's when the product ID exists (it comes from Partner Center, not the repo); its Windows
   tile is `family.connect/win/store/images/app-tile-icon-300x300.png`, which would need its own `copy-file` line.
