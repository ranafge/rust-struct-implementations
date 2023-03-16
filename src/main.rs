

fn main() {
  let weather_vec = vec![
    vec!["Dahaka", "cloudy", "sumy", "22"],
    vec!["Rangpur", "SUNY", "333"]
  ];

  for mut my_vec in weather_vec {
    println!("The city is {}", my_vec[0]);

    while let Some(information) = my_vec.pop()  {
        if let Ok(number) = information.parse::<i32>()  {
            println!("The number is {}", number)
        }
    }
  }
  
}