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
        <div class="h-screen flex items-center bg-slate-800 text-white">
            <div class="container mx-auto">
                <div class="flex justify-center">
                    <figure class="basis-8/12 my-4 px-2 py-4 text-center bg-slate-200 text-slate-800 rounded-md">
                        <div class="flex">
                            <div class="basis-9/12 px-2">
                                <input id="input" type="text" class="w-full p-4 bg-white text-slate-800 shadow-md rounded-md outline-none" />
                            </div>
                            <div class="basis-3/12 px-2">
                                <button class="w-full p-4 bg-slate-800 text-white shadow-md rounded-md">
                                    {"Shorten"}
                                </button>
                            </div>
                        </div>
                    </figure>
                </div>
                <div class="flex justify-center">
                    <figure class="basis-8/12 my-4 px-2 py-4 text-center bg-slate-200 text-slate-800 rounded-md">
                        <RecordList
                            records={(*records).clone()}
                            on_click={redirect}
                        />
                    </figure>
                </div>
            </div>
        </div>
    }
}
