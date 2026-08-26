# Modifying GitHub Actions Workflows and Actions

Rules for changing any workflow (`.github/workflows/*.yml`) or action
(`.github/actions/*/action.yml`).

## Pinning

Pin actions to commit SHAs with a `# vN` comment (e.g.,
`@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7`).

## Validation

Run actionlint and fix every finding before committing; re-run it after each
change until clean:

```bash
actionlint .github/workflows/*.yml
```

## Shell selection on Windows runners

Always specify `shell: bash` on steps using bash-specific syntax (such as group
redirection `{ ... } >> "$GITHUB_OUTPUT"`, bash heredocs, etc.) when the
workflow runs on Windows. The default shell on Windows runners is PowerShell,
which doesn't support bash syntax:

```yaml
- name: Step using bash syntax
  shell: bash
  run: |
    {
      echo "body<<BODYEOF"
      some_command
      echo "BODYEOF"
    } >> "$GITHUB_OUTPUT"
```

## Template injection

Never use `${{ }}` expansion inside `run` blocks - it is strictly forbidden.
This applies to all context values (`inputs`, `secrets`, `env`, `needs`,
`steps`, `matrix`, `github`, etc.). Direct interpolation (e.g.,
`${{ inputs.foo }}` inside a `run:` block) creates a template-injection risk.
Always pass values through the `env` block and reference them as shell
variables instead:

```yaml
- name: Example
  env:
    MY_VAR: ${{ inputs.foo }}
  run: gh command "$MY_VAR"
```
