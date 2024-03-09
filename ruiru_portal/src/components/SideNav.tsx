// @flow strict

import { useEffect, useState } from "react"

function SideNav() {
    let [folders,setFolders]=useState([])
    async function open(url:string,root:any){
        try {
            const response=await fetch(url,{
                method:"POST",
                headers:{
                    "content-type":"application/json"
                },
                body:JSON.stringify({
                    root
                })
            })
            const parseRes=await response.json()
            console.log(parseRes)
            setFolders(parseRes.contents)
        } catch (error:any) {
            console.error(error.message)
        }
    }

    let closure: Closure<dyn FnMut()> = Closure::new(move|| {
        create_resource(|| (), |_| async move { 
            match open(,path_dir.as_str()).await {
                Ok(data) => {
                    let dom_elem=web_sys::window().unwrap().document().unwrap().get_element_by_id("folders").unwrap();
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
                        let _file_extension=js_sys::Reflect::get(&metadata, &JsValue::from_str("file_extension"))
                        .map_err(|_| JsValue::from_str("Failed to access file_extension property")).unwrap();
        
                        // Convert the filename to a Rust String
                        let name_str = name.as_string().unwrap_or_default();
                        let path_str = path.as_string().unwrap_or_default();
                
                        let folder=format!("
                            <a href='#' id='folders_{name_str}' class='flex items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                <span class='material-symbols-outlined md-16 pr-[3px]'>folder</span>
                                <p class='text-[#e5e5e5 text-[11px] uppercase'>{name_str}</p>
                            </a>
                        "); 
                        let file=format!("
                            <a href='#' id='folders_{name_str}' class='flex items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                <span class='material-symbols-outlined md-16 pr-[3px]'>draft</span>
                                <p class='text-[#e5e5e5 text-[11px] uppercase'>{name_str}</p>
                            </a>
                        "); 
                     
                        let item_element =web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
                        // item_element.set_class_name("resize");

                        if !is_file.clone().as_bool().unwrap(){
                            item_element.set_inner_html(&folder);
                            dom_elem.append_child(&Node::from(item_element)).unwrap();

                            let path_str_copy=path_str.clone();
                            let open_folder: Closure<dyn FnMut()> = Closure::new(move|| {
                                let path=path_str_copy.clone();
                                web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("path",&path).unwrap(); 
                                web_sys::window().unwrap().location().reload().unwrap();
                            });
                            let btn=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("folders_{name_str}").as_str()).unwrap();
                            btn.add_event_listener_with_callback("dblclick", &open_folder.as_ref().unchecked_ref()).unwrap();
                            open_folder.forget();
                        }else {
                            item_element.set_inner_html(&file);
                            dom_elem.append_child(&Node::from(item_element)).unwrap();

                            let path_str_copy=path_str.clone();
                            let open_file: Closure<dyn FnMut()> = Closure::new(move|| {
                                let path=path_str_copy.clone();
                                wasm_bindgen_futures::spawn_local(open_file("http://localhost:8000/api/open".to_string(),path))
                            });
                            let btn=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("folders_{name_str}").as_str()).unwrap();
                            btn.add_event_listener_with_callback("dblclick", &open_file.as_ref().unchecked_ref()).unwrap();
                            open_file.forget();
                        }
                        
                        
                    }
                }
                Err(e) => { 
                    web_sys::console::error_1(&e.into());
                }
            }
            match fetch_data("http://localhost:8000/api/shared_folder").await {
                Ok(data) => {
                    web_sys::console::log_1(&data.clone().into());
                    let dom_elem=web_sys::window().unwrap().document().unwrap().get_element_by_id("shared_folder").unwrap();
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
                        let _file_extension=js_sys::Reflect::get(&metadata, &JsValue::from_str("file_extension"))
                        .map_err(|_| JsValue::from_str("Failed to access file_extension property")).unwrap();
        
                        // Convert the filename to a Rust String
                        let name_str = name.as_string().unwrap_or_default();
                        let path_str = path.as_string().unwrap_or_default();
                
                        let folder=format!("
                            <a href='#' id='shared_folders_{name_str}' class='flex items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                <span class='material-symbols-outlined md-16 pr-[3px]'>folder</span>
                                <p class='text-[#e5e5e5 text-[11px] uppercase'>{name_str}</p>
                            </a>
                        "); 
                        let file=format!("
                            <a href='#' id='shared_folders_{name_str}' class='flex items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                <span class='material-symbols-outlined md-16 pr-[3px]'>draft</span>
                                <p class='text-[#e5e5e5 text-[11px] uppercase'>{name_str}</p>
                            </a>
                        "); 
                     
                        let item_element =web_sys::window().unwrap().document().unwrap().create_element("div").unwrap();
        
                        
                        if !is_file.clone().as_bool().unwrap(){
                            item_element.set_inner_html(&folder);
                            dom_elem.append_child(&Node::from(item_element)).unwrap();

                            let path_str_copy=path_str.clone();
                            let open_folder: Closure<dyn FnMut()> = Closure::new(move|| {
                                let path=path_str_copy.clone();
                                web_sys::window().unwrap().local_storage().unwrap().unwrap().set_item("path",&path).unwrap(); 
                                web_sys::window().unwrap().location().reload().unwrap();
                            });
                            let btn=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("shared_folders_{name_str}").as_str()).unwrap();
                            btn.add_event_listener_with_callback("dblclick", &open_folder.as_ref().unchecked_ref()).unwrap();
                            open_folder.forget();
                        }else {
                            item_element.set_inner_html(&file);
                            dom_elem.append_child(&Node::from(item_element)).unwrap();

                            let path_ref=path_str.clone();
                            let open_file: Closure<dyn FnMut()> = Closure::new(move|| {
                                let path=path_ref.clone();
                                wasm_bindgen_futures::spawn_local(open_file("http://localhost:8000/api/open".to_string(),path))
                            });
                            let btn=web_sys::window().unwrap().document().unwrap().get_element_by_id(&format!("shared_folders_{name_str}").as_str()).unwrap();
                            btn.add_event_listener_with_callback("dblclick", &open_file.as_ref().unchecked_ref()).unwrap();
                            open_file.forget();
                        }
                        
                    }
                }
                Err(e) => { 
                    web_sys::console::error_1(&e.into());
                }
            }
        });
    });
    

   async function handleFetchData(url:string){
    try{
        const response=await fetch(url,{
            method:"GET"
        })
        const parseRes=await response.json()
        console.log(parseRes)
    }catch(error:any){
        console.error(error.message)
    }
   }
   useEffect(()=>{
        open("http://localhost:8000/api/directory_content",localStorage.getItem("path"))
   },[])
    return (
        <div id="sidebar" className="overflow-hidden h-[100vh] fixed pb-[12px] bottom-[18px] left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
            <div className="flex flex-col mb-3">
                <div className="h-[33px] flex items-center text-[#999999] uppercase pl-[12px] pr-[8px]">
                    <button className="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">draft</button>
                    <button className="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">search</button>
                    <button onClick={()=>window.location.reload()} className="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">refresh</button>
                </div>
                {/* folders */}
                <div className="resize-y">
                    <div className="flex items-center text-[11px] uppercase px-[8px] h-[35px] hover:text-white text-[#e5e5e5]">
                        <p className="pl-[12px]">EXPLORER</p>
                        <span className="material-symbols-outlined md-16 text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">more_horiz</span>
                    </div>
                    <div id="folders" className="sidebar_folders overflow-y-auto pb-[12px] h-[45vh]">
                        {folders?(
                            <div>
                            </div>
                        ):"No folder"}
                    </div>
                </div>

                {/* shared folder */}
                <div className="resize-y">
                    <div className="flex items-center cursor-pointer hover:text-white text-[#e5e5e5] text-[11px] uppercase px-[8px] h-[35px]">
                        <p className="pl-[12px]">SHARED FOLDER</p>
                        <span className="material-symbols-outlined md-16 text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">more_horiz</span>
                    </div>
                    <div id="shared_folder" className="sidebar_shared_folder pb-[12px] overflow-y-auto h-[45vh]">
                    </div>
                </div>

                {/* search */}
                <div id="search"></div>
                
            </div>
        </div>
    );
};

export default SideNav;