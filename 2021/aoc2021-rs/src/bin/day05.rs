fn parse_line_info(line: &str) -> ((isize, isize), (isize, isize))
{
    let pt = line.split(" -> ").map(|pt_str| {
        pt_str.split(",").map(str::parse::<isize>).map(Result::unwrap).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    ((pt[0][0], pt[0][1]), (pt[1][0], pt[1][1]))
}
fn main()
{
    // const DEMO_INPUT: &str = "0,9 -> 5,9
    // 8,0 -> 0,8
    // 9,4 -> 3,4
    // 2,2 -> 2,1
    // 7,0 -> 7,4
    // 6,4 -> 2,0
    // 0,9 -> 2,9
    // 3,4 -> 1,4
    // 0,0 -> 8,8
    // 5,5 -> 8,2";
    // let input = DEMO_INPUT.to_owned();
    let input = include_str!("../../../inputs/day05.txt");
    
    let points = input.lines()
                                            .map(parse_line_info)
                                            .collect::<Vec<_>>();


    // Cosider only horizontal or vertical lines in this case 
    let part1_points = points.iter().filter(|(pt1, pt2)| {
        pt1.0 == pt2.0 || pt1.1 == pt2.1
    }).collect::<Vec<_>>();

    let mut map = nalgebra::DMatrix::<u8>::zeros(1000, 1000);

    for pt in part1_points {
        let ((y1, x1), (y2, x2)) = *pt;
        let x1 = x1 as usize;
        let x2 = x2 as usize;
        let y1 = y1 as usize;
        let y2 = y2 as usize;
        // Horizontal line
        if x1 == x2
        {
            if y1 > y2
            {
                map.index_mut((x1, y2..(y1+1))).add_scalar_mut(1);
            }else{
                map.index_mut((x1, y1..(y2+1))).add_scalar_mut(1);
            }
        }else if y1 == y2{  // Vertical line
            if x1 > x2{
                map.index_mut((x2..(x1+1), y1)).add_scalar_mut(1);
            }else{
                map.index_mut((x1..(x2+1), y1)).add_scalar_mut(1);
            }
            
        }
    }
    let sol = map.iter().filter(|&&v| v >= 2).count();
    println!("Part 1: {}", sol);

    map.fill(0);
    for pt in points.clone() {
        let ((y1, x1), (y2, x2)) = pt;

        let mut dx = x2 - x1;
        if dx != 0
        {
            dx = dx/dx.abs();
        }
        let mut dy = y2 - y1;
        if dy != 0
        {
            dy = dy/dy.abs();
        }
        let mut x = x1-dx;
        let mut y = y1-dy;
        loop
        {            
            if x != x2
            {
                x = (x + dx).clamp(0, 1000);
            }
            if y != y2{
                y = (y + dy).clamp(0, 1000);
            }
            map[(x as usize, y as usize)] += 1;
            if x == x2 && y == y2
            {
                break;
            }
        }
    }

    let sol = map.iter().filter(|&&v| v >= 2).count();
    println!("Part 2 : {}", sol);
}
