
pub trait IActivity{
    fn on_create(&mut self,ctx :&egui::Context,frame:&mut eframe::Frame);
}