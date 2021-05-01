#![feature(in_band_lifetimes)]

use cellular_automata_comonad::Grid;
use image::GrayImage;
use nalgebra::DMatrix;
use structopt::StructOpt;

/// Filtro gaussiano implementado como commonads
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn image2matrix(img: GrayImage) -> DMatrix<u8> {
    let (width, height) = img.dimensions();
    DMatrix::from_fn(width as usize, height as usize, move |r, c| {
        img.get_pixel(r as u32, c as u32)[0]
    })
}

fn matrix2image(m: &DMatrix<u8>) -> GrayImage {
    let nrows = m.nrows();
    let ncols = m.ncols();
    let mut imgbuf = image::ImageBuffer::new(nrows as u32, ncols as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = *m.index((x as usize, y as usize));
        *pixel = image::Luma([value]);
    }
    imgbuf
}

fn main() {
    let args = Cli::from_args();
    let img = image::open(args.path).expect("Image not found");
    let img = image2matrix(img.to_luma8());

    let mut layer = Grid::new(img, (0, 0));

    let rule = |s: &Grid<u8>| -> u8 {
        let s11 = s.get((-1, -1)).extract() as usize;
        let s12 = s.get((0, -1)).extract() as usize;
        let s13 = s.get((1, -1)).extract() as usize;
        let s21 = s.get((-1, 0)).extract() as usize;
        let s22 = s.get((0, 0)).extract() as usize;
        let s23 = s.get((1, 0)).extract() as usize;
        let s31 = s.get((-1, 1)).extract() as usize;
        let s32 = s.get((0, 1)).extract() as usize;
        let s33 = s.get((1, 1)).extract() as usize;

        let value = (s11 + s12 + s13 + s21 + s22 + s23 + s31 + s32 + s33) / 9;
        value as u8
    };

    layer = layer.extend(rule);
    matrix2image(&(*layer.data))
        .save("output.png")
        .expect("Erro on save");
}
