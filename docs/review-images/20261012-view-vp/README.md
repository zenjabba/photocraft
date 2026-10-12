# View and Vanishing Point integration evidence

Actual native offscreen eframe captures from the final PR #2058 repair,
2026-10-12. The source bytes match code commit
`2a380f2761c37ab3afc949714dea1b34d124a3ac` on main base
`d380309338afafd7560c1f61649a15c682c82291`; committing changed no captured source.

The synthetic document uses pixel aspect 2. The first image uses display scale
1 and camera rotation 0 degrees; the second uses scale 2 and rotation 37 degrees.
The perspective plane, corner and midpoint handles, grid and red clipboard
preview follow the corrected surface. Native functional tests assert the exact
handle radius, grab coordinates and commit; these images show rendered presentation.

Renderer: CPU canvas with Vulkan llvmpipe. These unchanged renderer outputs were
independently inspected. No OS cursor upload, Photoshop screenshot or hardware
GPU measurement is represented by these captures.

- `vp-dpi1-rot0.png` SHA-256: `478bd50bd715f413a880f01bbc5c7b729c65f399948910288cb0dca9852609e2`.
- `vp-dpi2-rot37.png` SHA-256: `81b2580d7d0381170c7e43c646446790a5b278d5f76149da7f7677d1a9f185ba`.

This fork-only branch hosts review evidence; the code PR contains no image assets.
Synthetic content and documentation are MIT OR Apache-2.0; license texts are
adjacent. Existing app branding retains its documented license.
