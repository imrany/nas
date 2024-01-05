use leptos::*;

#[component]
pub fn Sidenav()->impl IntoView{
    view!{
        <div class="overflow-hidden fixed bottom-0 left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
            <div class="flex flex-col my-3">
                <div class="h-[33px] flex items-center text-[#999999] uppercase pl-[12px] pr-[8px]">
                    <span class="material-symbols-outlined md-16 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">draft</span>
                    <span class="material-symbols-outlined md-16 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">search</span>
                    <span class="material-symbols-outlined md-16 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">refresh</span>
                </div>
                <div class="flex items-center text-[#e5e5e5] text-[11px] uppercase px-[8px] h-[35px]">
                    <p class="pl-[12px]">EXPLORER</p>
                    <span class="material-symbols-outlined md-16 text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">more_horiz</span>
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
        </div>
    }
} 