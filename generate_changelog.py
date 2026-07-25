#!/usr/bin/env python3
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT-0

"""Generate or update CHANGELOG.md in Keep a Changelog format.

Usage:
  python3 generate_changelog.py                     # Full regeneration
  python3 generate_changelog.py --tag mdns-browser-v1.9.0  # Incremental update
"""

import json
import subprocess
import sys
import re
import argparse
from datetime import datetime, timezone


def run(cmd):
    """Run a command as argument list with shell=False."""
    if isinstance(cmd, str):
        cmd = cmd.split()
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(f"Command failed: {' '.join(cmd)}\n{result.stderr.strip()}")
    return result.stdout.strip()


def run_optional(cmd):
    """Run a command, returning empty string on failure."""
    try:
        return run(cmd)
    except RuntimeError:
        return ""


def fetch_releases():
    raw = run(["gh", "release", "list", "--limit", "500",
               "--json", "tagName,name,publishedAt,isDraft,isPrerelease"])
    if not raw:
        return []
    releases = json.loads(raw)
    return [
        r for r in releases
        if r["tagName"].startswith("mdns-browser-v")
        and not r["isDraft"]
        and not r["isPrerelease"]
    ]


def fetch_release_body(tag):
    return run_optional(["gh", "release", "view", tag,
                         "--json", "body", "-q", ".body"])


def fetch_release_date(tag):
    return run_optional(["gh", "release", "view", tag,
                         "--json", "publishedAt", "-q", ".publishedAt"])


def parse_date(published_at):
    if not published_at:
        return "Unknown"
    dt = datetime.fromisoformat(published_at.replace("Z", "+00:00"))
    return dt.strftime("%Y-%m-%d")


def parse_date_to_aware(date_str):
    """Parse date string to timezone-aware datetime for comparison."""
    if not date_str:
        return None
    date_str = date_str.strip()
    try:
        if "T" in date_str:
            dt = datetime.fromisoformat(date_str.replace("Z", "+00:00"))
        else:
            dt = datetime.strptime(date_str[:10], "%Y-%m-%d").replace(tzinfo=timezone.utc)
        return dt
    except ValueError:
        return None


def strip_version_prefix(tag):
    """Strip version prefix from tag."""
    for prefix in ["mdns-browser-v", "webkit2gtk-nvidia-quirk-v"]:
        if tag.startswith(prefix):
            return tag[len(prefix):]
    return tag


def is_ci_only_pr(files):
    """Check if PR only changes CI/dependency files."""
    if not files:
        return False
    return all(
        f.startswith(".github/") or f == "Cargo.lock"
        for f in files
    )


def update_pr_labels_if_needed(pr):
    """Add chore label to PR if it only has CI/dependency changes."""
    files = pr.get("files", [])
    if not is_ci_only_pr(files):
        return

    labels = {label["name"].lower() for label in pr.get("labels", [])}
    if "chore" in labels:
        return

    pr_number = pr["number"]
    print(f"  Adding chore label to PR #{pr_number} (CI-only changes)", file=sys.stderr)
    run_optional(["gh", "pr", "edit", str(pr_number), "--add-label", "chore"])


def fetch_merged_prs(prev_tag, current_tag):
    """Fetch PRs merged between two tags with their labels and files."""
    prev_date_str = fetch_release_date(prev_tag)
    curr_date_str = fetch_release_date(current_tag)

    if not prev_date_str or not curr_date_str:
        if prev_tag:
            prev_date_str = run_optional(["git", "log", "-1", "--format=%ai", prev_tag])
        curr_date_str = run_optional(["git", "log", "-1", "--format=%ai", current_tag])

    prev_date = parse_date_to_aware(prev_date_str)
    curr_date = parse_date_to_aware(curr_date_str)

    if not prev_date or not curr_date:
        return []

    raw = run_optional(
        ["gh", "pr", "list", "--state", "merged", "--base", "main",
         "--limit", "500", "--json", "number,title,labels,mergedAt,files"]
    )
    if not raw:
        return []

    prs = json.loads(raw)
    return [
        pr for pr in prs
        if parse_date_to_aware(pr["mergedAt"]) is not None
        and prev_date <= parse_date_to_aware(pr["mergedAt"]) <= curr_date
    ]


# CI/internal label patterns - PRs with these labels are not user-facing
CI_LABELS = {"chore", "ci", "dependencies", "documentation", "test", "ignore"}

# User-facing label patterns
USER_FACING_LABELS = {"enhancement", "bug", "bugfix", "security", "feature"}


