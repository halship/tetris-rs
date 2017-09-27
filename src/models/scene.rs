/// ゲームのシーン（場面）
#[derive(PartialEq)]
pub enum Scene {
    /// プレイ中
    Playing,
    /// ゲームオーバー
    GameOver,
}
