use std::collections::HashMap;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25 {
        let v = expensive_result.values(intensity).unwrap_or_else(|err| {
            eprintln!("failed to generate workout plan: {}", err);
            process::exit(1);
        });
        println!("Today, do {} pushups!", v);
        println!("Next, do {} situps!", v);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            let v = expensive_result.values(intensity).unwrap_or_else(|err| {
                eprintln!("failed to generate workout plan: {}", err);
                process::exit(1);
            });
            println!("Today, run for {} minutes!", v)
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, Option<u32>>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn values(&mut self, arg: u32) -> Result<u32, &str> {
        match self.values.get(&arg) {
            Some(v) => match v {
                Some(u) => Ok(*u),
                None => Err("expected value to be present"),
            },
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, Some(v));
                Ok(v)
            }
        }
    }
}
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let _v1 = c.values(1);
        let v2 = c.values(2).unwrap();
        assert_eq!(v2, 2)
    }
}
