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
                <RecordItem
                    record={record.clone()}
                    on_click={on_click}
                />
            }
        })
        .collect()
}

#[derive(PartialEq, Properties)]
pub struct RecordItemProps {
    pub record: Record,
    pub on_click: Callback<Record>,
}

#[function_component(RecordItem)]
pub fn record_item(RecordItemProps { record, on_click }: &RecordItemProps) -> Html {
    html! {
        <p
            onclick={
                let on_click = on_click.clone();
                let record = record.clone();
                Callback::from(move |_| (on_click).emit(record.clone()))
            }
            style="cursor: pointer;"
        >
            {format!("{}/{}", API_URL, record.id)}
        </p>
    }
}
