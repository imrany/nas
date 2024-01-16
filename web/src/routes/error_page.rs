use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    window,
};

struct ErrorBody{
    error_type:JsValue,
    message:String,
    solution:String,
}

#[component]
pub fn Error_page()->impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let query = use_query_map();
    let error_params =move || query.with_untracked(|err| err.get("error").cloned().unwrap_or_default());

    let a = Closure::<dyn Fn()>::new(move || {
        let error_type_dom=document.get_element_by_id("error_type").expect("Failed to get error_type element"); 
        let error_message_dom=document.get_element_by_id("error_message").expect("Failed to error_message element");
        let solution_dom=document.get_element_by_id("solution").expect("Failed to get solution element");
    
        match error_params().clone().as_str() {
            "not_supported" => {
                let err=ErrorBody{
                    error_type:JsValue::from_str("Not supported"),
                    message:"This item or feature might not be supported on the current version, you can update to the lastest version.".to_string(),
                    solution:"
                    If the issue persists,
                    <strong> please contact our support team at </strong>
                    <a href='mailto:imranmat254@gmail.com' class='font-normal underline'>imranmat254@gmail.com</a>
                    for further assistance, making sure to copy the error details below.
                    ".to_string()
                };
                error_type_dom.set_text_content(Some(format!("TypeError: \t {}",err.error_type.as_string().unwrap()).as_str()));
                error_message_dom.set_inner_html(err.message.as_str());
                solution_dom.set_inner_html(err.solution.as_str());
            },
            "Failed to fetch data" => {
                let err=ErrorBody{
                    error_type:JsValue::from_str("Failed to fetch data"),
                    message:"You are not connected to Anvel,please restart Anvel on your terminal or CMD by running <code class='text-[#C2C2C2] rounded-sm bg-[#252525] py-[2px] px-2'>anvel</code> .".to_string(),
                    solution:"
                    If the issue persists,
                    <strong> please contact our support team at </strong>
                    <a href='mailto:imranmat254@gmail.com' class='font-normal underline'>imranmat254@gmail.com</a>
                    for further assistance, making sure to copy the error details below.
                    ".to_string()
                };
                error_type_dom.set_text_content(Some(format!("TypeError: \t {}",err.error_type.as_string().unwrap()).as_str()));
                error_message_dom.set_inner_html(err.message.as_str());
                solution_dom.set_inner_html(err.solution.as_str());
            },
            _ => {
                let err=ErrorBody{
                    error_type:JsValue::from_str("Not found"),
                    message:"This item might not be visible because you're not connected.".to_string(),
                    solution:"
                    If the issue persists,
                    <strong> please contact our support team at </strong>
                    <a href='mailto:imranmat254@gmail.com' class='font-normal underline'>imranmat254@gmail.com</a>
                    for further assistance, making sure to copy the error details below.
                    ".to_string()
                };
                error_type_dom.set_text_content(Some(format!("TypeError: \t {}",err.error_type.as_string().unwrap()).as_str()));
                error_message_dom.set_inner_html(err.message.as_str());
                solution_dom.set_inner_html(err.solution.as_str());
            }
        }
    });
    window.set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 500).unwrap();
    a.forget();

    view! {
        <Title text="Something went wrong"/>
        <div class="flex flex-col w-[100vw] p-[91px] z-9 text-white h-[100vh] justify-center bg-[#0E0E0E]">
            <div class="mb-[8px] pb-[18px]">
                <a href="/" class="flex items-center font-semibold text-[25px]">
                    <img src="/assets/icons/favicon.svg" style="width:25px; height:25px;"/>
                    <span class="ml-1">Anvel</span>
                </a>
            </div>
            <div class="max-w-[528px] flex flex-col">
                <div class="mb-[24px]">
                    <h1 class="font-semibold text-[32px]">Something went wrong</h1>
                </div>
                <p id="error_message" class="text-[13px] text-[#999999] mb-[24px]">
                    
                </p>
                <p id="solution" class="text-[13px] text-[#999999] mb-[24px]">
                </p>
                <details>
                    <summary class="text-[#999999] cursor-pointer hover:text-white active:text-white focus:text-white text-[13px]">Problem details and configurations</summary>
                    <div class="text-[#999999] bg-[#151515] rounded-sm mt-[24px] p-[24px] text-[13px]">
                        <p id="error_type"></p>
                        <div class="mt-[16px] grid grid-rows-2 gap-3">
                            <code class="grid grid-cols-2 gap-6">
                                <span>"Version"</span> 
                                <span>"0.1.0"</span>
                            </code>
                            <code class="grid grid-cols-2 gap-6">
                                <span>"Environment"</span>      
                                <span>"Local"</span>
                            </code>
                        </div>
                    </div>
                </details>
                <div class="flex items-center mt-[24px]">
                    <button on:click=move|_|window.location().reload().unwrap() class="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none font-semibold h-[28px] w-[119px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                        Refresh page
                    </button>
                    <a href="/" class="mr-[12px] py-[4px] px-[16px] border-[1px] border-[#343434] font-semibold hover:bg-[#292d39]  hover:text-white h-[28px] w-[119px] text-[13px] text-[#C2C2C2] rounded-sm bg-[#252525]">
                        Back Home
                    </a>
                </div>
            </div>
        </div>
    }
}