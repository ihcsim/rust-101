use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Select an option:");
        println!("1. Add employee to department");
        println!("2. View departments directory");
        println!("3. Group employees by department");

        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("failed to read input");

        let opt = opt.trim();
        match opt {
            "1" => {
                let employee = input_employee();
                let department = input_department();
                add_employee(&employee, &department, &mut departments);
            }

            "2" => {
                let department = input_department();
                println!("employees in {}:", department);
                for employee in list_department(&department, &departments) {
                    println!("- {}", employee);
                }
            }

            "3" => {
                for (employee, department) in group_employees(&departments) {
                    println!("- {} ({})", employee, department);
                }
            }

            _ => println!("unrecognized option {}", opt),
        }
    }
}

fn input_employee() -> String {
    println!("enter employee name");
    let mut employee = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("failed to read employee name");
    String::from(employee.trim())
}

fn input_department() -> String {
    println!("enter department name");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("failed to read department name");
    String::from(department.trim())
}

fn add_employee(
    employee_name: &str,
    department: &str,
    departments: &mut HashMap<String, Vec<String>>,
) {
    let v = departments
        .entry(String::from(department))
        .or_insert(Vec::new());
    (*v).push(String::from(employee_name));
}

fn list_department(department: &str, departments: &HashMap<String, Vec<String>>) -> Vec<String> {
    match departments.get(&String::from(department)) {
        Some(v) => v.to_vec(),
        None => Vec::new(),
    }
}

fn group_employees(departments: &HashMap<String, Vec<String>>) -> HashMap<&String, &String> {
    let mut r = HashMap::new();
    for (department, employees) in departments {
        for employee in employees {
            r.insert(employee, department);
        }
    }

    r
}
