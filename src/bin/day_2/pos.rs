pub struct Pos {
    pub x: i32,
    pub depth: i32,
    pub aim: i32,
}
impl Pos {
    pub fn new(x: i32, depth: i32, aim: i32) -> Self {
        Self { x, depth, aim }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }

    pub fn set_aim(&mut self, aim: i32) {
        self.aim = aim;
    }
    pub fn result(&mut self) -> i32 {
        self.x * self.depth
    }
}
