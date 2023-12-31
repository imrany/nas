use leptos::*;
use web_sys::{
    window,
};

#[component]
pub fn Nav()-> impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();

    let open_share_dialog=move|_|{
        let dialog_bg=document.get_element_by_id("dialog_bg").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().add_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
        // console::log_1(&"dblclick on dialog bg".into());
    };
    view!{
        <nav class="fixed inset-0">
            <div class="bg-[#151515] font-semibold text-[13px] px-[12px] flex justify-between items-center text-white min-h-[48px]">
                <div>
                    <a href="/">Zippy</a>
                </div>
                <div class="text-[#C2C2C2] flex min-w-[10%] justify-around">
                    <p class="font-medium">Downloads/Telegram</p>
                </div>
                <div class="text-[#C2C2C2] flex min-w-[10%] justify-around">
                    <button on:click=open_share_dialog.clone() class="hover:text-white active:text-white focus:text-white">Share</button>
                    <a href="/" class="hover:text-white active:text-white focus:text-white">Sign in</a>
                </div>
            </div>
        </nav>
    }
}