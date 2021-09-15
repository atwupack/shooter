
pub trait HasPosition {
    fn x(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn y(&self) -> f32;
    fn set_y(&mut self, y: f32);
}

pub trait HasVelocity: HasPosition {
    fn dx(&self) -> f32;
    fn set_dx(&mut self, dx: f32);
    fn dy(&self) -> f32;
    fn set_dy(&mut self, dy: f32);

    fn apply_velocity(&mut self) {
        self.set_x( self.x() + self.dx());
        self.set_y( self.y() + self.dy());
    }
}

pub trait IsRendered<T>: HasPosition {
    fn entity_type(&self) -> &T;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}