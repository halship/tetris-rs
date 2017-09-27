mod piece;
mod field;
mod scene;

use rand::Rng;
pub use self::piece::Piece;
pub use self::field::{Cell, Field};
pub use self::scene::Scene;

/// ブロックの色の種類
#[derive(Clone, Copy, PartialEq)]
pub enum BlockColor {
    Cyan,
    Yellow,
    Green,
    Red,
    Blue,
    Orange,
    Magenta,
}

/// ゲーム世界
pub struct World {
    /// 全画面分のブロックの色を格納する配列
    pub field: Field,
    /// 現在操作できるピース
    pub piece: Piece,
    /// 次のピース
    pub next_piece: Piece,
    /// ピースの落ちるスピード
    pub fall_speed: f64,
}

impl World {
    /// Worldを生成して返す
    pub fn new<R: Rng>(rng: &mut R) -> World {
        World {
            field: Field::new(),
            piece: Piece::from_rng(rng),
            next_piece: Piece::from_rng(rng),
            fall_speed: 1.0,
        }
    }

    /// Worldをリセットする
    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.field.reset();
        self.piece = Piece::from_rng(rng);
        self.next_piece = Piece::from_rng(rng);
        self.fall_speed = 1.0;
    }
}
