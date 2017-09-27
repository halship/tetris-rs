use models::{Cell, Field, Piece};
use utils::Direction;

/// ゲームで使われるアクション
pub struct Action {
    /// 左への移動
    pub move_left: bool,
    /// 右への移動
    pub move_right: bool,
    /// 下への移動
    pub move_down: bool,
    /// 回転
    pub rotate: bool,
    /// ゲームをもう一回やる
    pub more_play: bool,
}

impl Default for Action {
    fn default() -> Action {
        Action {
            move_left: false,
            move_right: false,
            move_down: false,
            rotate: false,
            more_play: false,
        }
    }
}

/// ピースを回転する
pub fn rotate_piece(piece: &mut Piece, field: &Field) {
    let prev_form = piece.form;

    for i in 0..4 {
        for j in 0..4 {
            piece.form[i][j] = prev_form[3 - j][i];
        }
    }

    if is_collided_with(piece, field) {
        piece.form = prev_form;
    }
}

/// ピースを移動する
/// 戻り値は移動が実際にされたかどうか（true->成功 false->失敗）
pub fn move_piece(piece: &mut Piece, field: &Field, direction: Direction) -> bool {
    let prev_pos = piece.pos;
    match direction {
        Direction::Down => piece.pos[1] += 1,
        Direction::Left => piece.pos[0] -= 1,
        Direction::Right => piece.pos[0] += 1,
    };

    // 衝突していた場合、移動前の座標に戻す
    if is_collided_with(piece, field) {
        piece.pos = prev_pos;
        return false;
    }
    true
}

/// ピースがフィールド上のブロックまたは壁に衝突しているかを返す
pub fn is_collided_with(piece: &Piece, field: &Field) -> bool {
    for (i, row) in piece.form.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n == 0 {
                continue;
            }
            let pos = piece.get_absolute_position([j, i]);
            if field.get(pos[1], pos[0]) != Cell::Empty {
                return true;
            }
        }
    }
    false
}

/// ピースをフィールドに移動する
pub fn to_field(piece: &Piece, field: &mut Field) {
    for (i, row) in piece.form.iter().enumerate() {
        for (j, n) in row.iter().enumerate() {
            if *n == 0 {
                continue;
            }

            let pos = piece.get_absolute_position([j, i]);
            field.set(pos[1], pos[0], piece.color);
        }
    }
}
