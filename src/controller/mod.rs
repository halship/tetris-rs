use piston_window::Button;
use rand::Rng;
use models::{Piece, Scene, World};
use utils::Direction;
use settings::*;

mod action;
mod timer;
use self::action::Action;
pub use self::timer::Timer;

/// ゲームのコントローラ
pub struct Controller {
    /// ゲームのアクション
    action: Action,
    /// 消された行の配列
    deleted_line: Vec<usize>,
}

impl Controller {
    /// Controllerを生成して返す
    pub fn new() -> Controller {
        Controller {
            action: Action::default(),
            deleted_line: Vec::new(),
        }
    }

    /// ゲーム全体を更新する
    pub fn update_game<R: Rng>(
        &mut self,
        world: &mut World,
        rng: &mut R,
        timer: &mut Timer,
        scene: &mut Scene,
    ) {
        if *scene == Scene::GameOver {
            if self.action.more_play {
                *scene = Scene::Playing;
                world.reset(rng);
                timer.reset();
            }
            return;
        }

        let mut change_next_piece = false;

        if timer.current_time - timer.last_move >= MOVE_SPEED {
            if self.action.move_left {
                action::move_piece(&mut world.piece, &world.field, Direction::Left);
                timer.update_last_move();
            }
            if self.action.move_right {
                action::move_piece(&mut world.piece, &world.field, Direction::Right);
                timer.update_last_move();
            }
            if self.action.move_down {
                if !action::move_piece(&mut world.piece, &world.field, Direction::Down) {
                    change_next_piece = true;
                }
                timer.update_last_move();
                timer.update_last_fall();
            }
            if self.action.rotate {
                action::rotate_piece(&mut world.piece, &world.field);
                timer.update_last_move();
            }
        }

        if timer.current_time - timer.last_fall >= world.fall_speed {
            if !action::move_piece(&mut world.piece, &world.field, Direction::Down) {
                change_next_piece = true;
            }
            timer.update_last_fall();
        }

        if (timer.current_time - timer.last_delete >= SHIFT_TIME) && !self.deleted_line.is_empty() {
            for i in &self.deleted_line {
                world.field.shift_line(*i);
            }
            self.deleted_line.clear();
        }

        if change_next_piece {
            // ピースが上にはみ出ていた場合ゲームオーバーにする
            if world.piece.pos[1] < 0 {
                *scene = Scene::GameOver;
                return;
            }

            action::to_field(&world.piece, &mut world.field);

            // ブロックで埋まった行を消す
            self.deleted_line = world.field.get_filled_lines();
            for i in &self.deleted_line {
                world.field.delete_line(*i);
            }
            timer.update_last_delete();

            // 次にセットされていたピースを現在のものにし次のピースを新たにセットする
            world.piece = world.next_piece.clone();
            world.next_piece = Piece::from_rng(rng);
        }
    }

    /// ボタンが押された時の処理をする
    pub fn handle_button(&mut self, key: &Button, pressed: bool) {
        use piston_window::Key;

        match *key {
            Button::Keyboard(Key::Left) => self.action.move_left = pressed,
            Button::Keyboard(Key::Right) => self.action.move_right = pressed,
            Button::Keyboard(Key::Down) => self.action.move_down = pressed,
            Button::Keyboard(Key::Up) => self.action.rotate = pressed,
            Button::Keyboard(Key::Space) => self.action.more_play = pressed,
            _ => {}
        };
    }
}
