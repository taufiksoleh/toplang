# TopLang Homebrew Formula

This directory contains the Homebrew formula for installing TopLang.

## For Users: Installing via Homebrew

Once the Homebrew tap is set up, users can install TopLang with:

```bash
# Add the tap (only needed once)
brew tap taufiksoleh/toplang https://github.com/taufiksoleh/toplang

# Install toplang
brew install toplang

# Verify installation
topc --version
```

## For Maintainers: Setting Up the Homebrew Tap

To make TopLang installable via Homebrew, follow these steps:

### Option 1: Using this repository directly (Recommended)

Since the formula is already in this repository, users can tap directly:

```bash
brew tap taufiksoleh/toplang https://github.com/taufiksoleh/toplang
brew install toplang
```

Homebrew will automatically look for formulas in the `homebrew/` directory.

### Option 2: Creating a dedicated tap repository

Create a new repository named `homebrew-toplang` and copy the formula:

1. Create a new repository: `https://github.com/taufiksoleh/homebrew-toplang`
2. Copy `toplang.rb` to the root of that repository
3. Users can then install with:
   ```bash
   brew tap taufiksoleh/toplang
   brew install toplang
   ```

## Updating the Formula

When releasing a new version:

1. Update the `version` field in `toplang.rb`
2. Download the release binaries and calculate their SHA256 checksums:
   ```bash
   shasum -a 256 toplang-macos-x64
   shasum -a 256 toplang-linux-x64
   ```
3. Update the `sha256` fields in the formula
4. Commit and push the changes
5. Users can update with: `brew upgrade toplang`

## Testing the Formula Locally

Before publishing, test the formula locally:

```bash
# Audit the formula
brew audit --strict toplang.rb

# Test installation
brew install --build-from-source toplang.rb

# Run tests
brew test toplang

# Uninstall
brew uninstall toplang
```

## Current Status

- ✅ Formula created
- ⏳ Waiting for first release to populate SHA256 checksums
- ⏳ Tap repository to be set up

Once you create your first release (v0.1.0 or later), the SHA256 checksums need to be updated in the formula.
