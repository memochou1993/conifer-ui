use dotenv_codegen::dotenv;
use gloo_net::http::Request;
use serde::Deserialize;
use std::time::SystemTime;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const API_URL: &str = dotenv!("API_URL");

#[derive(Deserialize)]
struct Response {
    data: Vec<Record>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Record {
    id: String,
    url: String,
    expired_at: SystemTime,
    updated_at: SystemTime,
    created_at: SystemTime,
}

#[derive(PartialEq, Properties)]
struct RecordListProps {
    records: Vec<Record>,
    on_click: Callback<Record>,
}

#[function_component(RecordList)]
fn record_list(RecordListProps { records, on_click }: &RecordListProps) -> Html {
    records
        .iter()
        .map(|record| {
            html! {
                <p
                    onclick={
                        let on_click = on_click.clone();
                        let record = record.clone();
                        Callback::from(move |_| on_click.emit(record.clone()))
                    }
                    style="cursor: pointer;"
                >
                    {format!("{}/{}", API_URL, record.id)}
                </p>
            }
        })
        .collect()
}

#[function_component(App)]
pub fn app() -> Html {
    let records = use_state(|| vec![]);

    {
        let records = records.clone();
        use_effect_with_deps(
            move |_| {
                let records = records.clone();
                spawn_local(async move {
                    let url = format!("{}/api/records", API_URL);
                    let res: Response = Request::get(&url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    records.set(res.data);
                });
                || ()
            },
            (),
        );
    }

    let on_record_click = {
        Callback::from(move |record: Record| {
            log::info!("{:#?}", record);
        })
    };

    html! {
        <div>
            <RecordList
                records={(*records).clone()}
                on_click={on_record_click}
            />
        </div>
    }
}
