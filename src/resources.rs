use piston_window::{Flip, G2dTexture, Glyphs, PistonWindow, Texture, TextureSettings, Window};
use piston_window::types::SourceRectangle;
use settings::*;

/// ブロックに関するリソース
pub struct BlockResource {
    pub texture: G2dTexture,
    pub src_rects: Vec<SourceRectangle>,
}

/// ゲームのリソース
pub struct Resources {
    pub block: BlockResource,
    pub font: Glyphs,
}

impl Resources {
    // Resourcesを生成して返す
    pub fn new<W: Window>(window: &mut PistonWindow<W>) -> Resources {
        let texture = Texture::from_path(
            &mut window.factory,
            "resources/blocks.png",
            Flip::None,
            &TextureSettings::new(),
        ).unwrap();

        let src_rects = (0..COLOR_NUM)
            .map(|i| [BLOCK_SIZE * i as f64, 0.0, BLOCK_SIZE, BLOCK_SIZE])
            .collect();

        let block = BlockResource { texture, src_rects };

        let font = Glyphs::new(
            "resources/mplus-1p-regular.ttf",
            window.factory.clone(),
            TextureSettings::new(),
        ).unwrap();

        Resources { block, font }
    }
}
