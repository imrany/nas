use leptos::*;

#[component]
pub fn Bottomnav()->impl IntoView{
    view!{
        <div class="fixed bottom-0 h-[22px] py-[2px] bg-[#151515] px-[7px] left-0 right-0">
            <div class="flex text-[12px] text-[#808080]">
                <p>anvel</p>
                <div class="flex ml-auto text-[12px]">
                    <a href="/" class="mx-[3px] px-[5px]">Folder: 2</a>
                    <a href="/" class="mx-[3px] px-[5px]">File: 49</a>
                    <a href="/" class="mx-[3px] px-[5px]">UTF-8</a>
                </div>
            </div>
        </div>
    }
}