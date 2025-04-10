pub mod mall;
pub use guard::Guard;
pub use mall::floor::store::employee::Employee;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::*;
/// Returns the store with the largest square meters in the mall
pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| &floor.stores)
        .max_by_key(|store| store.square_meters)
        .cloned()
        .unwrap()
}

/// Returns a vector of employees with the highest salary in the mall
pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut highest_salary = 0.0;
    let mut highest_paid: Vec<Employee> = Vec::new();
    
    // First pass: find the highest salary
    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                }
            }
        }
    }
    
    // Second pass: collect all employees with that salary
    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if employee.salary == highest_salary {
                    highest_paid.push(employee.clone());
                }
            }
        }
    }
    
    highest_paid
}

/// Returns the total number of employees and guards in the mall
pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut count = mall.guards.len();
    
    for floor in &mall.floors {
        for store in &floor.stores {
            count += store.employees.len();
        }
    }
    
    count
}

/// Ensures there is at least 1 guard for every 200 square meters of floor size
/// If not, adds guards from the provided vector to meet the requirement
pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let mut g = guards.clone();
    let mut count = 0_usize;
    let s = mall
        .floors
        .iter()
        .flat_map(|floor| &floor.stores)
        .fold(0_u64, |acc, x| acc + x.square_meters);
    while count * 200 < s as usize && g.len() > 0 {
        count += 1;
        mall.guards.push(g[0].clone());
        g.remove(0);
    }
}

// 5. Adjust employee salaries based on working hours
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                // Calculate working hours
                let hours = employee.working_hours.1 - employee.working_hours.0;
                
                if hours > 10 {
                    // Raise salary by 10%
                    employee.salary *= 1.1;
                } else {
                    // Cut salary by 10%
                    employee.salary *= 0.9;
                }
            }
        }
    }
}