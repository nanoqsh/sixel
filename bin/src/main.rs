fn main() {
    use {
        sixel::{
            encode::{self, Image, Palette},
            Color,
        },
        std::{collections::HashMap, env, io},
    };

    let im = {
        let path = env::args()
            .nth(1)
            .unwrap_or_else(|| "unknown.png".to_owned());

        image::open(path).expect("read")
    };

    let im = im.as_rgb8().expect("rgb");

    let mut buf = Vec::with_capacity(im.width() as usize * im.height() as usize);
    let mut colors = Vec::new();
    let mut indxs = HashMap::new();

    for chunk in im.chunks(3) {
        let color = {
            let array: [u8; 3] = chunk.try_into().expect("array");
            let array = array.map(|v| v & !0b1111);
            Color::from(array)
        };

        let index = indxs.entry(color).or_insert_with(|| {
            let index = colors.len() as u32;
            colors.push(color);
            index
        });

        buf.push(*index);
    }

    let image = Image {
        pixels: &buf,
        width: im.width() as usize,
    };

    let palette = Palette {
        colors: &colors,
        dither: &[],
    };

    let out = io::stdout();
    encode::encode(image, palette, out).expect("encode");
}
