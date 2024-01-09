use leptos::*;
use web_sys::{
    window,
};
// use js_sys::{
//     Array
// };

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
    view!{
        <div id="connection_dialog" 
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
pub fn op_dialog()->impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let close_dialog=move|_|{
        let dialog_bg=document.get_element_by_id("op_dialog").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().remove_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
    };
    view!{
        <div id="op_dialog" 
            on:dblclick=close_dialog.clone()
            class="fixed top-0 bottom-0 left-0 right-0 z-4 opacity-[0.9] bg-[#151515]/70 none"
        >
            <div class="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" class="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div class="flex ml-auto mb-[8px] justify-end h-[22px] pb-[4px] text-white">
                        <span on:click=close_dialog.clone() class="material-symbols-outlined md-16 cursor-pointer">close</span>
                    </div>
                    <div class="w-[452px] h-[162px]"> 
                        <h2 class="text-white font-medium text-base">Lost Connection</h2>
                        <div class="text-[13px] mt-[4px] mb-[13px] pb-[24px] text-[#999999]">
                            <p>
                                "We are unable to reconnect you. Please check that you are online before trying again. If the problem persists, it means the development environment has reached an error state and you can try restarting it. Don't worry, your work will still be here when you get back."
                            </p>
                        </div>
                        <div class="flex justify-end items-center">
                            <button on:click=move|_|window.location().reload().unwrap() class="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                Restart
                            </button>
                            <button on:click=move|_|web_sys::window().unwrap().location().reload().unwrap() class="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                Reconnect
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}