#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String, 
    pub next: Link,
}

impl WorkEnvironment {
    // Initializez a new WorkEnvironment with "grade" set to "None"
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    // Adds A ew worker to the beginning of the list.
    pub fn add_worker(&mut self, role: String, name: String) {
        // Create a new worker
        let new_worker = Worker {
            role, 
            name,
            next: self.grade.take(), // Take the current head
        };

        // Update the head to point to the new worker
        self.grade = Some(Box::new(new_worker));
    }

    // Removes the last worker that was added (the head of the list) and returns their name.
    pub fn remove_worker(&mut self) -> Option<String> {
        // Take current head
        self.grade.take().map(|worker|{
            self.grade = worker.next; // update the head to point to the next worker
            worker.name
        })
    } 

    // Returns a tuple containing the name and role of the last added worker (the head of the list)
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| {
        (worker.name.clone(), worker.role.clone())
        })
    }


}
