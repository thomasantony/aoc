use ::aoc2019::{parse_digits, read_stdin};

type ImageLayer = Vec<Vec<u32>>;

#[derive(Debug, PartialEq)]
pub struct Image {
    pub image_data: Vec<u32>,
    pub layers: Vec<ImageLayer>,
    pub shape: (usize, usize)
}

impl Image {
    pub fn new(shape: (usize, usize), data: Vec<u32>) -> Self
    {
        let num_pixels = data.len();
        let num_pixels_in_layer = shape.0 * shape.1;
        assert!(num_pixels % num_pixels_in_layer == 0, "Invalid number of pixels");

        let mut layers = Vec::new();
        for layer_data in data.chunks(num_pixels_in_layer)
        {
            let rows = layer_data
                            .chunks(shape.0)
                            .map(Vec::from)
                            .collect();
            layers.push(rows);
        }
        Self {
            image_data: data,
            layers,
            shape
        }
    }
    pub fn layers<'a>(&'a self) -> impl Iterator<Item=Layer<'a>>
    {
        let layer_data_size = self.shape.0 * self.shape.1;
        self.image_data.chunks(layer_data_size)
    }
}

type Layer<'a> = &'a [u32];

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
                                layers: vec![vec![vec![1,2,3], vec![4,5,6]],
                                             vec![vec![7,8,9], vec![0,1,2]]
                                ],
                                shape: (3, 2)});

        image.layers().map(|layer| {

        })
    }
}