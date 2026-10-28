# Skipping a Failed Release

Releases are fully owned by release-please: merging its release PR creates
the tag and a draft release, the build jobs attach artifacts, and the
publish job flips the draft live. A failed build leaves the draft
unpublished so the release can be rebuilt or skipped. Never create or push
tags by hand — the checksum and build jobs download and check out the tag,
so a missing or hand-pushed tag breaks the release.

## Rebuild (keep the version)

Re-run the failed jobs of the release workflow run; the draft stays in
place and gets published once everything succeeds:

```bash
gh run rerun <run-id> --failed
```

## Skip (abandon the version)

1. Delete the draft release and its tag. Release-please creates the
   tag before the builds start, so it is usually already on the remote:

   ```bash
   gh release delete vX.Y.Z --yes --cleanup-tag
   git tag -d vX.Y.Z # only if it still exists locally
   git ls-remote --tags origin | grep vX.Y.Z # must print nothing
   ```

2. Open a PR that rewinds the release bookkeeping to the last good
   version: `.release-please-manifest.json`, the stale `CHANGELOG.md`
   section(s), and the version fields in `package.json`,
   `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and
   `src-tauri/tauri.conf.json`. Release-please regenerates all of these,
   so a plain rollback is enough.
3. To skip the version number itself (e.g. go from 2.9.5 straight to
   2.9.7), add a `Release-As: 2.9.7` footer. It must land in the
   **commit trailer** of the commit on main: squash merges use whatever
   message is in the merge dialog, so verify the dialog still contains
   the `Release-As` trailer line before confirming. Text that only
   lives in the PR body is dropped as soon as the merger edits the
   message, as happened with #2983.
4. After merge, check that the release-please PR retargeted to the new
   version before merging it.
