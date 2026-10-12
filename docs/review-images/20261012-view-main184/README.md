# View and Vanishing Point: fresh main184 captures

Actual stock native snapshot output from the frozen PR2058 repair tree `69e8549033866d24c9abdfcc4234e30cfd6ea83c` on base `184c226a76c9ca6234d473912d438feaa48c1cb0`. All 1,659 source inputs were verified unchanged around capture, including the new aspect regression fixture. Renderer: CPU canvas with Vulkan llvmpipe frontend; snapshot binary SHA-256 `4de96e012673196eaacf1ba4da364e1b8841f5b0b8b8517d80c1622531543d1c`.

Synthetic RGB16 document with pixel aspect 2. Display scale1/rotation0 and scale2/rotation37 show the perspective plane, grid, handles and clipboard preview following the document surface. Root inspected both images. Meaningful functional tests separately establish native handle/grab/commit behavior; these images show presentation. No OS cursor upload or hardware GPU performance is represented.

These review assets are fork-only and outside the code PR. Synthetic content/documentation: MIT OR Apache-2.0; license texts are adjacent. App branding retains its documented license.
