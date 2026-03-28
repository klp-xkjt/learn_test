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
    fn new_a_worker(name: String, gender: Genders, edu: EduBackground, date: String) -> Result<Self, String> {
        if Self::worker_test(&edu) {
            Ok(Worker {
                name,
                gender,
                edu,
                date_of_induction: date,
                is_working: true
            })
        } else {
            Err(format!("The worker is not enough to new because of educational background."))
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

    let w1: Result<Worker, String> = Worker::new_a_worker(
        "Alice".to_string(),
        Genders::Female,
        EduBackground::University,
        "2023-01-15".to_string(),
    );
    
    match w1 {
        Ok(worker) => {
            company_staff.insert(worker.name.clone(), worker);
            println!("Alice initialized successfully.");
        }
        Err(e) => println!("{}", e)
    }

    loop {
        let mut input: String = String::new();
        
        println!("\n--- Worker Management System ---");
        println!("1. Show the number of workers.");
        println!("2. New a worker.");
        println!("3. Show information of workers"); 
        println!("4. Resign a worker.");
        println!("No. Exit program.");
        println!("Please input your choice:");

        io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim() {
            "1" => {
                println!("Number of workers: {}", company_staff.len());
            }
            "2" => {
                match create_input() {
                    Ok(new_worker) => {
                        company_staff.insert(new_worker.name.clone(), new_worker);
                        println!("Worker added successfully!");
                    }
                    Err(e) => {
                        println!("Failed to create worker: {}", e);
                    }
                }
            }
            "3" => {
                if company_staff.is_empty() {
                    println!("No workers in the system.");
                } else {
                    println!("All Workers Information List:");
                    for (name, worker) in &company_staff {
                        println!("{}: {}", name, get_worker_info(worker));
                    }
                }
            }
            "4" => {
                if company_staff.is_empty() {
                    println!("No workers to resign.");
                    continue;
                }

                println!("Current workers: {:?}", company_staff.keys());
                let mut resign_name: String = String::new();
                println!("Which worker do you want to resign? (Enter name): ");
                io::stdin().read_line(&mut resign_name).expect("Error");

                let resign_name: &str = resign_name.trim();
                if let Some(worker) = company_staff.get_mut(resign_name) {
                    if worker.is_working {
                        worker.resign();
                        println!("{} has resigned successfully.", worker.name);
                    } else {
                        println!("{} has already resigned.", worker.name);
                    }
                } else {
                    println!("Error: Worker '{}' not found.", resign_name);
                }
            }
            "No" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
        println!("---------------------");
    }
}

fn get_worker_info(worker: &Worker) -> String {
    let gender: &str = match &worker.gender {
        Genders::Male => "Male",
        Genders::Female => "Female",
        Genders::Others(a) => a.as_str(),
    };
    
    let edu: &str = match &worker.edu {
        EduBackground::Senior => "Senior",
        EduBackground::University => "University",
        EduBackground::Higher => "Higher level",
        EduBackground::Primary => "Primary",
        EduBackground::Junior => "Junior",
    };
    
    let status: &str = if worker.is_working { "On Duty" } else { "Resigned" };

    format!(
        "Gender: {}, Education: {}, Induction: {}, Status: {}",
        gender, edu, worker.date_of_induction, status
    )
}

fn create_input() -> Result<Worker, String> {
    let mut name_input: String = String::new();
    println!("Enter name: ");
    io::stdin().read_line(&mut name_input).map_err(|_| "IO Error")?;
    let name: String = name_input.trim().to_string();
    
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let mut gender_input: String = String::new();
    println!("Gender: Male/Female/Others");
    io::stdin().read_line(&mut gender_input).map_err(|_| "IO Error")?;
    let gender: Genders = match gender_input.trim() {
        "Male" => Genders::Male,
        "Female" => Genders::Female,
        "Others" => {
            let mut other_desc: String = String::new();
            println!("Your gender description: ");
            io::stdin().read_line(&mut other_desc).map_err(|_| "IO Error")?;
            Genders::Others(other_desc.trim().to_string())
        }
        _ => {
            println!("Invalid value. Gender is set to Male.");
            Genders::Male
        }
    };

    let mut edu_input: String = String::new();
    println!("Educational background: Primary/Junior/Senior/University/Higher");
    io::stdin().read_line(&mut edu_input).map_err(|_| "IO Error")?;
    let edu: EduBackground = match edu_input.trim() {
        "Primary" => EduBackground::Primary,
        "Junior" => EduBackground::Junior,
        "Senior" => EduBackground::Senior,
        "University" => EduBackground::University,
        "Higher" => EduBackground::Higher,
        _ => {
            println!("Invalid value. Educational background is set to Senior.");
            EduBackground::Senior
        }
    };

    let mut date: String = String::new();
    println!("Date of induction (YYYY-MM-DD): ");
    io::stdin().read_line(&mut date).map_err(|_| "IO Error")?;

    Worker::new_a_worker(name, gender, edu, date.trim().to_string())
}