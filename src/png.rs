use crate::error::{ErrorKind, ImageError, ImageType};

pub struct Png(Vec<u8>, u32, u32);

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

    pub fn line(&mut self,from:(u32,u32),to:(u32,u32),thickness:u32,color:(u8,u8,u8,u8)) -> Result<(),ImageError> {
        if from.1 == to.1 {
            // Horizontal
            self.fill_rectangle(from.0,from.1,from.0+to.0,thickness,color)?;
        } else if from.0 == to.0 {
            // Vertical
            self.fill_rectangle(from.0,from.1,thickness,from.1+to.1,color)?;
        } else if from.0 < to.0 {
            // Right lopsided
            let shift = to.0-from.0;
            let mut shift_x_count = 0;
            let mut shift_y_count = 0;

            let height = ((from.1+to.1)/shift);

            for _ in 0..shift {
                self.fill_rectangle(from.0+shift_x_count,from.1+shift_y_count,thickness,height,color)?;
                shift_x_count += thickness;
                shift_y_count += height;
            }

            let actual_height = height*shift;
            if actual_height < from.1+to.1 {
                self.fill_rectangle(from.0+shift_x_count-1,from.1+shift_y_count,thickness,from.1+to.1-actual_height,color)?;
            }

        } else if to.0 < from.0 {
            // Left lopsided
        }

        Ok(())
    }

    pub fn point(&mut self,x:u32,y:u32,color: (u8,u8,u8,u8)) -> Result<(),ImageError> {
        if x >= self.1 {
            return Err(ImageError::new_simple(ErrorKind::OutOfRange(self.1-1,x)));
        }

        if y >= self.2 {
            return Err(ImageError::new_simple(ErrorKind::OutOfRange(self.2-1,y)));
        }

        let index = if x == 0 {
            (x*4+(y*self.2)*4) as usize
        }else {
            (x*4+(y*self.2)*4) as usize
        };

        self.0[index] = color.0;
        self.0[index+1] = color.1;
        self.0[index+2] = color.2;
        self.0[index+3] = color.3;

        Ok(())
    }

    pub fn fill_rectangle(&mut self, x:u32, y:u32, width:u32, height:u32, color: (u8, u8, u8, u8)) -> Result<(),ImageError>{
        let mut xc = 0;
        let mut yc = 0;

        if width == 0 || height == 0 {
            return Err(ImageError::new_simple(ErrorKind::InvalidValue(ImageType::Png)));
        }

        let mut vec = vec![];

        for i in 0..((self.1*self.2)*4) {
            if xc >= x && xc <= (width+x) {
                if yc >= y && yc <= (height+y)-1 {
                    vec.push((xc,yc,color));
                }
            }
            xc += 1;
            if xc == self.1*4 {
                yc += 1;
                xc = 0;
            }
        }

        for i in vec {
            match self.point(i.0,i.1,i.2) {
                Ok(_) => {}
                Err(err) => {
                    error!("{}",err);
                }
            }
        }

        Ok(())
    }

    pub fn draw_rectangle(&mut self, x:u32, y:u32, width:u32, height:u32, thickness: u32, color: (u8, u8, u8, u8)) -> Result<(),ImageError>{
        if width == 0 || height == 0 || thickness == 0 {
            return Err(ImageError::new_simple(ErrorKind::InvalidValue(ImageType::Png)));
        }

        if thickness >= width {
            return Err(ImageError::new_simple(ErrorKind::InvalidValue(ImageType::Png)));
        }

        self.fill_rectangle(x,y,width,thickness,color)?;

        self.fill_rectangle(x,y,thickness,height,color)?;

        self.fill_rectangle(x+width-thickness,y,thickness,height,color)?;

        self.fill_rectangle(x,y+height-thickness,width,thickness,color)?;

        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}