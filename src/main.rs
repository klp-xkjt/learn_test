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
        println!("1.Show the number of workers.\n2.New a worker.\nNo.Exit it.");
        /*
            1.Show the number of workers.
            2.New a worker.
            No.Exit it.
        */
        io::stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "1" => println!("Number: {}", get_workers_number(1, &company_staff)),
            "2" => {
                
            }
            "No" => {
                println!("You exit the program.");
                std::process::exit(0);
            },
            _ => println!("Sorry, try to input others")
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

fn get_workers_number(get_type: u32, workers: &HashMap<String, Worker>) -> usize {
    match get_type {
        1 => {
            let num_of_workers: usize = workers.len();
            num_of_workers
        },
        _ => 0
    }
}