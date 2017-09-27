use piston_window::{Context, G2d, Transformed};
use resources::Resources;
use models::{BlockColor, Cell, Field, Piece, Scene, World};
use settings::*;

/// ゲーム全体を描画する
pub fn render_game(c: &Context, g: &mut G2d, world: &World, res: &mut Resources, scene: &Scene) {
    use piston_window::clear;

    clear([0.0, 0.0, 0.0, 1.0], g);
    render_screen(c, g, &world.field, res);
    render_piece(c, g, &world.piece, res);
    render_next_piece(c, g, &world.next_piece, res);
    render_line(c, g);

    if *scene == Scene::GameOver {
        render_text(c, g, res);
    }
}

/// テキストを描画する
fn render_text(c: &Context, g: &mut G2d, res: &mut Resources) {
    use piston_window::Text;

    Text::new_color([1.0; 4], 20).draw(
        "Game Over",
        &mut res.font,
        &c.draw_state,
        c.trans(10.0, 50.0).transform,
        g,
    );
}

/// スクリーンを描画する
fn render_screen(c: &Context, g: &mut G2d, field: &Field, res: &Resources) {
    for i in 0..SCREEN_HEIGHT {
        for j in 0..SCREEN_WIDTH {
            let x = BLOCK_SIZE * j as f64;
            let y = BLOCK_SIZE * i as f64;

            if let Cell::Block { color } = field.get(i as isize, j as isize) {
                render_block(c, g, x, y, &color, res);
            }
        }
    }
}

/// 境界線を描画する
fn render_line(c: &Context, g: &mut G2d) {
    use piston_window::Line;

    let x = BLOCK_SIZE * SCREEN_WIDTH as f64 + 1.0;
    Line::new([0.8, 0.8, 0.8, 1.0], 1.0).draw(
        [x, 0.0, x, BLOCK_SIZE * SCREEN_HEIGHT as f64],
        &c.draw_state,
        c.transform,
        g,
    );
}

/// ピースを描画する
fn render_piece(c: &Context, g: &mut G2d, piece: &Piece, res: &Resources) {
    for i in 0..4 {
        for j in 0..4 {
            if piece.form[i][j] == 1 {
                let pos = piece.get_absolute_position([j, i]);
                let x = pos[0] as f64 * BLOCK_SIZE;
                let y = pos[1] as f64 * BLOCK_SIZE;

                render_block(c, g, x, y, &piece.color, res);
            }
        }
    }
}

/// 次のピースを描画する
fn render_next_piece(c: &Context, g: &mut G2d, next_piece: &Piece, res: &Resources) {
    for i in 0..4 {
        for j in 0..4 {
            if next_piece.form[i][j] == 0 {
                continue;
            }
            let x = BLOCK_SIZE * (j + SCREEN_WIDTH + 1) as f64;
            let y = BLOCK_SIZE * (i + 1) as f64;
            render_block(c, g, x, y, &next_piece.color, res);
        }
    }
}

/// ブロックを描画する
fn render_block(c: &Context, g: &mut G2d, x: f64, y: f64, color: &BlockColor, res: &Resources) {
    use piston_window::Image;

    let ind = match *color {
        BlockColor::Cyan => 0,
        BlockColor::Yellow => 1,
        BlockColor::Green => 2,
        BlockColor::Red => 3,
        BlockColor::Blue => 4,
        BlockColor::Orange => 5,
        BlockColor::Magenta => 6,
    };

    Image::new().src_rect(res.block.src_rects[ind]).draw(
        &res.block.texture,
        &c.draw_state,
        c.trans(x, y).transform,
        g,
    );
}
