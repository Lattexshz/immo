pub struct Png(Vec<u8>);

impl Png {
    pub fn new(width:u32,height:u32) -> Self {
        let mut data = vec![];
        for i in 0..width*height {
            data.push(0);
            data.push(0);
            data.push(0);
            data.push(0);
        }
        Self {
            0: data
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}