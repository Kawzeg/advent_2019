use std::io;

const width: usize = 25;
const height: usize = 6;

fn copy_from(image: &[u32]) -> [u32;width] {
    let mut arr = [0;width];
    let bytes = &image[..arr.len()];
    arr.copy_from_slice(bytes);
    arr
}

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = std::fs::read_to_string(file)?;
    let mut layers: Vec<Vec<u32>> = vec![];

    let mut layer: Vec<u32> = vec![];

    for c in input.trim().chars() {
        layer.push(c.to_digit(10).unwrap());
        if layer.len() == width*height {
            println!("Got layer {:?}", layer);
            layers.push(layer);
            layer = vec![];
        }
    }

    let mut min_zeroes = std::usize::MAX;
    let mut target_layer = 0;
    for (i, layer) in layers.iter().enumerate() {
        println!("layer is {:?}", layer);
        let zeroes = layer.iter().filter(|x|{**x==0}).count();
        println!("Has {} zeroes", zeroes);
        if zeroes < min_zeroes {
            target_layer = i;
            min_zeroes = zeroes;
        }
    }

    println!("Found layer {:?}", layers[target_layer]);

    let target = &layers[target_layer];
    let ones = target.iter().filter(|x|{**x==1}).count();
    let twos = target.iter().filter(|x|{**x==2}).count();
    println!("Solution: {}*{}={}", ones, twos, ones*twos);

    let mut image : Vec<u32> = vec![];
    for i in 0..width*height {
        let mut pixel = 2;
        for layer in &layers {
            if layer[i] == 2 {
                continue;
            } else {
                pixel = layer[i];
                break;
            }
        }
        image.push(pixel);
    }

    for x in 0..height {
        let row: [u32;width] = copy_from(&image[x*width..x*width+width]);
        for c in row.iter() {
            if *c == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        print!("\n");
    }

    Ok(())
}