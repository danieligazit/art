pub struct FRange {
    val: f32,
    end: f32,
    incr: f32,
}

impl FRange {    
    
    pub fn new(val: f32, end: f32, incr: f32) -> FRange {
        FRange{val: val, end: end, incr: incr}
    } 

    pub fn incr_by(&mut self, step: f32) -> &FRange{
        self.incr = step;
        self
    }

}
impl Iterator for FRange {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res > self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}
