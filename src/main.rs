use image::{GenericImageView, imageops::FilterType};
use std::env;

const ESC: &str = "\x1b";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: png-to-terminal <image.png> [width]");
        return;
    }

    let path = &args[1];
    let width: u32 = args.get(2).and_then(|w| w.parse().ok()).unwrap_or(40);

    let img = image::open(path).expect("Resim açılamadı");
    let (w, h) = img.dimensions();

    let char_aspect = 0.5;
    let target_h = ((h as f32 / w as f32) * width as f32 / char_aspect) as u32;

    let resized = img.resize_exact(
        width,
        target_h * 2,
        FilterType::CatmullRom,
    );

    for y in (0..resized.height()).step_by(2) {
        let mut line = String::new();

        for x in 0..resized.width() {
            let top = resized.get_pixel(x, y);
            let bot = resized.get_pixel(x, (y + 1).min(resized.height() - 1));

            line.push_str(&format!(
                "{esc}[38;2;{tr};{tg};{tb}m{esc}[48;2;{br};{bg};{bb}m▀",
                esc = ESC,
                tr = top[0],
                tg = top[1],
                tb = top[2],
                br = bot[0],
                bg = bot[1],
                bb = bot[2],
            ));
        }

        line.push_str(&format!("{ESC}[0m"));
        println!("{}", line);
    }
}
