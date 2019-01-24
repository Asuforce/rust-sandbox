#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    state: State,
}

struct State {
    tasks: Vec<Task>,
}

struct Task {
    name: String,
    assignee: String,
    mandays: u32,
    status: u32,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state: State {
                tasks: vec! [
                    Task { name: "Task 1".to_string(), assignee: "🐱".to_string(), mandays: 3, status: 1},
                    Task { name: "Task 2".to_string(), assignee: "🐶".to_string(), mandays: 2, status: 1},
                    Task { name: "Task 3".to_string(), assignee: "🐭".to_string(), mandays: 1, status: 2},
                    Task { name: "Task 4".to_string(), assignee: "🐹".to_string(), mandays: 3, status: 3},
                ]
            }
        }
    }

    fn update (&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section", id="board",>
                <div class="container",>
                    <div class="columns",>
                        { view_column(1, "未対応") }
                        { view_column(2, "対応中") }
                        { view_column(3, "完了") }
                    </div>
                </div>
            </section>
        }
    }
}

fn view_column(status: u32, status_text: &str) -> Html<Model> {
    html! {
        <div class=format!("column status-{}", status),>
            <div class="tags has-addons",>
                <span class="tag",>{ status_text }</span>
                <span class="tag is-dark",>{ 0 }</span>
            </div>
        </div>
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
