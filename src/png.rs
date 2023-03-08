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

    pub fn clear(&mut self,color: (u8,u8,u8,u8)) {
        let mut data = vec![];
        for _ in 0..self.1*self.2 {
            data.push(color.0);
            data.push(color.1);
            data.push(color.2);
            data.push(color.3);
        }
        self.0 = data;
    }

    pub fn point(&mut self,x:u32,y:u32,color: (u8,u8,u8,u8)) {
        let index = if x == 0 {
            (x*4+(y*self.2)*4) as usize
        }else {
            (x*4+(y*self.2)*4) as usize
        };

        self.0[index] = color.0;
        self.0[index+1] = color.1;
        self.0[index+2] = color.2;
        self.0[index+3] = color.3;
    }

    pub fn rectangle(&mut self,x:u32,y:u32,width:u32,height:u32,color: (u8,u8,u8,u8)) -> Result<(),()>{
        let mut xc = 0;
        let mut yc = 0;

        if width == 0 || height == 0 {
            return Err(());
        }

        for i in 0..((self.1*self.2)*4) {
            if xc >= x && xc <= (width+x)-1 {
                if yc >= y && yc <= (height+y)-1 {
                    self.point(xc,yc,color);
                }
            }
            xc += 1;
            if xc == self.1*4 {
                yc += 1;
                xc = 0;
            }
        }

        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}