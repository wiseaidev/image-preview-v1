use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let input_image_ref = use_node_ref();
    let input_image_handle = use_state(String::default);
    let input_image = (*input_image_handle).clone();

    let on_input_image = {
        let input_image_ref = input_image_ref.clone();

        Callback::from(move |_| {
            let input = input_image_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_image_handle.set(input.value());
            }
        })
    };

    html! {
           <div class="container">
                <div class="title">{"Image Preview App"}</div>
                <input
                  type="text"
                  class="input"
                  placeholder="Image URL"
                  ref={input_image_ref}
                  oninput={on_input_image}
                />
                <img
                  class="img-preview"
                  src={input_image}
                  alt="preview"
                />
           </div>
    }
}
