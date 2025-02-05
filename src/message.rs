
#[derive(Debug, Clone)]
pub enum Message {
    UpdateView(glam::Vec2, f32),
    UpdateInput(String, usize),
    UpdateExpr(usize),
}
