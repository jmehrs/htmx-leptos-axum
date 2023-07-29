use axum::{
    response::Html,
    Router,
    Form, extract::State, routing::post
};
use leptos::{ssr::render_to_string as render, *};
use std::sync::{Arc, Mutex};
use serde::Deserialize;

type DeviceState = Arc<Mutex<Vec<DeviceAddForm>>>;

pub fn device_router() -> Router {
    let device_state: DeviceState = Arc::new(Mutex::new(vec![]));
    Router::new()
        .route("/add", post(post_device_add))
        .with_state(device_state)
}

#[component]
pub fn DeviceAdd(cx: Scope) -> impl IntoView {
    view!{cx,
        <h2>"Add a device"</h2>
        <form 
            hx-post="/device/add"
            class="border rounded-lg"            
        >
            <label for="device-address">"Device address: "</label>
            <input id="device-address" name="device_address" type="text"
                class="border rounded-md"
            />
            <br/>
            <label for="device-username">"Username: "</label>
            <input id="device-username" name="device_username" type="text"
                class="border rounded-md"
            />
            <br/>
            <label for="device-password">"password: "</label>
            <input id="device-password" name="device_password" type="password"
                class="border rounded-md"
            />
            <br/>
            <button type="submit">"Submit"</button>
        </form>
    }
}


#[derive(Deserialize, Debug)]
struct DeviceAddForm {
    device_address: String,
    device_username: String,
    device_password: String,
}

async fn post_device_add(State(state): State<DeviceState>, Form(form): Form<DeviceAddForm>) -> Html<String> {
    dbg!{&form};
    {
        let mut state = state.lock().expect("The prior lock user panicked while holding this lock");
        state.push(form);
    }
    Html(render(|cx| { 
        view! { cx,
            <p>"Device Added"</p>
        }
    }))
}
