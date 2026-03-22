use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, PartialEq)]
enum Genders {
    Male,
    Female,
    Others(String)
}
#[derive(Debug, Clone, PartialEq)]
enum EduBackground {
    Primary,
    Junior,
    Senior,
    University,
    Higher
}

#[derive(Debug, Clone)]
struct Worker {
    name: String,
    gender: Genders,
    edu: EduBackground,
    date_of_induction: String, //YYYY-MM-DD
    is_working: bool
}

impl Worker {
    fn new_a_worker(name: String, gender: Genders, edu: EduBackground, date: String) -> Self {
        if Self::worker_test(&edu) {
            Worker {
                name,
                gender,
                edu,
                date_of_induction: date,
                is_working: true
            }
        } else {
            panic!("Not up to standard, the educational background must be above Junior.");
        }
    }
    fn worker_test(edu: &EduBackground) -> bool {
        match edu {
            EduBackground::Primary | EduBackground::Junior => false,
            _ => true
        }
    }

    fn resign(&mut self) {
        self.is_working = false;
    }
}

fn main() {
    let mut company_staff: HashMap<String, Worker> = HashMap::new();

    let w1: Worker = Worker::new_a_worker(
        "Alice".to_string(),
        Genders::Female,
        EduBackground::University,
        "2023-01-15".to_string(),
    );
    company_staff.insert(w1.name.clone(), w1);

    loop {
        let mut input: String = String::new();

        println!("Here is a worker management.What do you want to do?");
        println!("1. Show the number of workers.");
        println!("2. New a worker.");
        println!("3. Show information of workers"); 
        println!("4. Resign a worker.");
        println!("No. Exit it.");
        /*
            1.Show the number of workers.
            2.New a worker.
            No.Exit it.
        */
        io::stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "1" => {
                let worker_number = company_staff.len();
                println!("Number: {}", worker_number);
                println!("---------------------");
            },
            "2" => {
                let new_worker: Option<Worker> = create_input();
                if let Some(worker) = new_worker {
                    company_staff.insert(worker.name.clone(), worker);
                }
            },
            "3" => {
                println!("All Workers Information List");
                if company_staff.is_empty() {
                    println!("There's no workers yet.");
                    println!("---------------------");
                }

                for (k, v) in company_staff.iter() {
                    let info: String = get_worker_info(v);
                    print!("{} ", k);
                    println!("{}", info);
                    println!("---------------------");
                }
            },
            "4" => {
                let mut all_names: String = String::new();

                for (k, _) in company_staff.iter() {
                    all_names.push_str(k);
                    all_names.push(' ');
                }
                if all_names.is_empty() {
                    println!("No workers to resign.");
                } else {
                    println!("Current workers: {}", all_names);
                    println!("Which worker do you want to resign? (Enter name): ");

                    let mut resign_worker_input: String = String::new();
                    io::stdin().read_line(&mut resign_worker_input).expect("Error");
                    let resign_name: String = resign_worker_input.trim().to_string();

                    match company_staff.get_mut(&resign_name) {
                        Some(worker) => {
                            if worker.is_working {
                                worker.resign();
                                println!("{} has been resigned successfully.", worker.name);
                            } else {
                                println!("{} has already resigned.", worker.name);
                            }
                        },
                        None => {
                            println!("Error: Worker '{}' not found.", resign_name);
                        }
                    }
                }
            },
            "No" => {
                println!("You exit the program.");
                println!("---------------------");
                break;
            },
            _ => {
                println!("Sorry, try to input others");
                println!("---------------------");
            }
        }
    }
}

fn get_worker_info(worker: &Worker) -> String {
    let gender: String = match &worker.gender {
        Genders::Male => "Male".to_string(),
        Genders::Female => "Female".to_string(),
        Genders::Others(a) => a.to_string()
    };
    let edu: String = match &worker.edu {
        EduBackground::Senior => "Senior".to_string(),
        EduBackground::University => "University".to_string(),
        EduBackground::Higher => "Higher level".to_string(),
        _ => "Others".to_string()
    };
    let status: String = if worker.is_working {
        "On Duty".to_string()
    } else {
        "Resigned".to_string()
    };

    format!("Name: {}, Gender: {}, Educational background: {}, Induction date: {},Status: {}.", worker.name, gender, edu, worker.date_of_induction, status)
}

fn create_input() -> Option<Worker> {
    let mut name_input: String = String::new();
    println!("Name: ");
    io::stdin().read_line(&mut name_input).expect("Error");
    let name: String = name_input.trim().to_string();
    if name.is_empty() {
        println!("Please input a name.");
        return None;
    }

    let mut gender_input: String = String::new();
    println!("Gender: Male/Female/Others");
    io::stdin().read_line(&mut gender_input).expect("Error");
    let gender = match gender_input.trim() {
        "Male" => Genders::Male,
        "Female" => Genders::Female,
        "Others" => {
            let mut other_desc: String = String::new();
            println!("Your gender: ");
            io::stdin().read_line(&mut other_desc).expect("Error");
            Genders::Others(other_desc.trim().to_string())
        },
        _ => {
            println!("Invalid value.Gender is set to Male.");
            Genders::Male
        }
    };

    let mut edu_input: String = String::new();
    println!("Educational background: Primary/Junior/Senior/University/Higher");
    io::stdin().read_line(&mut edu_input).expect("Error");
    let edu = match edu_input.trim() {
        "Primary" => EduBackground::Primary,
        "Junior" => EduBackground::Junior,
        "Senior" => EduBackground::Senior,
        "University" => EduBackground::University,
        "Higher" => EduBackground::Higher,
        _ => {
            println!("Invalid value.Educational background is set to Senior.");
            EduBackground::Senior
        }
    };

    let mut date: String = String::new();
    println!("The date of induction: ");
    io::stdin().read_line(&mut date).expect("Error");

    println!("Create workers successfully.");
    Some(Worker::new_a_worker(name, gender, edu, date))
}