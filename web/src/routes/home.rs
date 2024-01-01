use leptos::*;
use leptos_meta::*;
// use leptos_icons::{
//     OcIcon::*,
//     FaIcon::*,
//     // ImIcon::*,
//     BsIcon::*,
//     BiIcon::*,
//     AiIcon::*,
//     Icon,
// };

#[path="../components/nav.rs"]
mod nav;
use nav::Nav;

// const ORANGE_ICON:&str =r#"
//     color: orange
// "#;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh] bg-[#1d1d1d]">
            <Nav/>
            <div class="overflow-hidden fixed bottom-0 left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
                <div class="flex flex-col my-3">
                    <div class="text-[#e5e5e5] text-[11px] uppercase p-3">
                        <p>EXPLORER</p>
                    </div>
                    <details class="flex flex-col">
                        <summary class="text-[#e5e5e5] mx-[1px] px-3 text-[11px] uppercase py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300">Zippy info</summary>
                        <details>
                            <summary class="hover:bg-[#3c3c3c]/35 pl-6 pr-3 mx-[1px] cursor-pointer text-[#999999] active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name</summary>
                            <div class="mt-[1px] flex flex-col text-[13px] text-[#999999]">
                                <button class="hover:bg-[#3c3c3c]/35 mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name is imran</button>
                                <button class="hover:bg-[#3c3c3c]/35 mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name is imran</button>
                            </div>
                        </details>
                    </details>
                    <details class="flex flex-col">
                        <summary class="text-[#e5e5e5] mx-[1px] px-3 text-[11px] uppercase py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300">Telegram</summary>
                        <details>
                            <summary class="hover:bg-[#3c3c3c]/35 pl-6 pr-3 mx-[1px] cursor-pointer text-[#999999] active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name</summary>
                            <div class="mt-[1px] flex flex-col text-[13px] text-[#999999]">
                                <button class="hover:bg-[#3c3c3c]/35 mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name is imran</button>
                                <button class="hover:bg-[#3c3c3c]/35 mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name is imran</button>
                            </div>
                        </details>
                    </details>
                </div>

                // <Icon icon=Icon::from(AiCarryOutTwotone) style="color: green" />
                // <Icon icon=Icon::from(BiGraphql) width="2em" height="2em" style="color: green"/>
                // <Icon icon=Icon::from(BiGraphql) style=ORANGE_ICON/>
                // <Icon icon=Icon::from(FaBarsSolid) />
                // <Icon icon=Icon::from(BsFolder) />
                // <Icon icon=Icon::from(OcAlertSm) />
            </div>
        </div>
    }
}