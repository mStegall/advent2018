use itertools::Itertools;
use regex::Regex;

pub fn main() {
    let data = include_str!("input.txt");

    let (players_count, last_marble): (i32, i32) =
        Regex::new(r"(\d+) players; last marble is worth (\d+) points")
            .unwrap()
            .captures(data.lines().next().unwrap())
            .unwrap()
            .iter()
            .skip(1)
            .filter_map(|s| s?.as_str().parse::<i32>().ok())
            .collect_tuple()
            .unwrap();

    let mut board: Vec<i32> = vec!(0);
    let mut pos:usize = 0;
    let mut players: Vec<i32> = vec!();
    for _ in 0 .. players_count{
        players.push(0);
    } 

    println!("{last_marble}");
    for i in 1 ..(last_marble + 1 ){
        
        if i % 23 == 0 {
            let mut next_pos:isize = pos as isize - 7;
            if next_pos < 0 {
                next_pos += board.len() as isize;
            }
            pos = next_pos.try_into().unwrap();
            let score = board.remove(pos);
            let player_index = (i  - 1) % players_count;
            if let Some(player_score) = players.get_mut(player_index as usize){
                *player_score += score + i;
            }
        } else {
            let insert_pos = (pos + 2) % board.len();
            board.insert(insert_pos, i);
            pos = insert_pos;
        }

        // println!("{} {} {board:?}",i, (i  - 1) % players_count + 1 );

    }

    println!("{:?}",players.into_iter().max() )
}
