fn main() {
    let data = include_str!("input.txt");

    let mut table = [[0u8; 1000]; 1000];

    let processed_lines = data.lines().map(|line| {
        let mut split_line = line.split('@');
        let id = split_line.next().unwrap();
        let id_removed = split_line.next().unwrap();
        let mut split = id_removed.split(':');
        let coords_string = split.next().unwrap().trim();
        let dimensions_string = split.next().unwrap().trim();
        let mut coord_split = coords_string.split(',');
        let coord_x: usize = coord_split.next().unwrap().trim().parse().unwrap();
        let coord_y: usize = coord_split.next().unwrap().trim().parse().unwrap();
        let mut dimensions_split = dimensions_string.split('x');
        let dimension_x: usize = dimensions_split.next().unwrap().trim().parse().unwrap();
        let dimension_y: usize = dimensions_split.next().unwrap().trim().parse().unwrap();
        return (coord_x, coord_y, dimension_x, dimension_y);
    });

    processed_lines.clone().for_each(|line| {
        let (coord_x, coord_y, dimension_x, dimension_y) = line;

        println!("({},{}), ({},{})", coord_x, coord_y, dimension_x, dimension_y);

        for x in 0..dimension_x {
            for y in 0..dimension_y {
                table[coord_x + x][coord_y+y] += 1;
            }
        }
    });

    let contested_cells: usize = table.iter().map(|row| {
        return row.iter()
            .filter(|cell| cell > &&1u8)
            .count();
    }).sum();

    println!("{}", contested_cells);
}