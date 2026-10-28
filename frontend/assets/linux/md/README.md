# `assets/linux/md/` — md for Linux pages and downloads

Trunk copies this directory verbatim into `dist/linux/md/` (the
`<link data-trunk rel="copy-dir" href="assets/linux" />` in
`frontend/index.html`), and nginx serves it at <https://nettrash.me/linux/md/>.
md for Linux is in no store: its packages are served from here, beside the two
pages, and the Home page's **Linux** tab (`LinuxTab` in
`frontend/src/components/home.rs`, session key `linux`) links to all of them.

| URL | Source | Used by |
| --- | --- | --- |
| `https://nettrash.me/linux/md/privacy.html` | Static page, edited by hand in this folder. | The app's **Help ▸ Privacy Policy**. |
| `https://nettrash.me/linux/md/support.html` | Static page, edited by hand in this folder. | The app's **Help ▸ md Help** (F1); the `.deb`'s `Homepage:`, the `.rpm`'s `URL:` and the AppStream metadata's homepage and help links (what GNOME Software shows). |
| `https://nettrash.me/linux/md/md_1.6.0_{amd64,arm64}.deb` | Release asset. **Not committed.** | The Linux card; support.html ▸ Download. |
| `https://nettrash.me/linux/md/md-1.6.0-1.fc44.{x86_64,aarch64}.rpm` | Release asset. **Not committed.** | The same. |
| `https://nettrash.me/linux/md/md-1.6.0-linux-{x86_64,aarch64}.tar.gz` | Release asset. **Not committed.** | The same. |
| `https://nettrash.me/linux/md/SHA256SUMS` | Release asset. **Not committed.** | The same. |

The source of truth for the facts on the two pages is the `md.linux` repo:
`PRIVACY.md`, `README.md` and `packaging/tarball/README`. Change a fact there
first, then here in the same sitting — the two must never disagree.

## The packages

The six packages and `SHA256SUMS` are the assets of the GitHub release of the
tag (`.github/workflows/release.yml` in md.linux builds and attaches them). They
are gitignored here (`.gitignore`), so they never reach the repo; they live only
in the deployed `dist/`. To put them in place before a deploy:

```bash
cd nettrash-me/frontend/assets/linux/md
gh release download v1.6.0 --repo nettrash/md.linux --clobber
shasum -a 256 -c SHA256SUMS        # sha256sum -c SHA256SUMS on Linux
cd ../../.. && trunk build --release
```

then deploy `dist/` as the README at the root says. Because the downloads must
not ship without the checksums, check that all seven files are in
`dist/linux/md/` after the build.

nginx (`nginx.conf`, `location ^~ /linux/`) serves them with their own media
types (`application/vnd.debian.binary-package`, `application/x-rpm`,
`application/gzip`, `text/plain` for `SHA256SUMS`), as attachments, with a
ten-minute cache, and answers **404** for a package that is missing — the SPA
fallback never reaches `/linux/`, so a missing `.deb` cannot download as the
site's `index.html`.

## A new version

The file names carry the version, so a release changes these places:

1. Delete the old packages and `SHA256SUMS` from this folder, then download the
   new release's as above.
2. `home.rs` ▸ `LinuxTab`: the six `href` / `download` names, the *Version*
   badge, and the two commands in the small print.
3. `support.html`: the *Applies to* line, the Download table, and the version
   in the install, reinstall and archive commands. `privacy.html`: the
   *Applies to* line, and anything `PRIVACY.md` changed.
4. The `.rpm`'s `fc44` stays while the release workflow builds on Fedora 44.