def has_user_facing_label(pr):
    """Check if PR has user-facing labels or changes."""
    labels = {label["name"].lower() for label in pr.get("labels", [])}
    if labels & USER_FACING_LABELS:
        return True
    if labels & CI_LABELS:
        return False
    files = pr.get("files", [])
    if not files:
        return True
    all_ci = all(
        f.startswith(".github/") or f == "Cargo.lock"
        for f in files
    )
    if all_ci:
        return False
    return True


def classify_pr(pr):
    """Classify PR into Keep a Changelog category based on title and labels."""
    title = pr["title"]
    labels = {label["name"].lower() for label in pr.get("labels", [])}

    if labels & {"security"} or "GHSA-" in title:
        return "Security"
    if title.startswith("feat"):
        return "Added"
    if title.startswith("fix"):
        return "Fixed"
    if title.startswith("refactor"):
        return "Changed"
    if title.startswith("breaking"):
        return "Changed"
    if title.startswith(("doc", "docs")):
        return "Changed"
    if title.startswith("chore"):
        return "Changed"
    if title.startswith("bump"):
        return "Changed"
    return None


# Content-based filtering for full regeneration mode
CI_CONTENT_KEYWORDS = [
    'workflow', 'release-drafter', 'tauri-action', 'gh release',
    'docker', ' CI', 'ubuntu builder', 'arch-aur', 'void package',
    'artifact', 'checksum', 'attest', 'sbom', 'bundl',
    'signing', 'deploy', 'actionlint', 'clippy', 'rustfmt',
    'leptosfmt', 'sccache', 'nextest', 'cargo-edit', 'cargo install',
    'trunk', 'winget', 'homebrew', 'aur', 'renovate', 'coderabbit',
    'env var', 'jq via', 'environment variable', 'release body',
    'release notes', 'release drafter', 'workflow_call', 'workflow_dispatch',
    'ruleset', 'ssh key', 'signing key', 'deploy key', 'GITHUB_TOKEN',
    'debug symbol', 'crash report', 'rerun', 're-run', 're run',
]

CI_PREFIX_PATTERNS = [
    r'^feat\(ci[:\)]', r'^fix\(ci[:\)]', r'^chore\(ci[:\)]',
    r'^chore\(renovate', r'^chore\(deps[:\)]',
    r'^chore\(arch-aur', r'^chore\(void[:\)]',
    r'^chore\(winget', r'^chore\(homebrew',
    r'^chore\(sbom', r'^chore\(signing',
    r'^chore\(bundler', r'^chore\(publish',
    r'^chore\(version', r'^chore\(android',
    r'^ci:', r'^test:', r'^chore\(test',
    r'^performance\(ci',
]

USER_FACING_OVERRIDES = [
    'nvidia', 'webkit2gtk', 'service', 'mdns', 'browse', 'listen',
    'ui', 'ux', 'button', 'dialog', 'theme', 'icon',
    'cli', 'command line', 'option', 'argument', 'splash',
    'filter', 'sort', 'copy', 'clipboard', 'auto-update', 'updater',
    'mobile', 'android', 'ios', 'platform', 'error handling',
    'graceful', 'disposal', 'memo', 'signal', 'reactive', 'store',
    'state', 'dead', 'alive', 'ipv4', 'ipv6', 'ip address',
    'network', 'interface', 'gpu', 'dma', 'render', 'wayland', 'x11',
]


def is_ci_change_content(entry):
    """Check if entry is CI/internal by content (for full regeneration)."""
    for pattern in CI_PREFIX_PATTERNS:
        if re.search(pattern, entry, re.IGNORECASE):
            return True
    entry_lower = entry.lower()
    for keyword in CI_CONTENT_KEYWORDS:
        if keyword.lower() in entry_lower:
            for override in USER_FACING_OVERRIDES:
                if override.lower() in entry_lower:
                    return False
            return True
    return False


def is_user_facing_content(entry):
    """Check if entry is user-facing by content (for full regeneration)."""
    if is_ci_change_content(entry):
        return False
    if entry.startswith(('feat:', 'fix:', 'refactor:', 'security:', 'GHSA-', 'breaking:')):
        return True
    return False


def extract_pr_numbers(entry):
    """Extract all PR numbers from entry."""
    return [int(m) for m in re.findall(r'#(\d+)', entry)]


def fetch_pr_files_batch(pr_numbers):
    """Fetch files for multiple PRs."""
    if not pr_numbers:
        return {}
    result = {}
    for pr_num in pr_numbers:
        raw = run_optional(["gh", "pr", "view", str(pr_num), "--json", "files"])
        if raw:
            data = json.loads(raw)
            result[pr_num] = [f["path"] for f in data.get("files", [])]
    return result


