//! Copy into crates/gpu/examples/gpu2053_benchmark.rs in BOTH compared trees.
//! Run cargo run --release -p photocraft-gpu --example gpu2053_benchmark.
//! Uses the app's GPU-or-CPU decision, includes readback, and checks every pixel.
use photocraft_color::{Color, ColorMode, SampleType};
use photocraft_doc::{Document, Fill, GradientStyle, Layer, LayerContent};
use photocraft_geom::Size;
use photocraft_gpu::{Compositor, render_to_vec};
use std::time::Instant;

fn main() {
    let label = std::env::var("PHOTOCRAFT_PROBE_LABEL").unwrap_or_else(|_| "unlabelled".into());
    let instance = wgpu::Instance::default();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).expect("GPU required");
    println!("ADAPTER {label} {:?}", adapter.get_info());
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();
    assert_eq!(Compositor::preferred_acc_format(&adapter), wgpu::TextureFormat::Rgba32Float);
    // The first creation includes cold driver compilation; subsequent rows expose caching.
    let mut comp = None;
    for rep in 0..3 {
        let start = Instant::now();
        let c = Compositor::try_new_with_format(&device, wgpu::TextureFormat::Rgba32Float).unwrap();
        println!("PIPELINES {label} rep={rep} ms={:.3}", start.elapsed().as_secs_f64() * 1000.0);
        comp = Some(c);
    }
    let mut comp = comp.unwrap();
    comp.set_composite_cache_budget(0);
    for count in [2, 4, 256, 257] {
        let stops = match count {
            2 => vec![(0.0, Color::BLACK), (1.0, Color::WHITE)],
            4 => vec![(0.0, Color::BLACK), (0.5, Color::WHITE), (0.5, Color::BLACK), (1.0, Color::WHITE)],
            _ => {
                let mut s: Vec<_> = (0..count).map(|i| (i as f32 / (count - 1) as f32, if i % 2 == 0 { Color::BLACK } else { Color::WHITE })).collect();
                s[1].0 = 0.0;
                s
            }
        };
        let mut fill = Fill::gradient(stops, 0.0, 1.0, GradientStyle::Linear, false);
        if count >= 256 && let Fill::Gradient { opacity_stops, .. } = &mut fill {
            *opacity_stops = (0..count).map(|i| (i as f32 / (count - 1) as f32, 0.5 + 0.5 * (i % 2) as f32)).collect();
            opacity_stops[1].0 = 0.0;
        }
        let mut doc = Document::new("24MP", Size::new(6000, 4000), ColorMode::Rgb, SampleType::U8);
        doc.layers.push(Layer::new("gradient", LayerContent::Fill(fill)));
        let reason = comp.supports(&doc).err();
        println!("PATH {label} count={count} gpu={} reason={reason:?}", reason.is_none());
        let expected_gpu = count == 2 || (count <= 256 && !label.contains("baseline"));
        assert_eq!(reason.is_none(), expected_gpu, "unexpected backend for {label} count={count}");
        for rep in 0..4 {
            let start = Instant::now();
            if reason.is_none() {
                let stats = comp.render(&device, &queue, &doc, doc.bounds(), |_, _| {}).unwrap();
                device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).unwrap();
                println!("RESIDENT {label} count={count} rep={rep} warmup={} ms={:.3} stats={stats:?}", rep == 0, start.elapsed().as_secs_f64() * 1000.0);
            } else {
                let pixels = photocraft_compose::render(&doc, doc.bounds());
                std::hint::black_box(&pixels);
                println!("RESIDENT {label} count={count} rep={rep} warmup={} ms={:.3} cpu_only=true", rep == 0, start.elapsed().as_secs_f64() * 1000.0);
            }
        }
        let mut out = Vec::new();
        for rep in 0..4 {
            let start = Instant::now();
            out = if reason.is_none() {
                render_to_vec(&mut comp, &device, &queue, &doc, doc.bounds()).unwrap()
            } else {
                photocraft_compose::render(&doc, doc.bounds()).px
            };
            println!("RENDER {label} count={count} rep={rep} warmup={} ms={:.3} midpoint={:?}", rep == 0, start.elapsed().as_secs_f64() * 1000.0, out[3000]);
        }
        let cpu = photocraft_compose::render(&doc, doc.bounds());
        let mut worst = 0.0_f32;
        for (a, b) in cpu.px.iter().zip(&out) {
            for ch in 0..4 {
                let d = if ch == 3 { (a[ch] - b[ch]).abs() } else { (a[ch] * a[3] - b[ch] * b[3]).abs() };
                assert!(d.is_finite());
                worst = worst.max(d);
            }
        }
        assert_eq!(out.len(), cpu.px.len());
        assert!(worst <= 1.0 / 255.0 + 4.0 * f32::EPSILON, "count={count} max diff={worst}");
        println!("PARITY {label} count={count} pixels={} max_diff_255={:.6}", out.len(), worst * 255.0);
    }
}
