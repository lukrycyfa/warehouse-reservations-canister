use std::collections::HashMap;

#[derive(Debug, Clone)]
struct EducationHistory {
    institution: String,
    degree: String,
    update_at: u64,
}

#[derive(Debug, Clone)]
struct WorkHistory {
    company: String,
    position: String,
    update_at: u64,
}

struct User {
    education_history: HashMap<String, EducationHistory>,
    work_history: HashMap<String, WorkHistory>,
}

impl User {
    fn add_education_history(&mut self, key: String, institution: String, degree: String) {
        let history = EducationHistory {
            institution,
            degree,
            update_at: get_current_time(),
        };
        self.education_history.insert(key, history);
    }

    fn update_education_history(&mut self, key: &str, institution: String, degree: String) {
        if let Some(history) = self.education_history.get_mut(key) {
            history.institution = institution;
            history.degree = degree;
            history.update_at = get_current_time();
        }
    }

    fn delete_education_history(&mut self, key: &str) {
        self.education_history.remove(key);
    }

    fn add_work_history(&mut self, key: String, company: String, position: String) {
        let history = WorkHistory {
            company,
            position,
            update_at: get_current_time(),
        };
        self.work_history.insert(key, history);
    }

    fn update_work_history(&mut self, key: &str, company: String, position: String) {
        if let Some(history) = self.work_history.get_mut(key) {
            history.company = company;
            history.position = position;
            history.update_at = get_current_time();
        }
    }

    fn delete_work_history(&mut self, key: &str) {
        self.work_history.remove(key);
    }

    fn get_work_history(&self, key: &str) -> Option<&WorkHistory> {
        self.work_history.get(key)
    }
}

// Helper function to get the current time
fn get_current_time() -> u64 {
    // Implement your logic to get the current time
    // For example, use the time crate or another appropriate method
    0
}

// Authorization function (example, replace with your actual implementation)
fn is_authorized(user_id: &str, caller_id: &str) -> bool {
    // Implement your authorization logic here
    // For example, check if the caller has the necessary permissions for the specified user
    user_id == caller_id
}

fn main() {
    let mut user = User {
        education_history: HashMap::new(),
        work_history: HashMap::new(),
    };

    let user_id = "user123";
    let caller_id = "admin";

    // Example usage with authorization checks
    if is_authorized(user_id, caller_id) {
        user.add_education_history("edu1".to_string(), "University A".to_string(), "Bachelor".to_string());
        user.update_education_history("edu1", "University B".to_string(), "Master".to_string());
        user.delete_education_history("edu1");

        user.add_work_history("work1".to_string(), "Company X".to_string(), "Developer".to_string());
        user.update_work_history("work1", "Company Y".to_string(), "Senior Developer".to_string());
        user.delete_work_history("work1");

        if let Some(work_history) = user.get_work_history("work1") {
            println!("Work History: {:?}", work_history);
        }
    } else {
        println!("Unauthorized access!");
    }
}
