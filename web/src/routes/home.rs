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
                <div class="flex flex-col m-3">
                    <div class="text-[#e5e5e5] text-[11px] uppercase py-3">
                        <p>EXPLORER</p>
                    </div>
                    <div class="flex flex-col">
                        <p class="text-[#e5e5e5] text-[11px] uppercase py-1"> ">" zippy info</p>
                    </div>
                    <div class="flex flex-col">
                        <p class="text-[#e5e5e5] text-[11px] uppercase py-1"
                        on:click=move |_| println!("hry")
                        > ">" Telegram</p>
                        <div class="pl-[8px] flex flex-col">
                            <a href="/docs"> ">" Documentation</a>
                        </div>
                        <summary class="ml-3">My name</summary>
                        <details>
                            <p>My name is imran</p>
                            <p>My name is imran</p>
                        </details>
                    </div>
                </div>
            </div>
        </div>
    }
}