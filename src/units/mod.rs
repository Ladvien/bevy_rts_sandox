pub mod skelly;

trait CurrentAnimation {
    fn walk(&mut self);
    fn run(&mut self);
    fn idle(&mut self);
    fn attack(&mut self);
    fn spawn(&mut self);
    fn alerted(&mut self);
}
