use crate::{service::Record, API_URL};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RecordListProps {
    pub records: Vec<Record>,
    pub on_click: Callback<Record>,
}

#[function_component(RecordList)]
pub fn record_list(RecordListProps { records, on_click }: &RecordListProps) -> Html {
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
