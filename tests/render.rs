use sks::render::{
    ImageRenderer,
    RenderOptions,
};
use std::time::Instant;

#[test]
fn image_renderer_cache() {
    let lvl = include_str!("levels/1-4.lbl.txt");
    let (_, data) = sks::format::decode(lvl).unwrap();

    let mut renderer = ImageRenderer::new();
    let opts = RenderOptions {
        width: 1280,
        height: 720,
    };
    let start = Instant::now();

    #[allow(unused_variables)]
    let f = renderer.render(&data, &opts).unwrap();

    let mid = Instant::now();

    #[allow(unused_variables)]
    let s = renderer.render(&data, &opts).unwrap();

    let end = Instant::now();

    #[allow(unused_variables)]
    let time1 = mid - start;

    #[allow(unused_variables)]
    let time2 = end - mid;

    // f.save("1.png").unwrap();
    // s.save("2.png").unwrap();

    // dbg!(time1);
    // dbg!(time2);

    assert!(time1 > time2);
}

// TODO: Consider adding tests comparing actual rendered data. This might fail after bumping the image crate.
