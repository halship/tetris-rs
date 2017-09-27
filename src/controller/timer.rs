/// 時間にかかわるデータ
pub struct Timer {
    /// 現在の経過時間
    pub current_time: f64,
    /// 最後にピースが落ちた時間
    pub last_fall: f64,
    /// 最後にピースが操作された時間
    pub last_move: f64,
    /// 最後に行が消された時間
    pub last_delete: f64,
}

impl Timer {
    /// Timerを生成して返す
    pub fn new() -> Timer {
        Timer {
            current_time: 0.0,
            last_fall: 0.0,
            last_move: 0.0,
            last_delete: 0.0,
        }
    }

    /// 現在の経過時間を更新する
    pub fn update_current_time(&mut self, dt: f64) {
        self.current_time += dt;
    }

    /// 最後にピースが落ちた時間を更新する
    pub fn update_last_fall(&mut self) {
        self.last_fall = self.current_time;
    }

    /// 最後にピースが操作された時間を更新する
    pub fn update_last_move(&mut self) {
        self.last_move = self.current_time;
    }

    /// 最後に行が消された時間を更新する
    pub fn update_last_delete(&mut self) {
        self.last_delete = self.current_time;
    }

    /// すべてリセットする
    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.last_delete = 0.0;
        self.last_fall = 0.0;
        self.last_move = 0.0;
    }
}
