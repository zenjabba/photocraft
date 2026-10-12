# Content-Aware Scale / Free Transform comparison

Actual native PhotoCraft offscreen renders for PR #2058 revision fe9cf767. Both images show the same synthetic red rectangle, corrected 2:1 pixel aspect, viewport, and outside-corner hover position. Content-Aware Scale shows the default pointer; Free Transform retains the rotation arrow.

Captured with the CPU compositor and llvmpipe frame rendering (`--safe-gpu`), Rust 1.99, using the snapshot example. These images verify cursor/overlay behavior and are not GPU performance measurements. Capture scripts and hashes are retained with the local validation report.

Synthetic pixels and documentation: MIT OR Apache-2.0, @zenjabba. Existing displayed branding remains under its existing license; no brand asset was changed.
