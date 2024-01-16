struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
}

fn new_map(pixels: Vec<u8>, size: (usize, usize)) {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size };
}

#[test]
fn test_not(){
    println!("{}",!31);
}



