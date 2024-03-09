export function OfflineDialog(){
    const close_dialog=()=>{
        let dialog_bg=document.getElementById("offline_dialog");
        dialog_bg?.classList.add("ease-in-out");
        dialog_bg?.classList.remove("block");
        dialog_bg?.classList.add("duration-1000");
        dialog_bg?.classList.add("delay-2000");
    };
    return(
        <div id="offline_dialog" className="fixed top-0 bottom-0 left-0 right-0 z-20 bg-[#151515]/70 none">
            <div className="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" className="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div className="w-[452px] h-[162px]"> 
                        <h2 className="text-white font-medium text-base">Lost Connection</h2>
                        <div className="text-[13px] mt-[4px] mb-[13px] pb-[24px] text-[#999999]">
                            <p>
                                We are unable to reconnect you. Please check that you are online before trying again. If the problem persists, it means the development environment has reached an error state and you can try restarting it. Don't worry, your work will still be here when you get back.
                            </p>
                        </div>
                        <div className="flex justify-end items-center">
                            <button onClick={()=>window.location.reload()} className="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                Restart
                            </button>
                            <button onClick={close_dialog} className="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                Reconnect
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export function FeedbackDialog(){
    const close_dialog=()=>{
        let dialog_bg=document.getElementById("offline_dialog");
        dialog_bg?.classList.add("ease-in-out");
        dialog_bg?.classList.remove("block");
        dialog_bg?.classList.add("duration-1000");
        dialog_bg?.classList.add("delay-2000");
    };
    return(
        <div id="feedback_dialog" className="fixed top-0 bottom-0 left-0 right-0 z-20 bg-[#151515]/70 none">
            <div className="flex flex-col justify-center items-center h-[100vh]">
                <div id="dialog" className="text-white items-center flex flex-col bg-[#252525] justify-center p-[24px] focus:ring-1 focus:ring-violet-300">
                    <div className="flex ml-auto mb-[8px] justify-end h-[22px] pb-[4px] text-white">
                        <span onClick={close_dialog} className="material-symbols-outlined md-16 cursor-pointer">close</span>
                    </div>    
                    <div className="w-[452px] h-[162px]"> 
                        <div id="feedback_container"></div>
                        <div className="flex justify-end items-center">
                            <button onClick={close_dialog} className="mr-[12px] py-[4px] px-[16px] hover:bg-[#EDFFA1] border-none h-[28px] w-[100px] text-[13px] text-[#1D1D1D] rounded-sm bg-[#EDFFA5]">
                                Close
                            </button>
                            <button onClick={()=>window.location.reload()} className="rounded-sm border-[1px] border-violet-300 mr-[12px] py-[4px] hover:text-white h-[28px] w-[70px] text-[13px] text-[#C2C2C2]">
                                Reload
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

