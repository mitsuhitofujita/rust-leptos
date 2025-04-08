use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Story {
    pub id: String,
    pub title: String,
}

impl Story {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub name: String,
}

impl State {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KanbanBoard {
    pub states: Vec<State>,
    pub stories: Vec<Story>,
    pub tasks: Vec<Task>,
    pub task_positions: std::collections::HashMap<String, (String, String)>, // task_id -> (story_id, state_id)
}

impl KanbanBoard {
    pub fn new() -> Self {
        let states = vec![
            State::new("To Do".to_string()),
            State::new("In Progress".to_string()),
            State::new("Done".to_string()),
        ];
        
        Self {
            states,
            stories: Vec::new(),
            tasks: Vec::new(),
            task_positions: std::collections::HashMap::new(),
        }
    }

    pub fn add_story(&mut self, story: Story) {
        self.stories.push(story);
    }
    
    pub fn add_task(&mut self, task: Task, story_id: &str, state_id: &str) {
        self.task_positions.insert(task.id.clone(), (story_id.to_string(), state_id.to_string()));
        self.tasks.push(task);
    }
    
    pub fn move_task(&mut self, task_id: &str, state_id: &str) {
        if let Some((story_id, _)) = self.task_positions.get(task_id) {
            self.task_positions.insert(task_id.to_string(), (story_id.clone(), state_id.to_string()));
        }
    }

    pub fn get_tasks_by_position(&self, story_id: &str, state_id: &str) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| {
                if let Some((s_id, st_id)) = self.task_positions.get(&task.id) {
                    s_id == story_id && st_id == state_id
                } else {
                    false
                }
            })
            .collect()
    }
}
