# `assets/.well-known/` — App-link verification endpoints

Trunk copies this folder verbatim into `dist/.well-known/` (see the
new `<link data-trunk rel="copy-dir" href="assets/.well-known">` in
`frontend/index.html`), and nginx serves it from
<https://nettrash.me/.well-known/> with the right MIME types and a
no-cache policy (see `nettrash-me/nginx.conf`).

Two payloads:

## `apple-app-site-association`

iOS Universal Links manifest. Apple's `swcd` daemon fetches this on
first install (and re-fetches periodically) to authorise the app
to claim deep links. **Must be served as `application/json` over
HTTPS, with no `.json` extension and no redirects.**

| Field         | Value                                  |
| ------------- | -------------------------------------- |
| `appIDs`      | `V4WM2SJ8Q9.me.nettrash.Scan` — `<DEVELOPMENT_TEAM>.<bundleId>`. Found in Apple Developer → Membership / `agvtool`. |
| `components`  | `/scan/*` is in-scope; `/appstore/scan/*` is excluded so the privacy page keeps opening in Safari. |

To rotate the Team ID (e.g. moving the app between developer
accounts), update both this file *and* `Scan.entitlements` in the
iOS project, and rebuild + redeploy the website. Devices may take a
few minutes to a few hours to pick up the new manifest.

## `assetlinks.json`

Android App Links manifest. The Play Services intent verifier
(`autoVerify="true"` in `AndroidManifest.xml`) fetches this on
install and any version-name bump, and only flips an
`https://nettrash.me/scan/*` filter from "browsable" to "verified"
when **every** key in `sha256_cert_fingerprints` round-trips.

Two fingerprints are listed:

1. **Upload key** (`scan-upload.jks`) — already filled in. Used by
   APKs side-loaded from <https://nettrash.me/play/scan/scan-latest.apk>.
   The fingerprint was extracted via:

   ```bash
   keytool -list -v -keystore scan-upload.jks -alias scan-upload \
     -storepass <password> | grep SHA256
   ```

2. **Play App Signing key** — placeholder. Pull the SHA-256 from
   Google Play Console → *Setup* → *App integrity* → *App signing
   key certificate*, substitute it in, redeploy. Until that's done,
   Play installs will fall back to the disambiguation chooser
   instead of opening Scan automatically — links still work, just
   not as smoothly.

## Caching

`nettrash.me/nginx.conf` adds a `location ^~ /.well-known/` block
that serves these files with:

- `Content-Type: application/json` (verifiers refuse to parse other
  types).
- `Cache-Control: no-store` — both Apple and Google cache the
  manifests aggressively, but a propagation delay measured in hours
  is much friendlier to the deploy cycle than one measured in days.
- No `try_files` fallback to `index.html` — the verifiers expect a
  hard 404 if the file is missing, not the SPA shell.
