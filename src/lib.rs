mod maths;
mod consts;
mod matrix;
mod body;
mod world;
mod manifold;

pub use body::Body;
pub use world::World;

pub trait VecImpl {
	fn get_x(&self) -> f32;
	fn get_y(&self) -> f32;
	fn set_x(&mut self, x: f32);
	fn set_y(&mut self, y: f32);
	fn normalized(&self) -> f32;
	fn plus(&self, other: &Self) -> Self;
	fn minus(&self, other: &Self) -> Self;
	fn dot(&self, other: &Self) -> f32;
}
