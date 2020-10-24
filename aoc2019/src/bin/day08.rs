use ::aoc2019::{parse_digits, read_stdin};

#[derive(Debug, PartialEq)]
pub struct Image {
    pub image_data: Vec<u32>,
    pub shape: (usize, usize),
    num_pixels_in_layer: usize
}

type Layer<'a> = &'a [u32];

impl Image {
    pub fn new(shape: (usize, usize), data: Vec<u32>) -> Self
    {
        let num_pixels = data.len();
        let num_pixels_in_layer = shape.0 * shape.1;
        assert!(num_pixels % num_pixels_in_layer == 0, "Invalid number of pixels");
        
        Self {
            image_data: data,
            shape,
            num_pixels_in_layer
        }
    }
    pub fn layers<'a>(&'a self) -> impl Iterator<Item=Layer<'a>>
    {
        self.image_data.chunks(self.num_pixels_in_layer)
    }

    pub fn decode(&self) -> Vec<Vec<u32>>
    {
        // Fill with transparent pixels initially
        let mut output_layer: Vec<u32> = vec![2; self.num_pixels_in_layer];
        
        for layer in self.layers() {
            for (i, &pixel) in layer.iter().enumerate() {
                // Output layer is transparent, so try to fill it
                if output_layer[i] == 2 && pixel != 2{
                    output_layer[i] = pixel;
                }
            }
        }
        output_layer.chunks(self.shape.0).map(Vec::from).collect()
    }
}


fn main() {
    let shape = (25, 6);
    
    let input = read_stdin();
    let data:Vec<u32> = parse_digits(&input).collect();
    let im = Image::new(shape, data);

    let layer_with_fewest_zeros = im.layers().min_by_key(|layer| 
        layer.iter().filter(|&&p|p==0).count()
    ).expect("Failed to find layer with fewest zeros");

    let number_of_ones = layer_with_fewest_zeros.iter().filter(|&&p|p==1).count();
    let number_of_twos = layer_with_fewest_zeros.iter().filter(|&&p|p==2).count();
    println!("Part A: {}", number_of_ones * number_of_twos);

    let decoded_image = im.decode();
    println!("Part B:");
    for row in decoded_image.iter()
    {
        for &pixel in row
        {
            let display = if pixel == 1 {
                '#'
            } else {
                ' '
            };
            print!("{}", display);
        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day08_input_parse()
    {
        let demo_input = "123456789012";
        let demo_shape = (3, 2);
        
        let pixels: Vec<u32> = parse_digits(demo_input).collect();
        let image = Image::new(demo_shape, pixels.clone());
        assert_eq!(image, Image{image_data: pixels, 
                                num_pixels_in_layer: 6,
                                shape: (3, 2)});
    }
    #[test]
    fn test_day08_decode()
    {
        let demo_input = "0222112222120000";
        let demo_shape = (2, 2);
        
        let pixels: Vec<u32> = parse_digits(demo_input).collect();
        let image = Image::new(demo_shape, pixels.clone());
        assert_eq!(image.decode(), vec![vec![0, 1], vec![1, 0]]);
    }
}