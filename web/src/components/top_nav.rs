use leptos::*;
use web_sys::{
    window,
    Event,
};
use wasm_bindgen::prelude::*;

#[path="../lib/functions.rs"]
mod functions;
use functions::open_dialog;

#[component]
pub fn Topnav()-> impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();
    
    // Close the dropdown if the user clicks outside of it
    let document_ref=document.clone();
    let click_handler: Closure<dyn FnMut(_)> = Closure::new(move|e: Event| {
        // check the windows if the onclick target is a button equal to the buttom with the btn_more class, 
        // then it doesnt close the dropdown
        let btns_more=document_ref.get_elements_by_class_name("btn_more");
        for i in 0..btns_more.clone().length(){
            let btn_more=&btns_more.get_with_index(i);
            let e_match_btn_more=e.target().unwrap().to_string()==btn_more.clone().unwrap().to_string();
            // web_sys::console::log_2(&e.target().unwrap().into(),&btn_more.as_deref().into());
            // web_sys::console::log_1(&elem_match.into());
            if !e_match_btn_more {
                let dropdowns=document_ref.get_elements_by_class_name("dropdown-content");
                for i in 0..dropdowns.clone().length(){
                    let open_dropdown=dropdowns.get_with_index(i);
                    if open_dropdown.clone().unwrap().class_list().contains("block"){
                        open_dropdown.clone().unwrap().class_list().remove_1("block").unwrap();
                        let context_more_share=document_ref.get_elements_by_class_name("context-more-share");
                        for i in 0..context_more_share.clone().length(){
                            let open_context_more_share=context_more_share.get_with_index(i);
                            open_context_more_share.clone().unwrap().class_list().remove_1("block").unwrap();
                        }
                    }
                }
            }
        }

    });
    window.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();
    
    let open_offline_dialog=move|_|{
        open_dialog("offline_dialog");
    };

    let document_ref_1=document.clone();
    let show_dropdown_menu=move|_|{
        let dropdown_list=document_ref_1.get_element_by_id("dropdown_list").unwrap();
        dropdown_list.class_list().toggle("block").unwrap();
    };
    let context_version=move|_| {
        let context_list=web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("context_more_version").as_str()).unwrap();
        context_list.class_list().toggle("block").unwrap();
    };
    view!{
        <nav class="fixed bg-[#151515] px-[12px] top-0 left-0 right-0 z-10">
            <div class="font-semibold text-[13px] flex justify-between min-h-[48px] items-center text-white">
                <div style="position:relative; display:inline-block;">
                    <button on:click=show_dropdown_menu class="flex justify-center items-center py-[4px] px-[12px] cursor-pointer h-[24px]">
                        <p>Anvel</p>
                        <span class="material-symbols-outlined md-18 p-[3px]">expand_more</span>
                    </button>
                    <div id="dropdown_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal z-10 py-[4px] dropdown-content none absolute bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                        <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                            <span class="material-symbols-outlined md-16 pr-[6px]">content_copy</span>
                            <p>Documentation</p>
                        </div>
                        <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                            <span class="material-symbols-outlined md-16 pr-[6px]">edit</span>
                            <p>Customize</p>
                        </div>
                        <div>
                            <button on:click=context_version class="btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                <span class="material-symbols-outlined md-16 pr-[6px]">dns</span>
                                <p>Version 0.1.0</p>
                                <span class="material-symbols-outlined md-16 ml-auto">chevron_right</span>
                            </button>
                            <div id="context_more_version" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[181px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                    <span class="material-symbols-outlined md-16 pr-[6px]">upgrade</span>
                                    <p>Update</p>
                                </div>
                            </div>
                        </div>
                        <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                            <span class="material-symbols-outlined md-16 pr-[6px]">login</span>
                            <p>"Info"</p>
                        </div>
                    </div>
                </div>
                <div class="text-[#C2C2C2] flex font-medium min-w-[10%] items-center">
                    <p class="rounded-md bg-[#252525] py-[2px] px-2 mr-2">Directory</p>
                    <p class="root_path_indicator"></p>
                </div>
                <div class="text-[#C2C2C2] flex min-w-[10vw] justify-around">
                    <button 
                        on:click=open_offline_dialog
                        class="hover:text-white active:text-white focus:text-white"
                    >
                        Connnect
                    </button>
                    <a href="/" class="hover:text-white ml-[10px] active:text-white focus:text-white">Sign in</a>
                </div>
            </div>
        </nav>
    }
}
