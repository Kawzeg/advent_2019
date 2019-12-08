use std::io;



fn main() -> io::Result<()> {
    let width = 25;
    let height = 6;
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

    Ok(())
}