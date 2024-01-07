use web_sys::{
    window,
};

pub fn open_dialog(dialog_id:&str){
    let window=window().unwrap();
    let dialog_bg=window.document().unwrap().get_element_by_id(dialog_id).unwrap();
    dialog_bg.class_list().add_1("ease-in-out").unwrap();
    dialog_bg.class_list().add_1("block").unwrap();
    dialog_bg.class_list().add_1("duration-1000").unwrap();
    dialog_bg.class_list().add_1("delay-2000").unwrap(); 
}