def classify_entry_content(entry):
    """Classify entry into category by content."""
    if entry.startswith('feat:'):
        return "Added"
    elif entry.startswith('fix:'):
        return "Fixed"
    elif entry.startswith('refactor:'):
        return "Changed"
    elif entry.startswith(('security:', 'GHSA-')):
        return "Security"
    elif entry.startswith('breaking:'):
        return "Changed"
    return None


def parse_release_body(body):
    """Parse release body into categories (content-based filtering)."""
    categories = {"Added": [], "Changed": [], "Fixed": [], "Security": []}
    if not body:
        return categories

    lines = body.split("\n")
    in_collapsed = False
    entries_with_prs = []

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("## What's Changed") or stripped.startswith("### Full Changelog"):
            continue
        if stripped.startswith("[mdns-browser-v"):
            continue
        if stripped.startswith("<details>"):
            in_collapsed = True
            continue
        if stripped.startswith("</details>") or stripped.startswith("<summary>"):
            in_collapsed = False
            continue
        if stripped.startswith("### :arrow_up: Dependency Updates"):
            continue
        if stripped.startswith("###"):
            continue

        if stripped.startswith("- ") and not in_collapsed:
            entry = stripped[2:]
            entry = re.sub(r'\s*@\S+', '', entry)
            entry = re.sub(r'\s*@\[.*?\]', '', entry)
            entry = entry.strip()
            if not entry or entry.startswith("["):
                continue
            if re.match(r'^chore\(version\):', entry, re.IGNORECASE):
                continue
            if 'bump version' in entry.lower():
                continue
            if not is_user_facing_content(entry):
                continue
            pr_nums = extract_pr_numbers(entry)
            entries_with_prs.append((entry, pr_nums))

    # Batch fetch PR files and filter CI-only PRs
    all_pr_nums = set()
    for _, pr_nums in entries_with_prs:
        all_pr_nums.update(pr_nums)
    pr_files = fetch_pr_files_batch(list(all_pr_nums))

    for entry, pr_nums in entries_with_prs:
        # Skip if ALL referenced PRs are CI-only
        if pr_nums and all(
            is_ci_only_pr(pr_files.get(p, []))
            for p in pr_nums
            if p in pr_files
        ):
            continue
        cat = classify_entry_content(entry)
        if cat:
            categories[cat].append(entry)

    return categories


def build_changelog_section(version, date, categories, prev_tag=None, tag=None, repository=None, note=None):
    """Build a changelog section for a single version."""
    has_changes = any(len(v) > 0 for v in categories.values())

    section = f"## [{version}] - {date}\n\n"

    if has_changes:
        for cat_name in ["Added", "Changed", "Fixed", "Security"]:
            entries = categories[cat_name]
            if entries:
                section += f"### {cat_name}\n\n"
                for entry in entries:
                    section += f"- {entry}\n"
                section += "\n"
    elif note:
        section += f"{note}\n\n"

    if prev_tag and tag and repository:
        section += f"[{version}]: https://github.com/{repository}/compare/{prev_tag}...{tag}\n"

    return section


def has_dep_updates(body):
    """Check if release body contains dependency update entries."""
    for line in body.split("\n"):
        if "Dependency Updates" in line or "chore(deps)" in line.lower():
            return True
    return False


def generate_full_changelog(repository="hrzlgnm/mdns-browser"):
    """Full regeneration of CHANGELOG.md."""
    print("Fetching releases...", file=sys.stderr)
    releases = fetch_releases()
    print(f"Found {len(releases)} releases", file=sys.stderr)

    lines = [
        "# Changelog\n",
        "All notable changes to this project will be documented in this file.\n",
        "The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),",
        "and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n",
        "## [Unreleased]\n",
        "### Added\n",
        "### Changed\n",
        "### Fixed\n",
    ]

    versions = []
    for release in releases:
        tag = release["tagName"]
        version = strip_version_prefix(tag)
        date = parse_date(release.get("publishedAt"))
        body = fetch_release_body(tag)
        categories = parse_release_body(body)
        note = None
        if not any(len(v) > 0 for v in categories.values()):
            if has_dep_updates(body):
                note = "Only dependencies were updated."
            else:
                note = "No user-facing changes."
        versions.append((version, date, categories, tag, note))

    link_defs = []
    for i, (version, date, categories, tag, note) in enumerate(versions):
        section = build_changelog_section(version, date, categories, note=note)
        if section:
            lines.append(section.rstrip())
            lines.append("")
        if i + 1 < len(versions):
            prev_tag = versions[i + 1][3]
            link_defs.append(f"[{version}]: https://github.com/{repository}/compare/{prev_tag}...{tag}")
        else:
            link_defs.append(f"[{version}]: https://github.com/{repository}/releases/tag/{tag}")

    if versions:
        lines.append(f"[Unreleased]: https://github.com/{repository}/compare/{versions[0][3]}...HEAD")
    for link in link_defs:
        lines.append(link)

    return "\n".join(lines)


