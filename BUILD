# SPDX-FileCopyrightText: 2023 Rivos Inc.
#
# SPDX-License-Identifier: Apache-2.0

package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:defs.bzl", "rust_clippy", "rust_library", "rustfmt_test")

rust_library(
    name = "sbi-rs",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/sbi.rs",
    proc_macro_deps = ["@sbi-index//:enum_dispatch"],
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
