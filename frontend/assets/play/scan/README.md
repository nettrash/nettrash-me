# `assets/play/scan/` — Scan for Android downloads

Trunk copies this directory verbatim into `dist/play/scan/` (see the
`<link data-trunk rel="copy-dir" href="assets/play">` in
`frontend/index.html`), and nginx then serves it at
<https://nettrash.me/play/scan/>.

The Play tab on the home page links here for two artefacts:

| URL                                              | Source                                                                                |
| ------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `https://nettrash.me/play/scan/privacy.html`     | Static page, edited by hand in this folder.                                            |
| `https://nettrash.me/play/scan/scan-latest.apk`  | Drop-in build artefact. **Not committed.** See "Updating the APK" below.              |

## Updating the APK

Direct APK side-loading on nettrash.me exists because Google's current
"closed testing then production" rollout policy makes it slow to push
each new build through Play. The website link is for users who want the
latest build now and are happy to install from "unknown sources".

To refresh `scan-latest.apk`:

1. From the `Scan.Android/` checkout, build a signed release APK:

   ```bash
   ./gradlew :Scan:assembleRelease
   ```

   Without `keystore.properties` / `SCAN_KEYSTORE_*` env vars this falls
   back to the debug signing config — fine for local sanity checks but
   **do not publish a debug-signed APK to nettrash.me**: users who have
   the Play build installed will not be able to install the debug build
   over it (different signing key), and Android will refuse to update a
   debug APK from a release one later. Always upload the upload-key-signed
   release artefact.

2. Copy the output APK into this folder, renamed to `scan-latest.apk`:

   ```bash
   cp Scan.Android/Scan/build/outputs/apk/release/Scan-release.apk \
      nettrash-me/frontend/assets/play/scan/scan-latest.apk
   ```

   (The exact source path may include a flavour / arch segment depending
   on what's in `build.gradle.kts` at the time — check the assemble
   task's "Wrote …apk" line.)

3. Update `version.properties`-derived strings on the Play tab in
   `frontend/src/components/home.rs` if you want the visible version
   number on the download button to match the new build.

4. Rebuild and redeploy the site:

   ```bash
   cd nettrash-me/frontend && trunk build --release
   ```

   The APK is gitignored (see `.gitignore` in this directory) so it does
   not bloat the repo; it only lives in the deployed `dist/`.

## What the website does for downloads

`nginx.conf` has a dedicated `location ~* \.apk$` block that:

- Forces `Content-Type: application/vnd.android.package-archive` so the
  browser hands the file to the package installer (vs. saving as
  `application/octet-stream`).
- Sends `Content-Disposition: attachment` so the link triggers a
  download dialog.
- Caps the cache at 10 minutes (vs. the 30-day immutable cache the rest
  of the static assets get) so a freshly uploaded APK shows up quickly
  on user devices.

## SHA-256 (optional but encouraged)

After uploading, drop a `scan-latest.apk.sha256` file next to the APK so
power users can verify the artefact. Generate with:

```bash
shasum -a 256 scan-latest.apk > scan-latest.apk.sha256
```
