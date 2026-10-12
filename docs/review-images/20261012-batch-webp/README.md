# Batch and Image Processor WebP controls

Actual native offscreen WGPU captures from the final Batch export tests on
2026-10-12. Rust source hashes match code commit `31998199c3326829e8c3e1e323564f159f99250d`
on main base `d380309338afafd7560c1f61649a15c682c82291`.

- `batch-quality.png`: Batch using WebP and Custom quality 12.
- `processor-formats.png`: the Image Processor format dropdown offers WebP.

These are unchanged renderer outputs, directly inspected. The tests use only
synthetic generated content and exercise the real controls through confirmation
to written and decoded files. Temporary paths shown belong to the test fixture.

- `batch-quality.png` SHA-256: `c70e28267fab77620e8f0365215de96771fe015a85b76f97e626574fa1e936ee`.
- `processor-formats.png` SHA-256: `2c6420c427937a4d36a366f832872feaefccfedc26cac4b170a8b917363a90e5`.

The source change is in a separate PR; this branch hosts review evidence only.
Synthetic content and documentation are MIT OR Apache-2.0; license texts are
adjacent. Existing app branding retains its documented license.
