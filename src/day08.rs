

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

  let input = std::fs::read_to_string("day08/input.txt")?;

  let width = 25;
  let height = 6;

  let input = input.chars().collect::<Vec<_>>();
  let rows = input.chunks_exact(width).collect::<Vec<&[char]>>();
  let layers = rows.chunks_exact(height).collect::<Vec<&[&[char]]>>();

  let (zeroes, fewest_zeroes_layer) = layers.iter().map(|layer| (count_value(layer, '0'), layer)).min().unwrap();

  let ones = count_value(fewest_zeroes_layer, '1');
  let twos = count_value(fewest_zeroes_layer, '2');

  let part1_ans = ones * twos;
  println!("{}", part1_ans);

  let mut final_image: Vec<Vec<char>> = (0..height).map(|_| (0..width).map(|_| '2').collect::<Vec<char>>()).collect();

  for layer in layers {
    for (y, row) in layer.iter().enumerate() {
      for (x, value) in row.iter().enumerate() {
        let prev_value = &mut final_image[y][x];
        if *prev_value == '2' {
          *prev_value = *value;
        }
      }
    }
  }

  print_layer(&final_image);

  Ok(())
}

fn count_value(layer: &[&[char]], value: char) -> usize {
  layer.iter().flat_map(|row| row.iter()).filter(|c| **c == value).count()
}

fn print_layer(layer: &[Vec<char>]) {
  for row in layer {
    let row_str = row.iter().map(|&c| if c == '1' { '█' } else { ' ' }).collect::<String>();
    println!("{}", row_str);
  }
}