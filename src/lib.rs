extern crate raster;

use raster::{
    editor::{blend, fill, resize},
    open, save, transform, BlendMode, Color, Image, PositionMode, ResizeMode,
};

struct Imagem {
    imagem: Image,
    salvar: String,
}

impl Imagem {
    fn obter_imagem() -> Self {
        if std::env::args().len() != 2 {
            panic!("Favor entrar com um arquivo.")
        };
        let arquivo = std::env::args().nth(1).unwrap();
        Self {
            imagem: open(&arquivo).unwrap(),
            salvar: arquivo.trim().replace(".jpg", "-10x15.jpg"),
        }
    }
    fn montar_cartela(mut self) {
        resize(&mut self.imagem, 354, 472, ResizeMode::Fit).unwrap();
        let mut img_dupla_3x4 = Image::blank(354, 974);
        fill(&mut img_dupla_3x4, Color::rgb(255, 255, 255)).unwrap();

        for &i in [0, 502].iter() {
            img_dupla_3x4 = blend(
                &img_dupla_3x4,
                &self.imagem,
                BlendMode::Normal,
                1.0,
                PositionMode::TopLeft,
                0,
                i,
            ).unwrap();
        }

        let mut cartela = Image::blank(1200, 1800);
        fill(&mut cartela, Color::rgb(255, 255, 255)).unwrap();
        let mut pos = 30;
        for i in 1..6 {
            cartela = blend(
                &cartela,
                &img_dupla_3x4,
                BlendMode::Normal,
                1.0,
                PositionMode::TopLeft,
                pos,
                30,
            ).unwrap();
            pos += 384;
            if i == 3 {
                transform::rotate(&mut cartela, 90, Color::rgb(255, 255, 255)).unwrap();
                pos = 30;
            }
        }
        save(&mut cartela, &self.salvar).unwrap();
    }
}

pub fn executar() {
    Imagem::obter_imagem().montar_cartela();
}
