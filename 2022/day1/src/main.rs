fn main() {
    let input = std::str::from_utf8(include_bytes!("../input.txt")).unwrap();
    let separated = input.split("\n\n");
    let mut max = 0;
    let mut second = 0;
    let mut third = 0;
    for elf in separated {
        let items = elf.split('\n');
        let mut subtotal = 0;
        for item in items {
            if !item.is_empty() {
                subtotal += item.parse::<isize>().unwrap();
            }
        }
        if subtotal >= max {
            third = second;
            second = max;
            max = subtotal;
        } else if subtotal >= second {
            third = second;
            second = subtotal;
        } else if subtotal > third {
            third = subtotal;
        }
    }
    println!("Part 1: {}", max);
    println!("Part 2: {}", max + second + third);
}
