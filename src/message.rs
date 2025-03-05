
#[derive(Debug, Clone)]
pub enum Message {
    AddInput,
    UpdateView(glam::Vec2, f32),
    UpdateInput(String, usize),
    UpdateExpr(usize),
    RemoveInput(usize),
}
