load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

# on changes to crate depdencies, run the following command:
# CARGO_BAZEL_REPIN=1 bazel sync --only=salus-index --only=salus-rwlock \
# --only=salus-once --only=salus-nodefault --only=salus-default

def sbi_dependencies():
    crates_repository(
        name = "sbi-index",
        isolated = False,
        cargo_lockfile = "@salus//sbi-rs/bazel-locks:Sbi-Cargo.Bazel.lock",
        lockfile = "@salus//sbi-rs/bazel-locks:sbi-cargo-bazel-lock.json",
        packages = {
            "arrayvec": crate.spec(
                version = "0.7.2",
                default_features = False,
            ),
            "enum_dispatch": crate.spec(
                version = "0.3.8",
            ),
            "flagset": crate.spec(
                version = "0.4.3",
            ),
            "static_assertions": crate.spec(
                version = "1.1",
            ),
        },
    )
