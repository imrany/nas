export function openDialog(dialog_id:string){
    let dialog_bg=document.getElementById(dialog_id);
    dialog_bg?.classList.add("ease-in-out");
    dialog_bg?.classList.toggle("none");
    dialog_bg?.classList.add("duration-1000");
    dialog_bg?.classList.add("delay-2000"); 
}