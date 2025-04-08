use leptos::prelude::*;
mod models;
use models::*;

#[component]
fn App() -> impl IntoView {
    let (kanban, _) = create_signal(KanbanBoard::with_demo_data());

    view! {
        <div class="kanban-board">
            <h1>"Kanban Board"</h1>
            
            <div class="controls">
                <button>"Add New Story"</button>
            </div>
            
            <div class="kanban-grid">
                // ヘッダー行
                <div class="header-row">
                    <div class="corner-cell">"Stories / States"</div>
                    {move || {
                        let board = kanban.get();
                        board.states.iter().map(|state| {
                            view! {
                                <div class="state-header">{state.name.clone()}</div>
                            }
                        }).collect_view()
                    }}
                </div>
                
                // ストーリー行
                {move || {
                    let board = kanban.get();
                    board.stories.iter().map(|story| {
                        let story_id = story.id.clone();
                        view! {
                            <div class="story-row">
                                <div class="story-cell">
                                    <div class="story-title">{story.title.clone()}</div>
                                    <span class="add-button">"+"</span>
                                </div>
                                
                                // 各ステートのセル
                                {board.states.iter().map(|state| {
                                    let state_id = state.id.clone();
                                    let state_name = state.name.clone().to_lowercase().replace(" ", "");
                                    let class = format!("task-cell state-{}", state_name);
                                    
                                    // このストーリーとステートに対応するタスクを取得
                                    let tasks = board.get_tasks_by_position(&story_id, &state_id);
                                    
                                    view! {
                                        <div class={class}>
                                            // タスクカード
                                            {tasks.iter().map(|task| {
                                                view! {
                                                    <div class="task-card">
                                                        <div class="task-title">{task.title.clone()}</div>
                                                        <div class="task-description">{task.description.clone()}</div>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
