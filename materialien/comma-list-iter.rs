use std::str::FromStr;

fn main() {
    let res = parse_list("1, 2, 3; , ,7, 8,9; 6;3,5;");
    println!("{:#?}", res);
    let res = parse_list("1, peter, 3, 4");
    println!("{:#?}", res);
}

// "1, 2, 3; 7, 8,9; 6;3,5;"
fn parse_list(input: &str)
    -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err>
{
    // The following code could be expressed even shorter, but this is the
    // exact code as shown in the lecture.
    input.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse())
                .collect()
        })
        .collect()
}
