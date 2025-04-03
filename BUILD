package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_clippy", "rust_library", "rustfmt_test")

rust_library(
    name = "sbi-rs",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/sbi.rs",
    deps = [
        "@sbi-index//:arrayvec",
        "@sbi-index//:flagset",
        "@sbi-index//:static_assertions",
    ],
)

rust_clippy(
    name = "clippy",
    deps = ["sbi-rs"],
)

rustfmt_test(
    name = "rustfmt",
    targets = ["sbi-rs"],
)
