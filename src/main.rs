fn main() {
    use std::fs;

    let data = fs::read_to_string("/home/rana/name.txt").expect("Unable to read the file");

    let result = data.split("\n").map(|i| i.trim()).collect::<Vec<&str>>();
    // println!("Result {:?}", result);
    let mut ans: Vec<String> = vec![];
    for i in 0..result.len() {

        if result[i].contains(",") {
            let pair = result[i]
                .split(",")
                .map(|l| l.trim())
                .collect::<Vec<&str>>();
                ans.push(pair[0].to_owned());
                ans.push(pair[1].to_owned());
        }else if result[i].len() >= 1{
            ans.push(result[i].to_string());
        }
    }
    println!("{ans:?}");
}
