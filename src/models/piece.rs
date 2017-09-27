use super::BlockColor;
use settings::*;
use rand::Rng;

const FORMS: [[[usize; 4]; 4]; 7] = [
    [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0]],
    [[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
];

/// ピース
#[derive(Clone)]
pub struct Piece {
    /// 形 1->ブロック 0->空白
    pub form: [[usize; 4]; 4],
    /// 色
    pub color: BlockColor,
    /// ブロックの位置
    pub pos: [isize; 2],
}

impl Piece {
    /// Pieceを数字から生成して返す
    pub fn new(n: usize) -> Piece {
        let (form, color) = match n {
            0 => (FORMS[0], BlockColor::Cyan),
            1 => (FORMS[1], BlockColor::Yellow),
            2 => (FORMS[2], BlockColor::Green),
            3 => (FORMS[3], BlockColor::Red),
            4 => (FORMS[4], BlockColor::Blue),
            5 => (FORMS[5], BlockColor::Orange),
            _ => (FORMS[6], BlockColor::Magenta),
        };
        let pos = [((SCREEN_WIDTH / 2) - 2) as isize, -3];

        Piece { form, color, pos }
    }

    /// Pieceを乱数生成機から生成して返す
    pub fn from_rng<R: Rng>(rng: &mut R) -> Piece {
        Piece::new(rng.gen_range(0, COLOR_NUM))
    }

    /// ピースからの相対座標から絶対座標を計算して返す
    pub fn get_absolute_position(&self, relative_pos: [usize; 2]) -> [isize; 2] {
        [
            self.pos[0] + relative_pos[0] as isize,
            self.pos[1] + relative_pos[1] as isize,
        ]
    }
}
