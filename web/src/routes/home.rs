use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh]">
            <nav class="bg-[#151515] font-semibold text-[13px] px-[12px] flex justify-between items-center text-white min-h-[48px]">
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
            </nav>
        </div>
    }
}