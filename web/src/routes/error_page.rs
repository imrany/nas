use leptos::*;
use leptos_meta::*;
use web_sys::{
    window,
    console,
    // Window,
    Event,
};
use js_sys::{
    Array
};

fn back(ev:Event){
    window().unwrap().alert_with_message("hey").unwrap();
    console::log_2(&ev,&"this event".into());
}

#[component]
pub fn Error_page()->impl IntoView{
    let window=window().unwrap();
    let reload=move|_| {
        let array = Array::new();
        array.push(&"Hello Console Log".into());
        console::log(&array);
        console::log_2(&"Hello World Gloo :%s".into(),
									&"WebAssemblyMan".into());
    };

    // let back=|_| {
    // };
    view! {
        <Title text="Something went wrong"/>
        <div class="flex flex-col w-[100vw] p-[91px] z-9 text-white h-[100vh] justify-center bg-[#0E0E0E]">
            <div class="mb-[8px] pb-[46px]">
                <a href="/" class="font-semibold text-[25px]">Zippy {window.origin()}</a>
            </div>
            <div class="max-w-[528px] flex flex-col">
                <div class="mb-[24px]">
                    <h1 class="font-semibold text-[32px]">Something went wrong</h1>
                </div>
                <p class="text-[13px] text-[#999999] mb-[24px]">
                    "The sandbox or repository might not be visible because you're not signed in, you can sign in by clicking the 'Sign in' button."
                </p>
                <p class="text-[13px] text-[#999999] mb-[24px]">
                    "If the issue persists,"
                    <strong> please contact our support team at </strong>
                    <a href="mailto:support@codesandbox.io" class="font-normal underline">support@codesandbox.io</a>
                    " for further assistance, making sure to copy the error details below."
                </p>
                <details>
                    <summary class="text-[#999999] cursor-pointer hover:text-white active:text-white focus:text-white text-[13px]">Problem details and configurations</summary>
                    <div class="text-[#999999] bg-[#151515] rounded-sm mt-[24px] p-[24px] text-[13px]">
                        <p>"TypeError:  Failed to fetch"</p>
                        <div class="mt-[16px] grid grid-rows-2 gap-3">
                            <code class="grid grid-cols-2 gap-6">
                                <span>"Version"</span> 
                                <span>"1.0.2"</span>
                            </code>
                            <code class="grid grid-cols-2 gap-6">
                                <span>"Environment"</span>      
                                <span>"Local"</span>
                            </code>
                        </div>
                    </div>
                </details>
                <div class="flex items-center mt-[24px]">
                    <button on:click=reload class="mr-[12px] py-[4px] px-[16px] hover:bg-[#ffcc00] border-none font-semibold h-[28px] w-[119px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                        Reload page
                    </button>
                    <button on:click=|ev:Event| back(ev) class="mr-[12px] py-[4px] px-[16px]  border-[1px] border-[#343434] font-semibold hover:bg-[#292d39]  hover:text-white h-[28px] w-[119px] text-[13px] text-[#C2C2C2] rounded-sm bg-[#252525]">
                        Back Home
                    </button>
                </div>
            </div>
        </div>
    }
}