def update_single_release(tag, repository):
    """Incremental update for a single release using PR labels."""
    version = strip_version_prefix(tag)
    date = parse_date(fetch_release_date(tag))

    print(f"Processing {tag} ({date})", file=sys.stderr)

    # Find previous release
    releases = fetch_releases()
    tag_names = [r["tagName"] for r in releases]
    try:
        idx = tag_names.index(tag)
        if idx + 1 >= len(tag_names):
            print("No previous release found", file=sys.stderr)
            return None
        prev_tag = tag_names[idx + 1]
    except ValueError:
        print(f"Tag {tag} not found", file=sys.stderr)
        return None

    # Fetch PRs merged between releases
    prs = fetch_merged_prs(prev_tag, tag)
    print(f"Found {len(prs)} PRs between {prev_tag} and {tag}", file=sys.stderr)

    # Update labels for CI-only PRs
    for pr in prs:
        update_pr_labels_if_needed(pr)

    categories = {"Added": [], "Changed": [], "Fixed": [], "Security": []}

    for pr in prs:
        if not has_user_facing_label(pr):
            continue
        cat = classify_pr(pr)
        if cat:
            entry = f"{pr['title']} #{pr['number']}"
            categories[cat].append(entry)

    # Also check release body for security entries
    body = fetch_release_body(tag)
    for line in body.split("\n"):
        stripped = line.strip()
        if stripped.startswith("- GHSA-") or stripped.startswith("- security:"):
            entry = stripped[2:].strip()
            if entry not in categories["Security"]:
                categories["Security"].append(entry)

    note = None
    if not any(len(v) > 0 for v in categories.values()):
        if has_dep_updates(body):
            note = "Only dependencies were updated."
        else:
            note = "No user-facing changes."

    return build_changelog_section(version, date, categories, prev_tag, tag, repository, note=note)


def insert_section_into_changelog(section, repository="hrzlgnm/mdns-browser"):
    """Insert a new section after [Unreleased] in CHANGELOG.md."""
    with open("CHANGELOG.md", "r") as f:
        content = f.read()

    lines = content.split("\n")
    insert_idx = None

    for i, line in enumerate(lines):
        if line.startswith("## [Unreleased]"):
            insert_idx = i + 1
            for j in range(i + 1, len(lines)):
                if lines[j].startswith("## ["):
                    insert_idx = j
                    break
            break

    if insert_idx is None:
        print("ERROR: Could not find [Unreleased] section", file=sys.stderr)
        return False

    section_lines = section.rstrip().split("\n")
    new_lines = lines[:insert_idx] + section_lines + [""] + lines[insert_idx:]

    # Update Unreleased comparison link from the new section
    for line in section_lines:
        m = re.search(r'\[([0-9]+\.[0-9]+\.[0-9]+)\]:.*compare/([^)]+)\.\.\.([^)]+)', line)
        if m:
            new_tag = m.group(3)
            for idx, ln in enumerate(new_lines):
                if ln.startswith("[Unreleased]:"):
                    new_lines[idx] = f"[Unreleased]: https://github.com/{repository}/compare/{new_tag}...HEAD"
                    break
            break

    with open("CHANGELOG.md", "w") as f:
        f.write("\n".join(new_lines))

    return True


def fetch_tags(prefix):
    """Fetch git tags with a specific prefix."""
    raw = run_optional(["git", "tag", "--sort=-version:refname"])
    if not raw:
        return []
    tags = raw.split("\n")
    return [t for t in tags if t.startswith(prefix)]


