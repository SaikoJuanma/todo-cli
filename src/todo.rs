#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn to_line(&self) -> String {
        format!("{}|{}|{}", self.id, self.title, self.done)
    }

    pub fn from_line(line: &str) -> Option<Todo> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Todo {
            id: parts[0].parse().ok()?,
            title: parts[1].to_string(),
            done: parts[2] == "true",
        })
    }
}
