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

    // HTMLに合わせた初期データを作成するメソッドを追加
    pub fn with_demo_data() -> Self {
        let mut board = Self::new();
        
        // ステートのIDを保存
        let todo_id = board.states[0].id.clone();
        let inprogress_id = board.states[1].id.clone();
        let done_id = board.states[2].id.clone();
        
        // Story 1: User Authentication
        let auth_story = Story::new("User Authentication".to_string());
        let auth_story_id = auth_story.id.clone();
        board.add_story(auth_story);
        
        // Story 2: Dashboard
        let dashboard_story = Story::new("Dashboard".to_string());
        let dashboard_story_id = dashboard_story.id.clone();
        board.add_story(dashboard_story);
        
        // Story 3: Settings Page
        let settings_story = Story::new("Settings Page".to_string());
        let _settings_story_id = settings_story.id.clone();
        board.add_story(settings_story);
        
        // User Authentication tasks
        let task1 = Task::new(
            "Design login screen".to_string(),
            "Create mockups for login, registration and password reset".to_string()
        );
        board.add_task(task1, &auth_story_id, &todo_id);
        
        let task2 = Task::new(
            "API endpoints".to_string(),
            "Implement REST API for auth operations".to_string()
        );
        board.add_task(task2, &auth_story_id, &todo_id);
        
        let task3 = Task::new(
            "JWT implementation".to_string(),
            "Add JWT token generation and validation".to_string()
        );
        board.add_task(task3, &auth_story_id, &inprogress_id);
        
        // Dashboard tasks
        let task4 = Task::new(
            "Chart components".to_string(),
            "Create reusable chart components".to_string()
        );
        board.add_task(task4, &dashboard_story_id, &todo_id);
        
        let task5 = Task::new(
            "Layout design".to_string(),
            "Design responsive dashboard layout".to_string()
        );
        board.add_task(task5, &dashboard_story_id, &done_id);
        
        board
    }
}
