use leptos::*;
use leptos_meta::*;
use wasm_bindgen::prelude::*;
use js_sys::{
    Array,
};
use web_sys::{
 window,
 Request, 
 RequestInit, 
 RequestMode, 
 Response,
 Node,
 Event,
};
use wasm_bindgen_futures::JsFuture;

#[path="../components/top_nav.rs"]
mod top_nav;
use top_nav::Topnav;

#[path="../components/side_nav.rs"]
mod side_nav;
use side_nav::Sidenav;

#[path="../components/bottom_nav.rs"]
mod bottom_nav;
use bottom_nav::Bottomnav;

#[path="../lib/functions.rs"]
mod functions;
use functions::{
    open_dialog,
};

// struct DirectoryObject {
//     name:String,
//     path:std::path::PathBuf,
//     metadata:FileMeta
// }
// struct FileMeta{
//     is_file:bool,
//     file_extension:Option<String>,
// }

// const ORANGE_ICON:&str =r#"
//     color: orange
// "#;



async fn fetch_data() ->Result<JsValue, JsValue> {
    let window=window().expect("Failed to get Window");
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url=format!("http://localhost:8000/api/directory_content");
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request
        .headers()
        .set("content-type", "application/json").unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    if let Ok(resp) =  resp_value {
        let resp:Response=resp.into();
        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        // Send the JSON response back to JS.
        Ok(json)
    }else {
        Err(JsValue::from_str("Failed to fetch data"))
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let window=window().expect("Failed to get Window");
    let _document=window.document().expect("Failed to get Document");
    let navigator=window.navigator();
    // window.alert_with_message(format!("Not online, {}",navigator.on_line()).as_str()).unwrap();

    let data=create_resource(|| (), |_| async move { 
        match fetch_data().await {
            Ok(data) => {
                // web_sys::console::log_1(&data.clone().into());
                let dom_elem=web_sys::window().unwrap().document().unwrap().get_element_by_id("test").unwrap();
                let object_content = js_sys::Reflect::get(&data, &JsValue::from_str("contents"))
                .map_err(|_| JsValue::from_str("Failed to access contents property")).unwrap();

                let contents=Array::from(&object_content);
                for content in contents {
                    let name = js_sys::Reflect::get(&content, &JsValue::from_str("name"))
                    .map_err(|_| JsValue::from_str("Failed to access name property")).unwrap();
                    let path = js_sys::Reflect::get(&content, &JsValue::from_str("path"))
                    .map_err(|_| JsValue::from_str("Failed to access path property")).unwrap();
                    // metadata content
                    let metadata = js_sys::Reflect::get(&content, &JsValue::from_str("metadata"))
                    .map_err(|_| JsValue::from_str("Failed to access metadata property")).unwrap();
                    let is_file = js_sys::Reflect::get(&metadata, &JsValue::from_str("is_file"))
                    .map_err(|_| JsValue::from_str("Failed to access is_file property")).unwrap();
                    let file_extension=js_sys::Reflect::get(&metadata, &JsValue::from_str("file_extension"))
                    .map_err(|_| JsValue::from_str("Failed to access file_extension property")).unwrap();
    
                    // Convert the filename to a Rust String
                    let name_str = name.as_string().unwrap_or_default();
                    let path_str = path.as_string().unwrap_or_default();
            
                    let item=format!("
                        <button id='{name_str}' class='flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:text-white dropdown_btn'>
                            <img id='img_{name_str}' alt='file' class='w-[75px] h-[75px]'/>
                            <div>
                                <p class='text-center'>{name_str}</p>
                            </div>
                        </button>
                        <div id='context_list_{name_str}' style='box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);' class='font-normal ml-[80px] mt-[40px] z-5 py-[4px] dropdown-content absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]'>
                            <div class='px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                <span class='material-symbols-outlined md-16 pr-[6px]'>open_in_new</span>
                                <p>Open</p>
                            </div>
                            <div class='px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                <span class='material-symbols-outlined md-16 pr-[6px]'>open_with</span>
                                <p>Open with media player</p>
                            </div>
                            <div>
                                <button id='context_share_{name_str}' class='btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                    <span class='material-symbols-outlined md-16 pr-[6px]'>share</span>
                                    <p>Share</p>
                                    <span class='material-symbols-outlined md-16 ml-auto'>chevron_right</span>
                                </button>
                                <div id='context_more_share_{name_str}' style='box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);' class='font-normal ml-[191px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]'>
                                    <div class='px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                        <span class='material-symbols-outlined md-16 pr-[6px]'>bluetooth</span>
                                        <p>Bluetooth</p>
                                    </div>
                                    <div class='px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                        <span class='material-symbols-outlined md-16 pr-[6px]'>rss_feed</span>
                                        <p>Send to</p>
                                    </div>
                                </div>
                            </div>
                            <div class='px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                <span class='material-symbols-outlined md-16 pr-[6px]'>edit</span>
                                <p>Rename</p>
                            </div>
                            <div class='px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                <span class='material-symbols-outlined md-16 pr-[6px]'>delete</span>
                                <p>Delete</p>
                            </div>
                        </div>
                    "); 
                
                    let item_element =web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
                    item_element.set_class_name("flex");
                    item_element.set_inner_html(&item);
    
                    dom_elem.append_child(&Node::from(item_element)).unwrap();
                    let name_str_copy=name_str.clone();
                    let show_context_menu: Closure<dyn FnMut(_)> = Closure::new(move|e:Event| {
                        e.prevent_default();
                        let context_list=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("context_list_{}",name_str_copy.as_str())).unwrap();
                        context_list.class_list().toggle("block").unwrap();
                    });
    
                    let name_str_copy=name_str.clone();
                    let context_share: Closure<dyn FnMut(_)> = Closure::new(move|e:Event| {
                        e.prevent_default();
                        let context_list=web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("context_more_share_{}",&name_str_copy).as_str()).unwrap();
                        context_list.class_list().toggle("block").unwrap();
                    });

                    let path_str_copy=path_str.clone();
                    let open_file: Closure<dyn FnMut()> = Closure::new(move|| {
                        web_sys::window().unwrap().location().set_href(path_str_copy.as_str()).unwrap();
                    });

                    let image=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("img_{}",name_str)).unwrap();
                    if is_file.clone().as_bool().unwrap() {
                        image.set_attribute("src", "/assets/icons/file.png").unwrap();
                        image.set_attribute("alt", "File").unwrap();
                    } else {
                        image.set_attribute("src", "/assets/icons/folder.png").unwrap();
                        image.set_attribute("alt", "Folder").unwrap();    
                    }
                    
                    let btn=web_sys::window().unwrap().document().unwrap().get_element_by_id(&name_str.as_str()).unwrap();
                    btn.add_event_listener_with_callback("dblclick", &open_file.as_ref().unchecked_ref()).unwrap();
                    btn.add_event_listener_with_callback("contextmenu", &show_context_menu.as_ref().unchecked_ref()).unwrap();
                    show_context_menu.forget();
                    open_file.forget();
    
                    web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("context_share_{}",&name_str.as_str())).unwrap().add_event_listener_with_callback("click", &context_share.as_ref().unchecked_ref()).unwrap();
                    context_share.forget();
                }
            }
            Err(e) => { 
                web_sys::console::error_1(&e.into());
            }
        }
     });

    if !navigator.on_line() {
        let closure: Closure<dyn FnMut()> = Closure::new(move|| {
            open_dialog("connection_dialog");
        });
        
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref()
        ,300).unwrap();
        closure.forget();
    }

    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh] bg-[#1d1d1d]">
            <Topnav/>
            <Sidenav/>

            <div class="ml-[200px] mt-[48px] mb-[22px] text-[#999999]">
                // folder view
                <div id="folder_view">
                    // folder view nav
                    <div id="folder_view_nav">
                        <div class="flex w-full h-[35px] bg-[#151515]">
                            <div class="bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                                <span class="material-symbols-outlined md-16 w-[16px] mr-[5px]">folder_open</span>
                                <p class="text-[#E5E5E5] text-[13px]">Downloads</p>
                                <span class="material-symbols-outlined md-16 w-[22px] ml-[3px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">close</span>
                            </div>

                            <div class="active:bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                                <span class="material-symbols-outlined md-16 w-[16px] mr-[5px]">folder</span>
                                <p class="text-[#E5E5E5] text-[13px]">telegram-desktop</p>
                                <span class="material-symbols-outlined md-16 w-[22px] p-[3px] ml-[3px] hover:bg-gray-500 rounded-sm hover:text-white">close</span>
                            </div>
                        </div>

                        <div class="w-full flex items-center h-[22px] text-[13px] text-[#999999] px-[16px]">
                            <p>src</p>
                            <span class="material-symbols-outlined md-16 w-[22px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">chevron_right</span>
                            <p>telegram-desktop</p>
                        </div>
                    </div>

                    {move || match data.get() {
                        None => view! { <p>"Loading..."</p> }.into_view(),
                        Some(_) =>view! { 
                        }.into_view()
                    }}
                    
                    //folder view body 
                    <div class="w-full flex flex-wrap" id="folder_view_body">
                        <div id="test" class="flex grid max-sm:grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 w-full gap-4 px-[25px] py-[10px]">
                        </div>
                    </div>
                </div>

                // share tab
                <div id="share_tab"></div>
            </div>

            <Bottomnav/>
        </div>
    }
}