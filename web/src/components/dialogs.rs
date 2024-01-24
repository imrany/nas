use leptos::*;
use web_sys::{
    window,
    Node,
    Event,
    HtmlFormElement,
    HtmlInputElement,
};
use wasm_bindgen::prelude::*;

#[path="../lib/functions.rs"]
mod functions;
use functions::{
    fetch_data,
};


#[component]
pub fn Offline_dialog()->impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let close_dialog=move|_|{
        let dialog_bg=document.get_element_by_id("offline_dialog").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().remove_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
    };
    view!{
        <div id="offline_dialog" 
            class="fixed top-0 bottom-0 left-0 right-0 z-20 bg-[#151515]/70 none"
        >
            <div class="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" class="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div class="w-[452px] h-[162px]"> 
                        <h2 class="text-white font-medium text-base">Lost Connection</h2>
                        <div class="text-[13px] mt-[4px] mb-[13px] pb-[24px] text-[#999999]">
                            <p>
                                "We are unable to reconnect you. Please check that you are online before trying again. If the problem persists, it means the development environment has reached an error state and you can try restarting it. Don't worry, your work will still be here when you get back."
                            </p>
                        </div>
                        <div class="flex justify-end items-center">
                            <button on:click=move|_|window.clone().location().reload().unwrap() class="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                Restart
                            </button>
                            <button on:click=close_dialog.clone() class="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                Reconnect
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Connection_dialog()->impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let close_dialog=move|_|{
        let dialog_bg=document.get_element_by_id("connection_dialog").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().remove_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
    };
    let submit_handle=move|_|{
        let form: HtmlFormElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("save_recipient_ip")
            .unwrap()
            .dyn_into()
            .unwrap();
        let form_ref=form.clone();
        let closure = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            let ip_input:HtmlInputElement=web_sys::window().unwrap().document().unwrap().get_element_by_id("ip_input").unwrap().dyn_into().unwrap();
            let ip_value = ip_input.value();
            let set_ip=web_sys::window().unwrap().session_storage().unwrap().unwrap().set_item("recipient_ip",&ip_value);
            match set_ip {
                Ok(_) => {
                    form_ref.reset();
                    let dialog_bg=web_sys::window().unwrap().document().unwrap().get_element_by_id("connection_dialog").unwrap();
                    dialog_bg.class_list().add_1("ease-in-out").unwrap();
                    dialog_bg.class_list().remove_1("block").unwrap();
                    dialog_bg.class_list().add_1("duration-1000").unwrap();
                    dialog_bg.class_list().add_1("delay-2000").unwrap();
                },
                Err(e) => web_sys::console::error_1(&e.into())
            }
        }) as Box<dyn FnMut(_)>);
    
        form.set_onsubmit(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    };
    view!{
        <div id="connection_dialog" 
            on:dblclick=close_dialog.clone()
            class="fixed top-0 bottom-0 left-0 right-0 z-4 bg-[#151515]/70 none"
        >
            <div class="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" class="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div class="flex ml-auto mb-[8px] justify-end h-[22px] pb-[4px] text-white">
                        <span on:click=close_dialog.clone() class="material-symbols-outlined md-16 cursor-pointer">close</span>
                    </div>
                    <div class="w-[452px] h-[162px]"> 
                        <h2 class="text-white font-medium text-base">"Enter recipient's Ip address"</h2>
                        <div class="text-[13px] mt-[4px] text-[#999999]">
                            <p>
                                "Paste or enter your recipient's local Ip and try again."
                            </p>
                            <form id="save_recipient_ip" class="my-3">
                                <input type="text" id="ip_input" name="ip" class="w-full py-[4px] px-[6px] rounded-sm bg-transparent border-violet-300 border-[1px] focus:ring-1 focus:ring-violet-300" placeholder="192.168.43.174"/>
                                <div class="flex justify-end items-center mt-[37px]">
                                    <button type="button" on:click=close_dialog.clone() class="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                        Cancel
                                    </button>
                                    <button on:click=submit_handle class="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                        Connect
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Network_dialog()->impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let close_dialog=move|_|{
        let dialog_bg=document.get_element_by_id("network_dialog").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().remove_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
    };

    let closure: Closure<dyn FnMut()> = Closure::new(move|| {
        create_resource(|| (), |_| async move { 
            match fetch_data("http://localhost:8000/api/get_ip_address").await {
                Ok(data) => {
                    let err_message = js_sys::Reflect::get(&data, &JsValue::from_str("message")).unwrap();
                   
                    match err_message.clone().is_string() {
                        true => {
                            let dom_elem=web_sys::window().unwrap().document().unwrap().get_element_by_id("network_info").unwrap();
                            let message_str = err_message.as_string().unwrap_or_default();
                            let item=format!("
                                <p>
                                    Error - {message_str}
                                </p>
                            "); 
                            
                            let item_element =web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
        
                            item_element.set_inner_html(&item);
                            dom_elem.append_child(&Node::from(item_element)).unwrap();
                        },
                        false => {
                            let dom_elem=web_sys::window().unwrap().document().unwrap().get_element_by_id("network_info").unwrap();
                            let internal_ip = js_sys::Reflect::get(&data, &JsValue::from_str("internal"))
                            .map_err(|_| JsValue::from_str("Failed to access internal property")).unwrap()
                            .as_string().unwrap_or_default();
                            let external_ip = js_sys::Reflect::get(&data, &JsValue::from_str("external"))
                            .map_err(|_| JsValue::from_str("Failed to access external property")).unwrap()
                            .as_string().unwrap_or_default();
                            
                            let item=format!("
                                <p>
                                    Your local Ip - {internal_ip}
                                </p>
                                <p>
                                    Your external Ip - {external_ip}
                                </p>
                                <p>
                                    Your anvel server url - <a href='http://{internal_ip}:8000'>http://{internal_ip}:8000</a>
                                </p>
                            "); 

                            let item_element =web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
        
                            item_element.set_inner_html(&item);
                            dom_elem.append_child(&Node::from(item_element)).unwrap();
                        }
                    };
                }
                Err(e) => { 
                    web_sys::console::log_1(&e.into());
                }
            }
        });
    });
    
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref()
    ,300).unwrap();
    closure.forget();

    view!{
        <div id="network_dialog" 
            on:dblclick=close_dialog.clone()
            class="fixed top-0 bottom-0 left-0 right-0 z-4 bg-[#151515]/70 none"
        >
            <div class="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" class="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div class="flex ml-auto mb-[8px] justify-end h-[22px] pb-[4px] text-white">
                        <span on:click=close_dialog.clone() class="material-symbols-outlined md-16 cursor-pointer">close</span>
                    </div>
                    <div class="w-[452px] h-[162px]"> 
                        <h2 class="text-white font-medium text-base">Network information</h2>
                        <div id="network_info" class="text-[13px] mt-[4px] mb-[13px] pb-[24px] text-[#999999]">
                            
                        </div>
                        <div class="flex justify-end items-center">
                            <button on:click=move|_|web_sys::window().unwrap().location().reload().unwrap() class="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                Try again
                            </button>
                            <button on:click=close_dialog.clone() class="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                Close
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Feedback_dialog()->impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let close_dialog=move|_|{
        let dialog_bg=document.get_element_by_id("feedback_dialog").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().remove_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
    };
    view!{
        <div id="feedback_dialog" 
            class="fixed top-0 bottom-0 left-0 right-0 z-20 bg-[#151515]/70 none"
        >
            <div class="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" class="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div class="flex ml-auto mb-[8px] justify-end h-[22px] pb-[4px] text-white">
                        <span on:click=close_dialog.clone() class="material-symbols-outlined md-16 cursor-pointer">close</span>
                    </div>    
                    <div class="w-[452px] h-[162px]"> 
                        <div id="feedback_container"></div>
                        <div class="flex justify-end items-center">
                            <button on:click=close_dialog.clone() class="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                Close
                            </button>
                            <button on:click=move|_|window.clone().location().reload().unwrap() class="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                Reload
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}