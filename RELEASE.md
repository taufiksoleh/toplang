# Release Process

This document describes how to create a new release of TopLang.

## Overview

TopLang uses an automated release pipeline via GitHub Actions. When a new version tag is pushed, the CI/CD pipeline automatically:

1. Builds release binaries for Linux, macOS, and Windows
2. Strips binaries to reduce size
3. Creates a GitHub Release
4. Uploads the binaries as release assets

## Release Checklist

### 1. Prepare the Release

1. **Update version number** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Update this
   ```

2. **Update CHANGELOG.md**:
   - Add a new version section with the release date
   - Document all changes under appropriate categories:
     - `Added` - New features
     - `Changed` - Changes in existing functionality
     - `Deprecated` - Soon-to-be removed features
     - `Removed` - Removed features
     - `Fixed` - Bug fixes
     - `Security` - Security fixes

3. **Test the changes**:
   ```bash
   # Run all tests
   cargo test

   # Run clippy
   cargo clippy -- -D warnings

   # Build release binary
   cargo build --release

   # Test the binary
   ./target/release/top examples/hello.top
   ```

4. **Commit the changes**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Prepare v0.2.0 release"
   git push
   ```

### 2. Create and Push the Tag

1. **Create an annotated tag**:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   ```

2. **Push the tag to trigger the release pipeline**:
   ```bash
   git push origin v0.2.0
   ```

   This will automatically trigger the `build-artifacts` job in `.github/workflows/ci.yml`.

### 3. Monitor the Release

1. Go to the **Actions** tab on GitHub
2. Watch the workflow run for your tag
3. The `build-artifacts` job will:
   - Build for `x86_64-unknown-linux-gnu` (Linux)
   - Build for `x86_64-pc-windows-msvc` (Windows)
   - Build for `x86_64-apple-darwin` (macOS)
   - Create a GitHub Release
   - Upload binaries as release assets

### 4. Finalize the Release

1. Go to the **Releases** page on GitHub
2. Find the newly created release (it should be marked as "Latest")
3. Edit the release description if needed:
   - Add highlights from CHANGELOG.md
   - Add installation instructions
   - Add upgrade notes if applicable
4. Verify that all three binaries are attached:
   - `toplang-linux-x64`
   - `toplang-windows-x64.exe`
   - `toplang-macos-x64`

## Release Artifacts

The automated pipeline creates the following artifacts:

| Platform | Architecture | Binary Name | Size (approx) |
|----------|-------------|-------------|---------------|
| Linux | x86_64 | `toplang-linux-x64` | ~2-3 MB |
| macOS | x86_64 | `toplang-macos-x64` | ~2-3 MB |
| Windows | x86_64 | `toplang-windows-x64.exe` | ~2-3 MB |

All binaries are:
- Statically linked (no external dependencies)
- Stripped of debug symbols
- Optimized with LTO (Link-Time Optimization)
- Built with `opt-level = 3`

## Versioning

TopLang follows [Semantic Versioning](https://semver.org/) (SemVer):

- **MAJOR** version (X.0.0): Incompatible API/language changes
- **MINOR** version (0.X.0): New features (backward-compatible)
- **PATCH** version (0.0.X): Bug fixes (backward-compatible)

### Examples:

- `0.1.0` → `0.2.0`: Added new features, removed unused code
- `0.2.0` → `0.2.1`: Bug fixes only
- `0.2.1` → `1.0.0`: First stable release with breaking changes

## Troubleshooting

### Tag already exists
If you need to recreate a tag:
```bash
# Delete local tag
git tag -d v0.2.0

# Delete remote tag
git push origin :refs/tags/v0.2.0

# Create new tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### Release build fails
1. Check the Actions tab for error details
2. Ensure all tests pass locally
3. Verify Cargo.toml is valid
4. Check that all dependencies compile

### Binaries not uploaded
1. Verify the tag matches the pattern `v*` (e.g., `v0.2.0`)
2. Check that `GITHUB_TOKEN` has the correct permissions
3. Review the workflow logs for upload errors

## Post-Release Tasks

After a successful release:

1. **Announce the release**:
   - Update README.md if needed
   - Share on social media/forums
   - Update documentation sites

2. **Update package managers** (if applicable):
   - Update Homebrew formula
   - Update installation scripts

3. **Monitor for issues**:
   - Watch GitHub Issues
   - Check download statistics
   - Respond to user feedback

## Quick Reference

```bash
# Complete release workflow
cargo test && cargo clippy -- -D warnings
cargo build --release
git add Cargo.toml CHANGELOG.md
git commit -m "Prepare v0.2.0 release"
git push
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

## CI/CD Pipeline Details

The release pipeline is defined in `.github/workflows/ci.yml` under the `build-artifacts` job:

- **Trigger**: Tag push matching pattern `refs/tags/*`
- **Platforms**: Ubuntu, Windows, macOS (latest)
- **Rust toolchain**: stable
- **Build command**: `cargo build --release --target <target-triple>`
- **Release action**: `softprops/action-gh-release@v1`

For more details, see `.github/workflows/ci.yml`.
