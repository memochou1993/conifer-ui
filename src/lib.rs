use crate::components::record::RecordList;
use dotenv_codegen::dotenv;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod components;
mod service;

const API_URL: &str = dotenv!("API_URL");

#[function_component(App)]
pub fn app() -> Html {
    let records = use_state(|| vec![]);

    {
        let records = records.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let res = service::fetch_records().await;
                    records.set(res.data);
                });
                || ()
            },
            (),
        );
    }

    let redirect = {
        Callback::from(move |record| {
            log::info!("{:#?}", record);
        })
    };

    html! {
        <div>
            <RecordList
                records={(*records).clone()}
                on_click={redirect}
            />
        </div>
    }
}
