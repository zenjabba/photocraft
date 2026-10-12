# Custom paper controls

Actual offscreen PhotoCraft Print Settings dialog renders for issue #3820, pinned to main 88637281 plus the isolated paper-controls patch. The before menu offers only presets; the after menu includes Custom. The dimensions example uses 7.25 × 9.25 inches, shown in landscape with English and French labels. The public UI integration regression exercises these controls and checks the resulting PDF MediaBox.

Rendered with egui-kittest/wgpu using the local Mesa software adapter. These captures verify controls, layout and localization; no printer hardware was used. Original synthetic test document, @zenjabba, MIT OR Apache-2.0.