def generate_crate_changelog(crate_name, crate_path, repository):
    """Generate a separate changelog for a workspace crate."""
    crate_dir = crate_path.rstrip("/")
    tag_prefix = f"{crate_name}-v"

    print(f"Fetching tags for {crate_name}...", file=sys.stderr)
    tags = fetch_tags(tag_prefix)
    print(f"Found {len(tags)} tags", file=sys.stderr)

    if not tags:
        return "# Changelog\n\nNo releases yet.\n"

    lines = [
        "# Changelog\n",
        f"All notable changes to `{crate_name}` will be documented in this file.\n",
        "The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).",
        "This changelog is auto-generated from commits that modify this crate.\n",
        "## [Unreleased]\n",
        "### Added\n",
        "### Changed\n",
        "### Fixed\n",
    ]

    versions = []
    for tag in tags:
        version = strip_version_prefix(tag)
        date_raw = run_optional(["git", "log", "-1", "--format=%ai", tag])
        date = parse_date_from_git(date_raw)

        tag_idx = tags.index(tag)
        prev_tag = tags[tag_idx + 1] if tag_idx + 1 < len(tags) else None

        # Get commits between tags
        if prev_tag:
            log = run_optional(["git", "log", "--oneline", f"{prev_tag}..{tag}", "--", f"{crate_dir}/"])
        else:
            log = run_optional(["git", "log", "--oneline", tag, "--", f"{crate_dir}/"])

        categories = {"Added": [], "Changed": [], "Fixed": [], "Security": []}
        for line in log.split("\n"):
            if not line.strip():
                continue
            parts = line.split(" ", 1)
            if len(parts) < 2:
                continue
            _commit_hash, message = parts
            # Skip version bump commits
            if re.match(r'^bump\s', message, re.IGNORECASE):
                continue
            # Skip revert of bump commits
            if re.match(r'^revert\s.*bump', message, re.IGNORECASE):
                continue

            cat = classify_commit_message(message)
            if cat:
                categories[cat].append(message)

        versions.append((version, date, categories, tag, None, prev_tag))

    link_defs = []
    for version, date, categories, tag, note, prev_tag in versions:
        section = build_changelog_section(version, date, categories, note=note)
        if section:
            lines.append(section.rstrip())
            lines.append("")
        if prev_tag:
            link_defs.append(f"[{version}]: https://github.com/{repository}/compare/{prev_tag}...{tag}")
        else:
            link_defs.append(f"[{version}]: https://github.com/{repository}/releases/tag/{tag}")

    if versions:
        lines.append(f"[Unreleased]: https://github.com/{repository}/compare/{versions[0][3]}...HEAD")
    for link in link_defs:
        lines.append(link)

    return "\n".join(lines)


def classify_commit_message(message):
    """Classify a commit message into a changelog category."""
    if message.startswith(("feat", "add")):
        return "Added"
    if message.startswith("fix"):
        return "Fixed"
    if message.startswith(("refactor", "doc", "docs", "chore")):
        return "Changed"
    if message.startswith(("security", "GHSA-")):
        return "Security"
    if message.startswith("breaking"):
        return "Changed"
    return None


def parse_date_from_git(date_raw):
    """Parse git date string to YYYY-MM-DD format."""
    if not date_raw:
        return "Unknown"
    try:
        dt = datetime.strptime(date_raw.strip()[:10], "%Y-%m-%d")
        return dt.strftime("%Y-%m-%d")
    except ValueError:
        return "Unknown"


def main():
    parser = argparse.ArgumentParser(description="Generate CHANGELOG.md")
    parser.add_argument("--tag", help="Release tag for incremental update (e.g., mdns-browser-v1.9.0)")
    parser.add_argument("--repository", default="hrzlgnm/mdns-browser", help="GitHub repository")
    parser.add_argument("--crate", help="Generate changelog for a specific crate (e.g., webkit2gtk-nvidia-quirk)")
    parser.add_argument("--crate-path", help="Path to crate directory (e.g., crates/webkit2gtk-nvidia-quirk)")
    args = parser.parse_args()

    if bool(args.crate) != bool(args.crate_path):
        parser.error("--crate and --crate-path must be used together")

    try:
        if args.tag:
            section = update_single_release(args.tag, args.repository)
            if section:
                print(section)
            else:
                print("Error: could not generate section", file=sys.stderr)
                sys.exit(1)
        elif args.crate and args.crate_path:
            changelog = generate_crate_changelog(args.crate, args.crate_path, args.repository)
            output_path = f"{args.crate_path}/CHANGELOG.md"
            with open(output_path, "w") as f:
                f.write(changelog)
            print(f"Changelog written to {output_path}", file=sys.stderr)
        else:
            changelog = generate_full_changelog(args.repository)
            with open("CHANGELOG.md", "w") as f:
                f.write(changelog)
            print("Changelog written to CHANGELOG.md", file=sys.stderr)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
