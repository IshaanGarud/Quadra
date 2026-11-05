use image::{ImageReader, DynamicImage};
use image::{ GenericImage, ImageBuffer, Pixel, Rgb, imageops::FilterType::Lanczos3};

const SIZE: (u32, u32) = (512, 512);
const THRESHOLD: f32 = 1000.0;

const SOURCE_IMAGE: &str = "images/mona_lisa.png";
const OUTPUT_IMAGE: &str = "images/OUTPUT.png";

#[derive(Debug)]
struct Chunk {
    pub width: u32,
    pub height: u32,
    
    pub position: (u32, u32),
    pub avg_color: [u8 ; 3],
    pub mse: f32,
    pub children: Option<Box<[Chunk; 4]>>
}

impl Chunk {
    fn calculate_average_source(&mut self, source_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
        // ? Average color -> Average of the chunk of the ORIGINAL IMAGE.
        let mut color_sum: [u32; 3] = [0, 0, 0];
        let mut n: u32 = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_color =  source_buffer.get_pixel(
                    self.position.0 + x,
                    self.position.1 + y
                ).0;
                color_sum[0] += pixel_color[0] as u32;
                color_sum[1] += pixel_color[1] as u32;
                color_sum[2] += pixel_color[2] as u32;

                n += 1;
            }
        }

        self.avg_color = [
            (color_sum[0]/n) as u8,
            (color_sum[1]/n) as u8,
            (color_sum[2]/n) as u8
        ];

    }

    fn calculate_mse(&mut self, source_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    //? MSE -> Sum of square of the difference between chunk_pixel_color (source wala) and the average.

        let mut sum_of_di: i64 = 0;
        let mut n: u32 = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                n += 1;
                
                let pixel_color =  source_buffer.get_pixel(
                    self.position.0 + x,
                    self.position.1 + y
                ).0;

                // D_i = (Ri - R_avg)^2 + (Gi - G_avg)^2 + (Bi - B_avg)^2
                // MSE = [ Sigma(Di) ] / 3n

                let di: i64 = ( pixel_color[0] as i64 - self.avg_color[0] as i64 ).pow(2) 
                        + ( pixel_color[1] as i64 - self.avg_color[1] as i64 ).pow(2)
                        + (pixel_color[2] as i64 - self.avg_color[2]  as i64).pow(2);
                sum_of_di += di;
            }
        }

        self.mse = (sum_of_di as f32) / (3.0 * n as f32);
    }


    fn split_chunk(&mut self, source_buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
        // Split
        self.calculate_average_source(source_buffer);   // sets self.avg_color
        self.calculate_mse(source_buffer);              // sets self.mse

        if self.mse > THRESHOLD {    // && (self.width > 8 && self.height > 8)
            let quadrant_size: (u32, u32) = (self.width/2 , self.height/2);
            let quadrant_offset: (u32, u32) = self.position; 

            // Top Left
            let mut q1 = Chunk {
                width: quadrant_size.0,
                height:  quadrant_size.1,
                position: quadrant_offset,
                avg_color: [200, 0, 200],
                mse: 0.0,
                children: None
            };
            q1.split_chunk(source_buffer);
             

            // Top Right 
            let mut q2 = Chunk {
                width: quadrant_size.0,
                height:  quadrant_size.1,
                position: (
                    quadrant_offset.0 + quadrant_size.0,
                    quadrant_offset.1
                ),
                avg_color: [0, 0, 0],
                mse: 0.0,
                children: None
            };
            q2.split_chunk(source_buffer);

            // Bottom Left
            let mut q3 = Chunk {
                width: quadrant_size.0,
                height:  quadrant_size.1,
                position: (
                    quadrant_offset.0,
                    quadrant_offset.1 + quadrant_size.1
                ),
                avg_color: [0, 0, 0],
                mse: 0.0,
                children: None
            };
            q3.split_chunk(source_buffer);

            // Bottom Right
            let mut q4 = Chunk {
                width: quadrant_size.0,
                height:  quadrant_size.1,
                position: (
                    quadrant_offset.0 + quadrant_size.0,
                    quadrant_offset.1 + quadrant_size.1
                ),
                avg_color: [0, 0, 0],
                mse: 0.0,
                children: None
            };
            q4.split_chunk(source_buffer);


            let children =  Some(
                Box::new([
                    q1,
                    q2,
                    q3,
                    q4
                ])
            );
            self.children = children;

        }
        else {
            self.children = None;
        }

    }

    fn to_dynamic_image(&self, output_image: &mut DynamicImage) {
        
        match &self.children {
            None => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        // println!("{:?}", Rgb(self.avg_color).to_rgba());
                        output_image.put_pixel(
                            x + self.position.0, 
                            y + self.position.1, 
                            Rgb(self.avg_color).to_rgba()
                        );
                    }
                }
            }
            Some(quadrants) => {
                for quad in quadrants {
                    quad.to_dynamic_image(output_image);
                }
            }
        }
    
    
    }

}




fn main() {
    let input_image_buffer = match ImageReader::open(SOURCE_IMAGE) {
        Ok(img_bufreader) => img_bufreader.decode().unwrap(),
        Err(e) => panic!("{}",
            format!("Error :- {} \t target image not found!", e)
        )
    }
    .resize_exact(SIZE.0, SIZE.1, Lanczos3)
    .to_rgb8();


    let mut main_chunk = Chunk {
        width: input_image_buffer.width(),
        height: input_image_buffer.height(),
        position: (0, 0),
        avg_color: [0, 0, 0],       // default
        mse: 0.0,
        children: None
    };

    main_chunk.split_chunk(&input_image_buffer);
    
    // dbg!("{:?}", &main_chunk);

    let mut output_buffer = DynamicImage::new_rgba8(SIZE.0, SIZE.1);
    
    main_chunk.to_dynamic_image(&mut output_buffer);
    
    let _ = output_buffer.save(OUTPUT_IMAGE);
}