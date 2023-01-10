use {
    crate::{Color, Index},
    std::{
        collections::HashSet,
        io::{self, Write},
    },
};

#[derive(Clone, Copy)]
pub struct Palette<'a> {
    pub colors: &'a [Color],
}

#[derive(Clone, Copy)]
pub struct Image<'a, I> {
    pub pixels: &'a [I],
    pub width: usize,
}

impl<I> Image<'_, I> {
    fn lines(&self) -> impl Iterator<Item = Line<I>> + '_ {
        let sixlen = self.width * 6;
        self.pixels.chunks(sixlen).map(|sixline| Line {
            sixline,
            width: self.width,
        })
    }
}

struct Line<'a, I> {
    sixline: &'a [I],
    width: usize,
}

impl<I> Line<'_, I> {
    fn write<W>(self, mut out: W) -> Result<(), Error>
    where
        I: Index,
        W: Write,
    {
        let mut colors_to_write = HashSet::with_capacity(64);
        for index in self.sixline {
            colors_to_write.insert(index.index());
        }

        let mut sixels = vec![0; self.width];
        let mut printed_line = false;
        for index in colors_to_write {
            sixels.fill(0);
            let mut any_pixel_used = false;
            for (n, pixel) in (0..).zip(self.sixline) {
                if pixel.index() == index {
                    let row = n / self.width;
                    let col = n % self.width;
                    sixels[col] |= 1 << row;
                    any_pixel_used = true;
                }
            }

            if any_pixel_used {
                if printed_line {
                    write!(out, "$")?;
                }

                printed_line = true;
                write!(out, "#{index}")?;
                write_sixels(&sixels, &mut out)?;
            }
        }

        Ok(())
    }
}

fn write_sixels<W>(sixels: &[u8], mut out: W) -> Result<(), Error>
where
    W: Write,
{
    let mut write = |six: u8, n: usize| -> Result<(), Error> {
        use std::slice;

        let six = six + b'?';
        let bytes = slice::from_ref(&six);
        if n > 3 {
            out.write_all(b"!")?;
            write!(out, "{n}")?;
            out.write_all(bytes)?;
        } else {
            for _ in 0..n {
                out.write_all(bytes)?;
            }
        }

        Ok(())
    };

    let mut to_print = (0, 0);
    for &six in sixels {
        match &mut to_print {
            (_, 0) => to_print = (six, 1),
            (left, t) if *left == six => *t += 1,
            (left, t) => {
                write(*left, *t)?;
                *left = six;
                *t = 1;
            }
        }
    }

    let (left, t) = to_print;
    write(left, t)
}

pub fn encode<I, W>(image: Image<I>, palette: Palette, mut out: W) -> Result<(), Error>
where
    I: Index,
    W: Write,
{
    use std::slice;

    const ESCAPE: &[u8] = slice::from_ref(&27);

    // Write header
    out.write_all(ESCAPE)?;
    out.write_all(b"Pq")?;

    // Write colors
    for (index, color) in (0..).zip(palette.colors) {
        color.write(index, &mut out)?;
    }

    // Write lines
    for line in image.lines() {
        line.write(&mut out)?;
        out.write_all(b"-")?;
    }

    // Write end
    out.write_all(ESCAPE)?;
    out.write_all(b"\\")?;
    out.flush()?;

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}
