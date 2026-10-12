# Smart Filter row menu

Actual native offscreen WGPU captures from the final Smart Filter tests on
2026-10-12. Rust source hashes match code commit `ea728ee28f33ca3daacf8f603e591a2c6714edbf`
on main base `d380309338afafd7560c1f61649a15c682c82291`.

English1x and French2x display the bottom Offset filter menu while another
layer remains active. Move Down is disabled at this boundary; Move Up and
Delete Smart Filter are enabled. The artwork is an original synthetic32x24
test fixture. These are directly inspected, unchanged renderer outputs.

- `smart-filter-menu-fr-2x.png` SHA-256: `a7bc08e1c13a6cb957b55e70d9a98bf494aaf8ca643a4b1bb11fe661e25b86b0`.
- `smart-filter-menu-en-1x.png` SHA-256: `f40a8b7bf8fdeba3c05c2f30e5d6b6f49c7db6bde47bf9dd04cb6b85847f9703`.

The source change is in a separate PR; this branch hosts review evidence only.
Synthetic content and documentation are MIT OR Apache-2.0; license texts are
adjacent. Existing app branding retains its documented license.
