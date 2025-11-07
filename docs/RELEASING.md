# Release Process

This document describes how to create a new release of asyncapi-rust.

## Prerequisites

Install git-cliff:
```bash
cargo install git-cliff
```

## Quick Release (Using Script)

The easiest way to create a release:

```bash
./scripts/release.sh
```

This script will:
1. Check you're on main branch with no uncommitted changes
2. Analyze commits and suggest next version
3. Show unreleased changes
4. Update all Cargo.toml files with new version
5. Generate/update CHANGELOG.md
6. Commit changes and create git tag
7. Provide next steps for publishing

**Then follow the printed instructions to:**
- Push to GitHub
- Publish crates to crates.io
- Create GitHub release

## Manual Release Process

If you prefer manual control:

### 1. Determine Next Version

```bash
# See suggested version based on commits
git-cliff --bump --unreleased

# View unreleased changes
git-cliff --unreleased
```

### 2. Update Version Numbers

Edit these files:
- `Cargo.toml` - workspace version
- `asyncapi-rust/Cargo.toml` - dependency versions

```toml
# Cargo.toml
[workspace.package]
version = "0.2.0"  # Update this

# asyncapi-rust/Cargo.toml
[dependencies]
asyncapi-rust-codegen = { version = "0.2.0", ... }  # Update this
asyncapi-rust-models = { version = "0.2.0", ... }   # Update this
```

### 3. Generate Changelog

```bash
# First release - generate full changelog
git-cliff -o CHANGELOG.md

# Subsequent releases - prepend new release
git-cliff --unreleased --tag v0.2.0 --prepend CHANGELOG.md
```

### 4. Commit and Tag

```bash
git add Cargo.toml asyncapi-rust/Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): prepare for v0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
```

### 5. Push to GitHub

```bash
git push origin main
git push origin v0.2.0
```

### 6. Publish to crates.io

**Important:** Publish in dependency order with delays for crates.io indexing.

```bash
# 1. Publish models (no dependencies)
cd asyncapi-rust-models
cargo publish
cd ..

# 2. Wait for indexing
sleep 30

# 3. Publish codegen (depends on models)
cd asyncapi-rust-codegen
cargo publish
cd ..

# 4. Wait for indexing
sleep 30

# 5. Publish main crate (depends on both)
cd asyncapi-rust
cargo publish
cd ..
```

### 7. Create GitHub Release

```bash
# Generate release notes from latest tag
git-cliff --latest --strip header > release-notes.txt

# Create release using GitHub CLI
gh release create v0.2.0 \
  --title "v0.2.0" \
  --notes-file release-notes.txt

# Or copy release-notes.txt and create release via GitHub web UI
```

## Version Bump Rules (Pre-1.0)

Following semantic versioning for pre-1.0 releases:

- **Breaking changes** → Bump MINOR (e.g., 0.1.0 → 0.2.0)
- **Features** → Bump MINOR (e.g., 0.1.0 → 0.2.0)
- **Bug fixes** → Bump PATCH (e.g., 0.1.0 → 0.1.1)
- **Docs, refactor, test, chore** → Bump PATCH

## Conventional Commit Format

Use conventional commits for automatic changelog generation:

```
feat(scope): add new feature
fix(scope): fix bug
docs(scope): update documentation
refactor(scope): refactor code
test(scope): add tests
chore(scope): miscellaneous
perf(scope): performance improvement

# Breaking change
feat(api)!: change API signature

# Or with body
feat(api): change API signature

BREAKING CHANGE: API signature changed
```

## Common Tasks

**Preview next release:**
```bash
git-cliff --bump --unreleased
```

**View unreleased changes:**
```bash
git-cliff --unreleased
```

**Regenerate entire changelog:**
```bash
git-cliff -o CHANGELOG.md
```

**Generate release notes for latest tag:**
```bash
git-cliff --latest --strip header
```

**Check what version bump is needed:**
```bash
git-cliff --bump --unreleased | grep -oE '[0-9]+\.[0-9]+\.[0-9]+'
```

## Troubleshooting

**"Could not determine next version"**
- No new commits since last tag
- Or commits don't follow conventional format

**"Uncommitted changes exist"**
- Commit or stash changes before running release script

**"Must be on main branch"**
- Checkout main: `git checkout main`

**Crates.io publish fails with "not found"**
- Wait longer between publishes (try 60s instead of 30s)
- Check crates.io to verify previous crate appeared

**Want to undo a release (before pushing):**
```bash
# Remove commit and tag
git reset --hard HEAD~1
git tag -d v0.2.0

# Restore files
git checkout Cargo.toml asyncapi-rust/Cargo.toml Cargo.lock CHANGELOG.md
```

## Release Checklist

- [ ] All tests passing: `cargo test`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Examples run: `cargo run --example message_integration`
- [ ] On main branch with no uncommitted changes
- [ ] Version numbers updated in all Cargo.toml files
- [ ] CHANGELOG.md generated and reviewed
- [ ] Git tag created
- [ ] Pushed to GitHub
- [ ] All three crates published to crates.io
- [ ] GitHub release created
- [ ] Docs.rs builds verified (check https://docs.rs/asyncapi-rust)

## See Also

- [git-cliff Documentation](https://git-cliff.org/docs)
- [Conventional Commits](https://www.conventionalcommits.org)
- [Semantic Versioning](https://semver.org)
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
