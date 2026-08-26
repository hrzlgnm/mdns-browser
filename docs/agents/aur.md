# AUR Packaging Tests

```bash
# Test source and binary packages from the repository root
~/.local/bin/test-aur-local --variant=both

# Test one package variant
~/.local/bin/test-aur-local --variant=source
~/.local/bin/test-aur-local --variant=bin
```

- Use `--no-build` only for generator and lint smoke tests; omit it to test package creation and installation.
- Use `--no-install` to skip installing the `-bin` package, and `--no-cleanup` or `--keep-dir=<path>` to retain build artifacts for debugging.
