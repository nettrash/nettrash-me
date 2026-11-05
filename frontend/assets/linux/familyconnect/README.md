# `assets/linux/familyconnect/` — Family Connect for Linux pages and downloads

Trunk copies this directory verbatim into `dist/linux/familyconnect/` (the
`<link data-trunk rel="copy-dir" href="assets/linux" />` in
`frontend/index.html`), and nginx serves it at
<https://nettrash.me/linux/familyconnect/>. Family Connect for Linux is in no
store: its packages are served from here, beside the two pages, the same way as
md's (`../md/`), and the Home page's **Linux** tab (`LinuxTab` in
`frontend/src/components/home.rs`, session key `linux`) links to all of them.

| URL | Source | Used by |
| --- | --- | --- |
| `https://nettrash.me/linux/familyconnect/support.html` | Static page, edited by hand in this folder. | The `.deb`'s `Homepage:`, the `.rpm`'s `URL:`, the AppStream metadata's homepage and help links (what GNOME Software shows), the man page and the archive's `README`. |
| `https://nettrash.me/linux/familyconnect/privacy.html` | Static page, edited by hand in this folder. | The Linux card. |
| `https://nettrash.me/linux/familyconnect/family-connect_1.3.0_{amd64,arm64}.deb` | Release asset. **Not committed.** | The Linux card; support.html ▸ Download. |
| `https://nettrash.me/linux/familyconnect/family-connect-1.3.0-1.fc44.{x86_64,aarch64}.rpm` | Release asset. **Not committed.** | The same. |
| `https://nettrash.me/linux/familyconnect/family-connect-1.3.0-linux-{x86_64,aarch64}.tar.gz` | Release asset. **Not committed.** | The same. |
| `https://nettrash.me/linux/familyconnect/SHA256SUMS` | Release asset. **Not committed.** | The same. |

The source of truth for the facts on the two pages is the `family.connect` repo:
`linux/packaging/tarball/README`, `linux/docs/packaging.md` and the Linux client
itself. Change a fact there first, then here in the same sitting — the two must
never disagree.

## The packages

The six packages and `SHA256SUMS` are the assets of the GitHub release of the
tag: `.github/workflows/release-linux.yml` in family.connect builds and attaches
them — **but only when `linux/packaging/release-ready` exists at the tag**
(nettrash's switch, 2026-10-29). They are gitignored here (`.gitignore`), so
they never reach this repo; they live only in the deployed `dist/`. To put them
in place before a deploy:

```bash
cd nettrash-me/frontend/assets/linux/familyconnect
gh release download v1.3 --repo nettrash/family.connect --clobber \
    --pattern 'family-connect*' --pattern SHA256SUMS
shasum -a 256 -c SHA256SUMS        # sha256sum -c SHA256SUMS on Linux
cd ../../.. && trunk build --release
```

**While the release is held** (no `release-ready`, so the tag has no Linux
assets), the same six packages are the artifacts of family.connect's `ci.yml`
run, `family-connect-linux-amd64` and `family-connect-linux-arm64` — the 1.3.0
ones deployed first came from the run of ec84568b (2026-11-02), the last commit
that touched `linux/`. Copy the six files in and write the checksums the way the
release workflow does:

```bash
cd nettrash-me/frontend/assets/linux/familyconnect
cp ~/Downloads/family-connect-linux-amd64/* ~/Downloads/family-connect-linux-arm64/* .
shasum -a 256 -- *.deb *.rpm *.tar.gz > SHA256SUMS
```

then deploy `dist/` as the README at the root says. Because the downloads must
not ship without the checksums, check that all seven files are in
`dist/linux/familyconnect/` after the build.

nginx (`nginx.conf`, `location ^~ /linux/`) serves them with their own media
types, as attachments, with a ten-minute cache, and answers **404** for a
package that is missing — the SPA fallback never reaches `/linux/`, so a
missing `.deb` cannot download as the site's `index.html`.

## A new version

The file names carry the version, so a release changes these places:

1. Delete the old packages and `SHA256SUMS` from this folder, then download the
   new release's as above.
2. `home.rs` ▸ `LinuxTab` ▸ the Family Connect card: the six `href` /
   `download` names, the *Version* badge, and the two commands in the small
   print.
3. `support.html`: the *Applies to* line, the Download table, and the version
   in the install and archive commands. `privacy.html`: the *Applies to* line.
4. The `.rpm`'s `fc44` stays while the release workflow builds on Fedora 44.
