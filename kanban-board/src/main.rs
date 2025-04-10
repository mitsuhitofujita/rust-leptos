use leptos::prelude::*;
mod models;
use models::*;
use leptos::web_sys::DragEvent;


#[component]
fn App() -> impl IntoView {
    // 初期データを持つカンバンボードを作成
    let (kanban, set_kanban) = signal(KanbanBoard::with_demo_data());
    
    // タスクを移動するコールバック関数
    let move_task = move |task_id: String, state_id: String| {
        set_kanban.update(|board| {
            board.move_task(&task_id, &state_id);
        });
    };
    
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
                                    
                                    // ドロップイベントのためのステートIDをクローン
                                    let drop_state_id = state_id.clone();
                                    let move_task_callback = move_task;
                                    
                                    view! {
                                        <div 
                                            class={class}
                                            // ドラッグオーバー時のイベント
                                            on:dragover=move |ev: DragEvent| {
                                                ev.prevent_default();
                                            }
                                            // ドロップ時のイベント
                                            on:drop=move |ev: DragEvent| {
                                                ev.prevent_default();
                                                // データ転送オブジェクトからタスクIDを取得
                                                if let Some(data_transfer) = ev.data_transfer() {
                                                    if let Ok(task_id) = data_transfer.get_data("text/plain") {
                                                        move_task_callback(task_id, drop_state_id.clone());
                                                    }
                                                }
                                            }
                                        >
                                            // タスクカード
                                            {tasks.iter().map(|task| {
                                                let task_id = task.id.clone();
                                                
                                                view! {
                                                    <div 
                                                        class="task-card"
                                                        draggable="true"
                                                        // ドラッグ開始時のイベント
                                                        on:dragstart=move |ev: DragEvent| {
                                                            if let Some(data_transfer) = ev.data_transfer() {
                                                                // タスクIDをデータとして設定
                                                                let _ = data_transfer.set_data("text/plain", &task_id);
                                                            }
                                                        }
                                                    >
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
