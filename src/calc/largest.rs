#[derive(Debug)]
pub struct Largest<T> {
    pub v1: T,
    pub v2: T,
}

impl Largest<i32> {
    pub fn bigger(&self) -> i32 {
        if &self.v1 > &self.v2 {
            self.v1
        } else {
            self.v2
        }
    }
    pub fn smaller(&self) -> i32 {
        if &self.v1 < &self.v2 {
            self.v1
        } else {
            self.v2
        }
    }
}
impl Largest<f32> {
    pub fn bigger(&self) -> f32 {
        if &self.v1 > &self.v2 {
            self.v1
        } else {
            self.v2
        }
    }

    pub fn smaller(&self) -> f32 {
        if &self.v1 < &self.v2 {
            self.v1
        } else {
            self.v2
        }
    }
}
impl Largest<String> {
    pub fn bigger(&self) -> &String {
        if &self.v1.len() > &self.v2.len() {
            &self.v1
        } else {
            &self.v2
        }
    }
    pub fn smaller(&self) -> &String {
        if &self.v1.len() < &self.v2.len() {
            &self.v1
        } else {
            &self.v2
        }
    }
}
