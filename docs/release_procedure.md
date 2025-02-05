# Release procedure

1. Update the version in Cargo.toml.
2. Commit and push changes.
3. Create a release on GitHub.

Once the release is created, the GitHub Actions workflow will automatically build and upload the release artifacts. It will also create a new release on crates.io.
