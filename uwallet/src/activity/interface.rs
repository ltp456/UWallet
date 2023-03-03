
pub trait IActivity{
    fn on_create(ctx :&egui::Context,frame:&mut eframe::Frame);
}