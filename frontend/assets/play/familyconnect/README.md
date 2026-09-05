# `assets/play/familyconnect/` — Family Connect for Android downloads

Trunk copies this directory verbatim into `dist/play/familyconnect/` (see
the `<link data-trunk rel="copy-dir" href="assets/play">` in
`frontend/index.html`), and nginx then serves it at
<https://nettrash.me/play/familyconnect/>.

The Play tab on the home page links here for three artefacts:

| URL                                                                    | Source                                                                   |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `https://nettrash.me/play/familyconnect/privacy.html`                  | Static page, edited by hand in this folder.                              |
| `https://nettrash.me/play/familyconnect/support.html`                  | Static page, edited by hand in this folder.                              |
| `https://nettrash.me/play/familyconnect/familyconnect-latest.apk`      | Drop-in build artefact. **Not committed.** See "Updating the APK" below. |

The card also links to <https://play.google.com/apps/testing/me.nettrash.familyconnect>,
which is Play's own opt-in page for the testing track — nothing is served
from this folder for it.

## Which flavour

**`nettrash`, not `standard`** — this is the one difference from the
other four apps in this folder, and getting it wrong ships a broken app.

`android/app/build.gradle.kts` declares two product flavours on the
`distribution` dimension, and they share one `applicationId`:

- `standard` leaves `DEFAULT_SERVER_URL` empty, so first run asks the
  user for a server. It is the source build, and it has no
  `google-services.json` — **no push at all**, which for a messenger
  means no message notifications and no incoming calls.
- `nettrash` ships pre-pointed at `https://fc.nettrash.me` and carries
  `app/src/nettrash/google-services.json`, so it is the flavour with
  FCM push. This is what goes to Play, and therefore what belongs here:
  an APK offered next to a "Get it on Google Play" button has to be
  installable *over* the Play build, and a user who side-loads
  `standard` would get a silent app pointed at nothing.

## Updating the APK

Direct APK side-loading on nettrash.me exists because Google's current
"closed testing then production" rollout policy makes it slow to push
each new build through Play. The website link is for users who want the
latest build now and are happy to install from "unknown sources".

To refresh `familyconnect-latest.apk`:

1. From the `family.connect/android/` checkout, build a signed release
   APK. The Android toolchain here needs the Android Studio JBR — Gradle
   9.6 / AGP 9.4 reject the Homebrew JDK 25:

   ```bash
   cd family.connect/android
   JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home" \
     ./gradlew :app:assembleNettrashRelease
   ```

   Signing is read from `keystore.properties` next to the root build
   file, or from `FAMILYCONNECT_KEYSTORE_PATH` /
   `FAMILYCONNECT_KEYSTORE_PASSWORD` / `FAMILYCONNECT_KEY_ALIAS` /
   `FAMILYCONNECT_KEY_PASSWORD`. With neither, the build still
   **succeeds** and quietly emits `app-nettrash-release-unsigned.apk` —
   enough to check R8, useless to publish. Never put an unsigned or
   debug-signed APK here: a user with the Play build installed cannot
   install a differently-keyed one over it.

2. Copy the output APK into this folder, renamed:

   ```bash
   cp family.connect/android/app/build/outputs/apk/nettrash/release/app-nettrash-release.apk \
      nettrash-me/frontend/assets/play/familyconnect/familyconnect-latest.apk
   ```

3. Note that the release build **bumps `android/version.properties`**,
   which is a tracked file — commit it with the release, or pass
   `-PnoBump` when you are only testing the build.

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

After uploading, drop a `familyconnect-latest.apk.sha256` file next to
the APK so power users can verify the artefact. Generate with:

```bash
shasum -a 256 familyconnect-latest.apk > familyconnect-latest.apk.sha256
```
