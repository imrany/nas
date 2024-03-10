import { toast } from "react-toastify";
import { useHotToast } from "../components/CustomHooks"

export function err_toast(msg:string){
    let screen_width=window.innerWidth;
    if(screen_width>640){
        toast.error(msg,{
            position: "bottom-left"
        })
    }else{
        useHotToast.error(msg,{
            position: "top-center"
        })
    }
}

export function success_toast(msg:string){
    let screen_width=window.innerWidth;
    if(screen_width>640){
        toast.success(msg,{
            position: "bottom-left"
        })
    }else{
        useHotToast.sucess(msg,{
            position: "top-center"
        })
    }
}

export function info_toast(msg:string){
    let screen_width=window.innerWidth;
    if(screen_width>640){
        toast.info(msg,{
            position: "bottom-left"
        })
    }else{
        useHotToast.info(msg,{
            position: "top-center"
        })
    }
}

export function openDialog(dialog_id:string){
    let dialog_bg=document.getElementById(dialog_id);
    dialog_bg?.classList.add("ease-in-out");
    dialog_bg?.classList.add("block");
    dialog_bg?.classList.add("duration-1000");
    dialog_bg?.classList.add("delay-2000"); 
}