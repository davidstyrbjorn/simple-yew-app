use yew::prelude::*;

/*
*    To run you need to have wasm32 toolchain installed by running
*    "rustup target add wasm32-unknown-unknown"
*    You also need Trunk
*    "cargo install trunk"
*    then run "trunk serve"
*/

// Components are the building blocks in Yew (similar to rust)
// Components can build themselves into HTML and be rendered into the DOM

// Let's create a component to try this out!

// The types for message represents the type of messages our component can recieve
enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

// Life-cycle methods for our component
impl Component for CounterComponent {
    type Message = Msg;
    type Properties = (); // Describes the data passed from a parent component

    // Called when our component is created
    fn create(_ctx: &Context<Self>) -> Self {
        return Self { count: 0 };
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render component
            }
        }
        // We do not want to return true here because we dont ALWAYS want to re-render a component each update
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{self.count}</p>
                <button
                    onclick={link.callback(|_| Msg::AddOne)}
                >
                    {"ADD ONE"}
                </button>
            </div>
        }
    }
}

fn main() {
    // Start yew app, passing our root component
    yew::start_app::<CounterComponent>();
}
