pub struct Png(Vec<u8>,u32,u32);

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
            0: data,
            1: width,
            2: height,
        }
    }

    pub fn point(&mut self,x:u32,y:u32,color: (u8,u8,u8,u8)) {
        let index = if x == 0 {
            (x*4+(y*self.2)*4) as usize
        }else {
            (x*4+(y*self.2)*4) as usize
        };
        println!("index: {} len: {}",index,self.0.len());
        self.0[index] = color.0;
        self.0[index+1] = color.1;
        self.0[index+2] = color.2;
        self.0[index+3] = color.3;
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}