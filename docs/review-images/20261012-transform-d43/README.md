# Actual transform comparisons on fixed main d43

Eight synthetic-document native offscreen captures bind the complete 1,659-input candidate to tree `174e23b6d8f07651f4e92347dd10b71a3f423a0f` on fixed main `d43db3e153c76aefce9b32cef9099e458eaf17a8`. They come from the sealed current snapshot binary, with all 19 generated native/Wasm catalogs decoded byte-for-byte to source. The four cases are Distort, Free Transform with Ctrl, Skew with Ctrl, and Perspective with Ctrl, each in Pro Medium at 1x and Studio Light at 2x, with a 1200×900 logical viewport.

Distort and Free+Ctrl move the selected corner to the visible guide intersection. Skew+Ctrl retains the skew parallelogram; Perspective+Ctrl retains paired-corner geometry. All eight images were visually inspected. The 27 snapping and 72 transform source tests establish the constraints and positions; images illustrate those states.

The rectangle, layers and guides are synthetic. Linux Mesa llvmpipe and CPU canvas composition with the WGPU frontend provide offscreen UI evidence, not native OS window, current Photoshop runtime, performance or hardware-budget validation. Performance is qualified separately in the PR.
