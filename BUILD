load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_tarball")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_rust//rust:defs.bzl", "rust_binary")

platform(
    name = "linux-amd64",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

rust_binary(
    name = "support",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    deps = all_crate_deps(normal = True),
)

pkg_tar(
    name = "layer",
    srcs = [":support"],
)

oci_image(
    name = "image",
    base = "@distroless_cc",
    entrypoint = ["/support"],
    tars = [":layer"],
)

oci_tarball(
    name = "image.tar",
    image = ":image",
    repo_tags = ["support:latest"],
)