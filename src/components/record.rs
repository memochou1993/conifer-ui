use crate::{service::Record, API_URL};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct RecordFormProps {
    pub on_submit: Callback<Record>,
}

#[function_component(RecordForm)]
pub fn record_form(RecordFormProps { on_submit }: &RecordFormProps) -> Html {
    html! {
        <figure class="basis-8/12 p-4 bg-slate-200 text-slate-800 rounded-md">
            <div class="flex">
                <div class="basis-9/12 mr-2">
                    <input id="input" type="text" class="w-full p-4 bg-white text-slate-800 shadow-md rounded-md outline-none" />
                </div>
                <div class="basis-3/12 ml-2">
                    <button
                        onclick={
                            let on_submit = on_submit.clone();
                            let record = Record::new(String::from("http://localhost")); // FIXME
                            Callback::from(move |_| (on_submit).emit(record.clone()))
                        }
                        class="w-full p-4 bg-blue-600 active:bg-blue-800 text-white shadow-md rounded-md">
                        {"Shorten"}
                    </button>
                </div>
            </div>
        </figure>
    }
}

#[derive(PartialEq, Properties)]
pub struct RecordListProps {
    pub records: Vec<Record>,
    pub on_click: Callback<Record>,
}

#[function_component(RecordList)]
pub fn record_list(RecordListProps { records, on_click }: &RecordListProps) -> Html {
    html! {
        <figure class="basis-8/12 bg-slate-200 text-slate-800 rounded-md">
            <div class="divide-y divide-slate-400 divide-solid">
                {
                    for records.iter().map(|record| {
                        html! {
                            <RecordItem
                                record={record.clone()}
                                on_click={on_click}
                            />
                        }
                    })
                }
            </div>
        </figure>
    }
}

#[derive(PartialEq, Properties)]
pub struct RecordItemProps {
    pub record: Record,
    pub on_click: Callback<Record>,
}

#[function_component(RecordItem)]
pub fn record_item(RecordItemProps { record, on_click }: &RecordItemProps) -> Html {
    html! {
        <div class="flex p-8 text-left">
            <div class="basis-6/12 mr-4 w-0">
                <p class="text-ellipsis overflow-hidden whitespace-nowrap">
                    {format!("{}", record.url)}
                </p>
            </div>
            <div class="basis-6/12 ml-4 w-0">
                <p class="text-ellipsis overflow-hidden whitespace-nowrap text-blue-600 cursor-pointer"
                    onclick={
                        let on_click = on_click.clone();
                        let record = record.clone();
                        Callback::from(move |_| (on_click).emit(record.clone()))
                    }
                >
                    {format!("{}/{}", API_URL, record.id)}
                </p>
            </div>
        </div>
    }
}
