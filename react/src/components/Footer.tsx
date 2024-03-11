import { Link } from "react-router-dom";
import { Folder, Notifications } from "../types/definitions";
import { MdClose, MdFileOpen, MdFolder, MdFolderShared, MdNotifications, MdOutlineNotificationsActive, MdRadar,  } from "react-icons/md";
import { FaChevronDown, FaChevronUp } from "react-icons/fa";
import { useState } from "react";
import { TbCircleX } from "react-icons/tb";
import {motion, AnimatePresence} from "framer-motion";

type Props={
    data:{
        folders:Folder,
        onlyFolders:any,
        onlyFiles:any,
    }
}
export default function Footer(props:Props){
    let folderCount=[]
    let fileCount=[]
    props.data.folders.contents.forEach((item)=>{
        if(!item.metadata.is_file){
            folderCount.push(item)
        }else{
            fileCount.push(item)
        }
    })
    function showToast(id:string){
        let toast=document.getElementById(id)
        toast?.classList.contains("none")?toast?.classList.remove("none"):toast?.classList.add("none")
    }
    let notifications:Notifications[]=[
        {
            priority:"not important",
            message:"Hello welcome to anvel, contact our support via imranmat254@gmail.com for help."
        },
        {
            priority:"not important",
            message:"Turn on Hotspot or WIFI and connect with other person using anvel."
        },
        {
            priority:"important",
            message:"Zero connections"
        },
    ]
    return(
        <footer className="fixed bottom-0 h-[25px] bg-[#e0ff72] text-[#252525] px-[7px] left-0 right-0">
            <div className="flex">
                <p className="px-[5px] py-[2px] text-[13px]">anvel</p>
                <div className="flex ml-auto">
                    <div className="flex items-center justify-center text-[13px] ">
                        <Link to="/" onMouseEnter={()=>showToast("sign_up")} onMouseLeave={()=>showToast("sign_up")} className="relative inline-block px-[15px] py-[2px] hover:bg-[#EDFFA5]">
                            <span id="sign_up" className="absolute none text-gray-300 flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-14 h-[25px] min-w-[150px]">Create an account</span>
                            <span>Sign up</span>
                        </Link>
                        <Link to="/" onMouseEnter={()=>showToast("login")} onMouseLeave={()=>showToast("login")} className="relative inline-block px-[15px] py-[2px] hover:bg-[#EDFFA5]">
                            <span id="login" className="absolute none text-gray-300 flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-14 h-[25px] min-w-[150px]">Login to your account</span>
                            <span>Login</span>
                        </Link>
                    </div>

                    <div className="flex items-center justify-center text-[13px] border-dotted border-l-[1px] border-gray-500">
                        <button onClick={()=>{
                        }} onMouseEnter={()=>showToast("connection_count")} onMouseLeave={()=>showToast("connection_count")} className="relative inline-block px-[15px] py-[2px] hover:bg-[#EDFFA5]">
                            <span id="connection_count" className="absolute none flex items-center justify-center text-gray-300 bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-8 h-[25px] min-w-[150px]">Get Connected</span>
                            <div className="flex gap-1 items-center ">
                                <MdRadar className="w-[15px] h-[15px]"/>
                                <span>Connections</span>
                            </div>
                        </button>
                    </div>

                    <div className="flex items-center justify-center text-[13px] border-dotted border-l-[1px] border-gray-500">
                        <button onClick={()=>{
                           props.data.onlyFolders()
                        }} onMouseEnter={()=>showToast("folder_count")} onMouseLeave={()=>showToast("folder_count")} className="relative inline-block px-[15px] py-[2px] hover:bg-[#EDFFA5]">
                            <span id="folder_count" className="absolute none flex items-center justify-center text-gray-300 bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-10 h-[25px] min-w-[150px]">{folderCount.length!==0?(<p>View folders : {folderCount.length}</p>):(<p>No folder</p>)}</span>
                            <div className="flex gap-1 items-center ">
                                <MdFolder/>
                                <span>Folder: {folderCount.length}</span>
                            </div>
                        </button>
                        <button onClick={()=>{
                            props.data.onlyFiles()
                        }} onMouseEnter={()=>showToast("file_count")} onMouseLeave={()=>showToast("file_count")} className="relative inline-block px-[15px] py-[2px] hover:bg-[#EDFFA5]">
                            <span id="file_count" className="absolute text-gray-300 none flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-10 h-[25px] min-w-[150px]">{fileCount.length!==0?(<p>View files : {fileCount.length}</p>):(<p>No file</p>)}</span>
                            <div className="flex gap-1 items-center ">
                                <MdFileOpen/>
                                <span>File: {fileCount.length}</span>
                            </div>
                        </button>
                        <Link to="/" onMouseEnter={()=>showToast("shared_folder")} onMouseLeave={()=>showToast("shared_folder")} className="relative inline-block px-[10px] py-[2px] hover:bg-[#EDFFA5]">
                            <span id="shared_folder" className="absolute text-gray-300 none flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-6 h-[25px] min-w-[150px]">View shared folders</span>
                            <div className="flex gap-1 items-center ">
                                <MdFolderShared/>
                                <span>Shared Folder: 8</span>
                            </div>
                        </Link>

                        <div className="relative inline-block">
                            <div id="single_notifications"
                            className="text-gray-300 flex flex-col justify-center z-20 fixed right-2 bottom-8  w-[380px]"
                            >
                                {notifications.length!==0?(
                                    <motion.div
                                        initial={{ opacity: 0 }}
                                        animate={{ opacity: 1 }}
                                        className="flex flex-col gap-2"
                                    >
                                        <AnimatePresence>
                                            {notifications.map((notification)=>{
                                                let [message,setMessage]=useState(<>{notification.message.slice(0,30)}...</>)
                                                let [showChevronDown,setShowChevronDown]=useState(false)
                                                let icon
                                                switch (notification.priority) {
                                                    case "not important":
                                                        icon=(
                                                            <motion.div
                                                            whileHover={{
                                                                rotateZ: [0, -20, 20, -20, 20, -20, 20, 0],
                                                                transition: { duration: 0.5 },
                                                            }}
                                                            >
                                                                <MdOutlineNotificationsActive className="text-green-500 w-[20px] h-[20px]"/>
                                                            </motion.div>
                                                        )
                                                        break;
                                                
                                                    case "important":
                                                        icon=(
                                                            <motion.div
                                                            whileHover={{
                                                                rotateZ: [0, -20, 20, -20, 20, -20, 20, 0],
                                                                transition: { duration: 0.5 },
                                                            }}
                                                            >
                                                                <TbCircleX className="text-red-500 w-[20px] h-[20px]"/>
                                                            </motion.div>
                                                        )
                                                        break;
                                                }
                                                return(
                                                    <motion.div
                                                        initial={{ opacity: 0 }}
                                                        animate={{ opacity: 1 }}
                                                        exit={{ x: "12rem", opacity: 0 }}
                                                        transition={{ duration: 0.2 }}
                                                        layout
                                                        key={notification.message} id={`single_${notification.message}`} style={{boxShadow:"0px 8px 16px 0px rgba(0,0,0,0.7)"}} className="px-[12px] py-2 rounded-md flex gap-2 items-center bg-[#3c3c3c]">
                                                        {icon}
                                                        <p className="uppercase text-[11px] w-[250px]">{message}</p>
                                                        <div className="ml-auto flex gap-2 items-center">
                                                            {!showChevronDown?(
                                                                <button onClick={()=>{
                                                                    document.getElementById(`single_${notification.message}`)?.classList.replace("py-2","py-3")
                                                                    setShowChevronDown(true)
                                                                    setMessage(<>{notification.message}</>)
                                                                }} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                                    <FaChevronUp className="text-gray-400 w-[13px] h-[12px]"/>
                                                                </button>
                                                            ):
                                                                <button onClick={()=>{
                                                                    document.getElementById(`single_${notification.message}`)?.classList.replace("py-3","py-2")
                                                                    setShowChevronDown(false)
                                                                    setMessage(<>{notification.message.slice(0,30)}...</>)
                                                                }} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                                    <FaChevronDown className="text-gray-400 w-[13px] h-[12px]"/>
                                                                </button>
                                                            }
                                                            <button 
                                                                onClick={()=>{
                                                                    showToast(`single_${notification.message}`)
                                                                }} 
                                                                className="p-[5px] hover:bg-[#265f5f3d] flex items-center justify-center rounded-md"
                                                            >
                                                                <MdClose className="text-gray-400 w-[13px] h-[12px]"/>
                                                            </button>
                                                        </div>
                                                    </motion.div>
                                                )
                                            })}
                                        </AnimatePresence>
                                    </motion.div>
                                ):
                                    <div id="single_no_new_notifications" className="px-[12px] py-[8px] rounded-md flex items-center bg-[#3c3c3c]/70">
                                        <p className="uppercase text-[11px]">No New Notifications</p>
                                        <div className="ml-auto flex gap-2 items-center">
                                            <button onClick={()=>showToast("single_no_new_notifications")} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                <MdClose className="text-gray-400 w-[13px] h-[12px]"/>
                                            </button>
                                        </div>
                                    </div>
                                }
                            </div>
                        </div>

                        <div className="relative inline-block">
                            <div id="notification_dialog"
                            style={{boxShadow:"0px 8px 16px 0px rgba(0,0,0,0.7)"}} 
                            className="none text-gray-300 flex flex-col rounded-md justify-center bg-[#252525] z-20 fixed right-2 bottom-8  w-[380px]"
                            >
                                {notifications.length!==0?
                                <AnimatePresence>
                                    <motion.div
                                        initial={{ opacity: 0 }}
                                        animate={{ opacity: 1 }}
                                    >
                                        <div className="px-[12px] border-b-[1px] border-[#9999991A] py-[8px] rounded-t-md flex items-center bg-[#3c3c3c]/70">
                                            <p className="uppercase text-[11px]">Notifications</p>
                                            <div className="ml-auto flex gap-2 items-center">
                                                <button onClick={()=>showToast("notification_dialog")} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                    <FaChevronDown className="text-gray-400 w-[13px] h-[12px]"/>
                                                </button>
                                            </div>
                                        </div>
                                        {notifications.map((notification)=>{
                                            let [showChevronDown,setShowChevronDown]=useState(false)
                                            let [message,setMessage]=useState(<>{notification.message.slice(0,30)}...</>)
                                            let icon
                                            switch (notification.priority) {
                                                case "not important":
                                                    icon=(
                                                        <motion.div
                                                        whileHover={{
                                                            rotateZ: [0, -20, 20, -20, 20, -20, 20, 0],
                                                            transition: { duration: 0.5 },
                                                        }}
                                                        >
                                                            <MdOutlineNotificationsActive className="text-green-500 w-[20px] h-[20px]"/>
                                                        </motion.div>
                                                    )
                                                    break;
                                            
                                                case "important":
                                                    icon=(
                                                        <motion.div
                                                        whileHover={{
                                                            rotateZ: [0, -20, 20, -20, 20, -20, 20, 0],
                                                            transition: { duration: 0.5 },
                                                        }}
                                                        >
                                                            <TbCircleX className="text-red-500 w-[20px] h-[20px]"/>
                                                        </motion.div>
                                                    )
                                                    break;
                                            } 
                                            return(
                                                <motion.div
                                                    initial={{ opacity: 0 }}
                                                    animate={{ opacity: 1 }}
                                                    exit={{ x: "12rem", opacity: 0 }}
                                                    transition={{ duration: 0.2 }}
                                                    layout onMouseEnter={()=>{
                                                    // showToast(`btn_${notification.message}`)
                                                }} onMouseLeave={()=>{
                                                    // showToast(`btn_${notification.message}`)
                                                }} key={notification.message} id={`bar_${notification.message}`} className="px-[12px] py-2 flex gap-2 items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/20 active:bg-[#3c3c3c]/20">
                                                    {icon}
                                                    <p className="w-[250px]">{message}</p>
                                                    <div id={`btn_${notification.message}`} className="ml-auto flex gap-2 items-center">
                                                        {!showChevronDown?(
                                                            <button onClick={()=>{
                                                                document.getElementById(`bar_${notification.message}`)?.classList.replace("py-2","py-3")
                                                                setShowChevronDown(true)
                                                                setMessage(<>{notification.message}</>)
                                                            }} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                                <FaChevronUp className="text-gray-400 w-[13px] h-[12px]"/>
                                                            </button>
                                                        ):
                                                            <button onClick={()=>{
                                                                document.getElementById(`bar_${notification.message}`)?.classList.replace("py-3","py-2")
                                                                setShowChevronDown(false)
                                                                setMessage(<>{notification.message.slice(0,30)}...</>)
                                                            }} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                                <FaChevronDown className="text-gray-400 w-[13px] h-[12px]"/>
                                                            </button>
                                                        }
                                                        <button onClick={()=>showToast(`bar_${notification.message}`)} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                            <MdClose className="w-[13px] h-[12px]"/>
                                                        </button>
                                                    </div>
                                                </motion.div>
                                            )
                                        })}
                                    </motion.div>
                                </AnimatePresence>
                                :
                                    <div id="bar_no_new_notifications" className="px-[12px] py-[8px] rounded-md flex items-center bg-[#3c3c3c]/70">
                                        <p className="uppercase text-[11px]">No New Notifications</p>
                                        <div className="ml-auto flex gap-2 items-center">
                                            <button onClick={()=>showToast("bar_no_new_notifications")} className="p-[5px] hover:bg-[#3c3c3c]/90 flex items-center justify-center rounded-md">
                                                <MdClose className="text-gray-400 w-[13px] h-[12px]"/>
                                            </button>
                                        </div>
                                    </div>
                                }
                                
                            </div>
                            <button onClick={()=>{
                                document.getElementById("single_notifications")?.classList.add("none")
                                showToast("notification_dialog")
                            }} onMouseEnter={()=>showToast("notification_toast")} onMouseLeave={()=>showToast("notification_toast")} className="relative inline-block px-[15px] h-[25px] hover:bg-[#EDFFA5]">
                                <span id="notification_toast" className="absolute text-gray-300 none flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-[115px] h-[25px] min-w-[150px]">No New Notifications</span>
                                <div className="flex gap-1 items-center">
                                    <MdNotifications className="w-[17px] h-[17px]"/>
                                </div>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    )
}
