use utils::read_lines;

fn main() {
    let lines:Vec<String> = read_lines("input.txt").unwrap();

    let mut elves:Vec<u32> = Vec::new();

    let mut acc:u32 = 0;

    for line in lines{
        println!("{:?}", line);
        if line.len() > 0{
            acc += line.as_str().parse::<u32>().unwrap()
        }
        else{
            elves.push(acc);
            acc = 0;
        }
    }

    elves.sort();
    elves.reverse();

    println!("Part1 : {}", elves[0]);

    println!("Part2 : {}", elves[0] + elves[1] + elves[2])





}
