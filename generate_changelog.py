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
from datetime import datetime


def run(cmd):
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr.strip()}", file=sys.stderr)
        return ""
    return result.stdout.strip()


def fetch_releases():
    raw = run("gh release list --limit 500 --json tagName,name,publishedAt,isDraft,isPrerelease")
    if not raw:
        return []
    releases = json.loads(raw)
    return [r for r in releases if r["tagName"].startswith("mdns-browser-v") and not r["isDraft"]]


def fetch_release_body(tag):
    return run(f'gh release view {tag} --json body -q ".body"')


def fetch_release_date(tag):
    return run(f'gh release view {tag} --json publishedAt -q ".publishedAt"')


def parse_date(published_at):
    if not published_at:
        return "Unknown"
    dt = datetime.fromisoformat(published_at.replace("Z", "+00:00"))
    return dt.strftime("%Y-%m-%d")


def strip_version_prefix(tag):
    return tag.replace("mdns-browser-v", "")


def fetch_merged_prs(prev_tag, current_tag):
    """Fetch PRs merged between two releases with their labels."""
    prev_date = fetch_release_date(prev_tag)
    curr_date = fetch_release_date(current_tag)
    if not prev_date or not curr_date:
        return []

    raw = run(
        f'gh pr list --state merged --base main --limit 500 '
        f'--json number,title,labels,mergedAt '
    )
    if not raw:
        return []

    prs = json.loads(raw)
    return [
        pr for pr in prs
        if pr["mergedAt"] >= prev_date and pr["mergedAt"] <= curr_date
    ]


# CI/internal label patterns - PRs with these labels are not user-facing
CI_LABELS = {"chore", "ci", "dependencies", "documentation", "test", "ignore"}

# User-facing label patterns
USER_FACING_LABELS = {"enhancement", "bug", "bugfix", "security", "feature"}


def has_user_facing_label(pr):
    """Check if PR has user-facing labels."""
    labels = {l["name"].lower() for l in pr.get("labels", [])}
    if labels & USER_FACING_LABELS:
        return True
    if not (labels & CI_LABELS):
        return True
    return False


def classify_pr(pr):
    """Classify PR into Keep a Changelog category based on title and labels."""
    title = pr["title"]
    labels = {l["name"].lower() for l in pr.get("labels", [])}

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
    'ui', 'ux', 'button', 'dialog', 'window', 'theme', 'icon',
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


def generate_full_changelog():
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
            link_defs.append(f"[{version}]: https://github.com/hrzlgnm/mdns-browser/compare/{prev_tag}...{tag}")
        else:
            link_defs.append(f"[{version}]: https://github.com/hrzlgnm/mdns-browser/releases/tag/{tag}")

    lines.append(f"[Unreleased]: https://github.com/hrzlgnm/mdns-browser/compare/mdns-browser-v{versions[0][0]}...HEAD")
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


def insert_section_into_changelog(section):
    """Insert a new section after [Unreleased] in CHANGELOG.md."""
    with open("CHANGELOG.md", "r") as f:
        content = f.read()

    lines = content.split("\n")
    insert_idx = None

    for i, line in enumerate(lines):
        if line.startswith("## [Unreleased]"):
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

    # Update Unreleased comparison link
    tag = None
    for line in section_lines:
        m = re.search(r'\[(\d+\.\d+\.\d+)\]:.*compare/([^.]+' , line)
        if m:
            tag = m.group(2)
            break
    if tag:
        for i, line in enumerate(new_lines):
            if line.startswith("[Unreleased]:"):
                new_lines[i] = f"[Unreleased]: https://github.com/hrzlgnm/mdns-browser/compare/{tag}...HEAD"

    with open("CHANGELOG.md", "w") as f:
        f.write("\n".join(new_lines))

    return True


def main():
    parser = argparse.ArgumentParser(description="Generate CHANGELOG.md")
    parser.add_argument("--tag", help="Release tag for incremental update (e.g., mdns-browser-v1.9.0)")
    parser.add_argument("--repository", default="hrzlgnm/mdns-browser", help="GitHub repository")
    args = parser.parse_args()

    if args.tag:
        section = update_single_release(args.tag, args.repository)
        if section:
            print(section)
        else:
            print("Error: could not generate section", file=sys.stderr)
            sys.exit(1)
    else:
        changelog = generate_full_changelog()
        with open("CHANGELOG.md", "w") as f:
            f.write(changelog)
        print("Changelog written to CHANGELOG.md", file=sys.stderr)


if __name__ == "__main__":
    main()
