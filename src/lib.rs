use crate::components::record::{RecordForm, RecordList};
use dotenv_codegen::dotenv;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod components;
mod service;

const API_URL: &str = dotenv!("API_URL");

#[function_component(App)]
pub fn app() -> Html {
    let records = use_state(|| vec![]);

    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    {
        let records = records.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let data = local_storage.get_item("records").unwrap();
                    if let Some(data) = data {
                        let value = serde_json::from_str(&data).unwrap();
                        records.set(value);
                    }
                });
                || ()
            },
            (),
        );
    }

    let submit = {
        Callback::from(move |record| {
            spawn_local(async move {
                let res = service::store_record(&record).await;
                log::info!("{:#?}", res.data); // TODO
            });
        })
    };

    let redirect = {
        Callback::from(move |record| {
            log::info!("{:#?}", record); // TODO
        })
    };

    html! {
        <div class="h-screen flex items-center bg-slate-800 text-white font-light">
            <div class="container mx-auto">
                <div class="flex justify-center my-8">
                    <p class="text-4xl">
                        {"URL Shortener"}
                    </p>
                </div>
                <div class="flex justify-center my-8">
                    <RecordForm
                        on_submit={submit}
                    />
                </div>
                <div class="flex justify-center my-8">
                    <RecordList
                        records={(*records).clone()}
                        on_click={redirect}
                    />
                </div>
            </div>
        </div>
    }
}
