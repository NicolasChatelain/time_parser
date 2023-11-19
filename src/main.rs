use std::collections::HashMap;
use std::fs;

fn main() {

    let path = "src/CSV/rust_times.csv";
    let contents = fs::read_to_string(path);
    let mut dates_and_times_map : HashMap<String, String> = HashMap::new();
    let mut time_lines = Vec::new();

    match contents {
        Ok(contents) => {

            for line in contents.lines() {

                time_lines.push(line.to_string());

            }

        }
        Err(_) => { println!("Something went wrong when reading the file.")}
    };


    for line in &time_lines {
        let parts: Vec<&str> = line.split(';').collect();

        if parts.len() == 2 {
            dates_and_times_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    println!("{:?}", dates_and_times_map.values());

    let mut hours = Vec::new();

    for time in dates_and_times_map.values_mut() {
        if time.contains(':'){
            hours.push(time.split_at(4).0);
        } else {
            hours.push(time.split_at(1).0);
        }
    }


    println!("{:?}", hours);

    let mut sum: i32 = 0;
    for hour in hours {

        if let Ok(hour_value) = hour.parse::<i32>() {
            sum += hour_value;
        } else {
            println!("Failed to parse the hour.");
        }
    }

    println!("Total hours: {}", sum);



}
