use criterion::{criterion_group, criterion_main, Criterion};
use image::ImageReader;

#[allow(unused)]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[allow(unused)]
fn fibonacci2(n: u32) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    for _ in 2..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

fn image_merge() {
    let img1 = ImageReader::open("benches/assets/image_merge/img1.jpg").unwrap().decode().unwrap();
    let img2 = ImageReader::open("benches/assets/image_merge/img2.jpg").unwrap().decode().unwrap();
    let imgs = vec![img1, img2];
    samsung_notes_merger::merge_images(&imgs);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("image merging", |b| b.iter(|| image_merge()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
