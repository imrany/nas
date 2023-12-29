use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh] bg-[#1d1d1d]">
            <nav class="fixed inset-0">
                <div class="bg-[#151515] font-semibold text-[13px] px-[12px] flex justify-between items-center text-white min-h-[48px]">
                    <div>
                        <a href="/">Zippy</a>
                    </div>
                    <div class="text-[#C2C2C2] flex min-w-[10%] justify-around">
                        <p class="font-medium">Downloads/Telegram</p>
                    </div>
                    <div class="text-[#C2C2C2] flex min-w-[10%] justify-around">
                        <a href="/" class="hover:text-white active:text-white focus:text-white">Share</a>
                        <a href="/" class="hover:text-white active:text-white focus:text-white">Sign in</a>
                        <a></a>
                    </div>
                </div>
            </nav>

            <div class="overflow-hidden fixed bottom-0 left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
                <div class="flex flex-col my-3">
                    <div class="text-[#e5e5e5] text-[11px] uppercase p-3">
                        <p>EXPLORER</p>
                    </div>
                    <details class="flex flex-col">
                        <summary class="text-[#e5e5e5] mx-[1px] px-3 text-[11px] uppercase py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300">zippy info</summary>
                        <details>
                            <summary class="hover:bg-[#999999] hover:bg-[#37373D] pl-6 pr-3 mx-[1px] cursor-pointer text-[#999999] active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name</summary>
                            <div class="mt-[1px] flex flex-col text-[13px] text-[#999999]">
                                <button class="hover:bg-[#999999] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name is imran</button>
                                <button class="hover:bg-[#999999] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">My name is imran</button>
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
            </div>
        </div>
    }
}