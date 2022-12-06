#![allow(dead_code)]
use png;
#[test]
fn gen_legs() {
    for _ in 0..100 {
        test_png("y:/youtube_bevy/bevy_advent_of_code/assets/elf/img/legs0_t.png", "legs", r#"(
            image: "{}",
            size: (63., 52.)
        )"#);
        test_png("y:/youtube_bevy/bevy_advent_of_code/assets/elf/img/hat0_t.png", "hat", r#"(
            image: "{}",
            margin: (Auto, Auto, Auto, Px(-30.)),
            size: (100., 54.)
        )"#);
    }
}

fn test_png(template_path: &str, part_id: &str, part_file: &str) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let decoder = png::Decoder::new(File::open(template_path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let mut new_buf = vec![0; buf.len()];
    let p1 = rng.gen_range(0..ColorPairs::PAIRS.len());
    let p2 = rng.gen_range(0..ColorPairs::PAIRS.len());
    let p3 = rng.gen_range(0..ColorPairs::PAIRS.len());
    let pair1 = ColorPairs::PAIRS[p1];
    let pair2 = ColorPairs::PAIRS[p2];
    let pair3 = ColorPairs::PAIRS[p3];
    for i in (0..buf.len()).step_by(4) {
        if buf[i..i+4] == [255, 0, 0, 255] {
            new_buf[i + 0] = pair1.light[0];
            new_buf[i + 1] = pair1.light[1];
            new_buf[i + 2] = pair1.light[2];
            new_buf[i + 3] = 255;
        }
        match buf[i..i+4] {
            [255, 000, 000, 255] => {fill(&mut new_buf, i, pair1.light)},
            [255, 255, 000, 255] => {fill(&mut new_buf, i, pair1.dark)},
            [000, 255, 255, 255] => {fill(&mut new_buf, i, pair2.light)},
            [000, 255, 000, 255] => {fill(&mut new_buf, i, pair2.dark)},
            [255, 255, 255, 255] => {fill(&mut new_buf, i, pair3.light)},
            [255, 000, 255, 255] => {fill(&mut new_buf, i, pair3.dark)},
            _ => {
                new_buf[i]     = buf[i];
                new_buf[i + 1] = buf[i + 1];
                new_buf[i + 2] = buf[i + 2];
                new_buf[i + 3] = buf[i + 3];
            }
        }
    }

    use std::path::PathBuf;
    use std::fs::File;
    use std::io::BufWriter;
    let path_string = format!("y:/youtube_bevy/bevy_advent_of_code/assets/elf/img/{}{}_{}_{}.png",part_id, p1, p2, p3);
    let new_path = format!("elf/img/{}{}_{}_{}.png",part_id, p1, p2, p3);
    let data = part_file.replace("{}", &new_path);
    let data_path = format!("y:/youtube_bevy/bevy_advent_of_code/assets/elf/{}/{}{}_{}_{}.part.ron",part_id, part_id, p1, p2, p3);
    std::fs::write(data_path, data.as_bytes()).unwrap();
    let path = PathBuf::from(path_string);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&new_buf).unwrap();
}

fn fill(fill: &mut [u8], index: usize,  with: [u8; 3]) {
    fill[index] = with[0];
    fill[index + 1] = with[1];
    fill[index + 2] = with[2];
    fill[index + 3] = 255;
}

#[derive(Clone, Copy)]
struct ColorPairs {
    light: [u8; 3],
    dark: [u8; 3],
}

impl ColorPairs {
    const PAIRS: [ColorPairs; 5] = [
        ColorPairs::GREEN, ColorPairs::GREEN2, ColorPairs::BLUE, ColorPairs::PURPLE, ColorPairs::PURPLE2
    ];
    const GREEN: ColorPairs = ColorPairs {
        light: [0, 167, 80],
        dark: [0, 113, 61],
    };
    const GREEN2: ColorPairs = ColorPairs {
        light: [0, 113, 61],
        dark: [0, 75, 39],
    };
    const BLUE: ColorPairs = ColorPairs {
        light: [0x28,0x7c,0xfa],
        dark: [0x19,0x50,0xff],
    };
    const PURPLE: ColorPairs = ColorPairs {
        light: [113, 0, 52],
        dark: [75, 0, 36],
    };
    const PURPLE2: ColorPairs = ColorPairs {
        light: [167, 0, 87],
        dark: [113, 0, 52],
    };
}