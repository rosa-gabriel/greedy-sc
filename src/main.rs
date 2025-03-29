use std::io;
use std::io::prelude::*;
use std::{collections::HashMap, fs};

fn main() {
    let file_content = fs::read_to_string("input.json").unwrap();

    let a: HashMap<String, HashMap<String, i32>> = serde_json::from_str(&file_content).unwrap();

    print!("Starter City: ");

    let stdin = io::stdin();

    io::stdout().flush().unwrap();
    let mut from = String::new();
    for l in stdin.lock().lines() {
        from = l.unwrap().trim().to_string();
        break;
    }

    print!("Target City: ");

    io::stdout().flush().unwrap();
    let mut to = String::new();
    for l in stdin.lock().lines() {
        to = l.unwrap().trim().to_string();
        break;
    }

    if !a.contains_key(&from) {
        println!("City does not exist");
        return;
    }

    a.get(&to).expect("Target city does not exist");

    let mut city_name = from.clone();
    let mut path_sum = 0;
    let mut city: &HashMap<String, i32> = a.get(&city_name).expect("Starter city does not exist");
    let mut visited_cities: HashMap<String, ()> = HashMap::new();

    visited_cities.insert(from, ());

    loop {
        let closest_city = city
            .iter()
            .filter(|x| !visited_cities.contains_key(x.0))
            .reduce(|a, b| if a.1 < b.1 { a } else { b });

        match closest_city {
            Some(closest_city) => {
                println!("{} -> {} - Distance of {}Km", city_name.clone(), closest_city.0, closest_city.1);

                visited_cities.insert(closest_city.0.clone(), ());

                path_sum += closest_city.1;

                if *closest_city.0 == to {
                    println!("Target reached in {}Km", path_sum);
                    return;
                }

                city = a.get(closest_city.0).expect("Closest city does not contain any neighbor cities");
                city_name = closest_city.0.to_string();
            }
            None => {
                println!("Path not found");
                return;
            }
        };
    }
}
