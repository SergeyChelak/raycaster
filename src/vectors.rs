use std::ops::{Add, AddAssign};

#[derive(Default, Copy, Clone)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> AddAssign<Self> for Vec2d<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl<T> AddAssign<&Self> for Vec2d<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Add<&Self> for Vec2d<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<Self> for Vec2d<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}
