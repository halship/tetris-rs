use super::BlockColor;
use settings::*;

/// フィールド内の中身
#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    /// ブロック
    Block { color: BlockColor },
    /// 壁
    Wall,
    /// なにもない
    Empty,
}

/// ゲームのフィールド
pub struct Field([[Cell; SCREEN_WIDTH]; SCREEN_HEIGHT]);

impl Field {
    /// Fieldを生成して返す
    pub fn new() -> Field {
        Field([[Cell::Empty; SCREEN_WIDTH]; SCREEN_HEIGHT])
    }

    /// 指定した位置の中身を返す
    pub fn get(&self, i: isize, j: isize) -> Cell {
        if i >= SCREEN_HEIGHT as isize || j < 0 || j >= SCREEN_WIDTH as isize {
            Cell::Wall
        } else if i < 0 {
            Cell::Empty
        } else {
            self.0[i as usize][j as usize]
        }
    }

    /// 指定した位置にブロックをセットする
    pub fn set(&mut self, i: isize, j: isize, color: BlockColor) {
        if i < 0 || i >= SCREEN_HEIGHT as isize || j < 0 || j >= SCREEN_WIDTH as isize {
            return;
        }
        self.0[i as usize][j as usize] = Cell::Block { color };
    }

    /// ブロックで埋まっている行の番号の配列を返す
    pub fn get_filled_lines(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, row)| if row.iter().all(|&cell| cell != Cell::Empty) {
                Some(i)
            } else {
                None
            })
            .collect()
    }

    /// 指定された行を削除する
    pub fn delete_line(&mut self, index: usize) {
        for j in 0..SCREEN_WIDTH {
            self.0[index][j] = Cell::Empty;
        }
    }

    /// 指定された行を下に詰める
    pub fn shift_line(&mut self, index: usize) {
        for i in (1..index).rev() {
            for j in 0..SCREEN_WIDTH {
                self.0[i + 1][j] = self.0[i][j];
            }
        }
        for j in 0..SCREEN_WIDTH {
            self.0[0][j] = Cell::Empty;
        }
    }

    /// フィールドをリセットする
    pub fn reset(&mut self) {
        for row in self.0.iter_mut() {
            for cell in row.iter_mut() {
                *cell = Cell::Empty;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use models::{BlockColor, Field};
    use settings::*;

    #[test]
    fn get_valid_filled_lines() {
        let mut field = Field::new();
        for j in 0..SCREEN_WIDTH {
            field.set(1, j as isize, BlockColor::Cyan);
            field.set(5, j as isize, BlockColor::Cyan);
        }

        assert_eq!(vec![1, 5], field.get_filled_lines());
    }
}
