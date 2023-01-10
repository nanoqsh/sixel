fn main() {
    use {
        sixel::{
            encode::{self, Image, Palette},
            Color,
        },
        std::{
            collections::HashMap,
            env,
            io::{self, BufWriter},
        },
    };

    let im = {
        let path = env::args()
            .nth(1)
            .unwrap_or_else(|| "unknown.png".to_owned());

        image::open(path).expect("read")
    };

    let im = im.as_rgb8().expect("rgb");
    let width = im.width() as usize;
    let height = im.height() as usize;

    let mut buf = Vec::with_capacity(width * height);
    let mut colors = Vec::with_capacity(256);
    let mut indxs = HashMap::with_capacity(256);

    for chunk in im.chunks(3) {
        let color = {
            let array: [u8; 3] = chunk.try_into().expect("array");
            let array = array.map(|v| v & !0b11111);
            Color::from(array)
        };

        let index = *indxs.entry(color).or_insert_with(|| {
            let index = colors.len().try_into().expect("to many colors");
            colors.push(color);
            index
        });

        buf.push(index);
    }

    let image = Image {
        pixels: &buf,
        width,
    };

    let palette = Palette { colors: &colors };

    let out = BufWriter::new(io::stdout().lock());
    encode::encode(image, palette, out).expect("encode");
}
