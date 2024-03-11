import { Link } from "react-router-dom";
import { Folder } from "../types/definitions";
import { MdFileOpen, MdFolder, MdFolderShared } from "react-icons/md";

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
    return(
        <footer className="fixed bottom-0 h-[25px] bg-blue-500 px-[7px] left-0 right-0">
            <div className="flex">
                <p className="px-[5px] py-[2px] text-[13px]">anvel</p>
                <div className="flex ml-auto">
                    <div className="flex items-center justify-center text-[13px] ">
                        <Link to="/" onMouseEnter={()=>showToast("sign_up")} onMouseLeave={()=>showToast("sign_up")} className="relative inline-block px-[15px] py-[2px] hover:bg-blue-400">
                            <span id="sign_up" className="absolute text-gray-300 none flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-14 h-[25px] min-w-[150px]">Create an account</span>
                            <span>Sign up</span>
                        </Link>
                        <Link to="/" onMouseEnter={()=>showToast("login")} onMouseLeave={()=>showToast("login")} className="relative inline-block px-[15px] py-[2px] hover:bg-blue-400">
                            <span id="login" className="absolute text-gray-300 none flex items-center justify-center bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-14 h-[25px] min-w-[150px]">Login to your account</span>
                            <span>Login</span>
                        </Link>
                    </div>
                    <div className="flex items-center justify-center text-[13px] border-dotted border-l-[1px] border-gray-500">
                        <button onClick={()=>{
                           props.data.onlyFolders()
                        }} onMouseEnter={()=>showToast("folder_count")} onMouseLeave={()=>showToast("folder_count")} className="relative inline-block px-[15px] py-[2px] hover:bg-blue-400">
                            <span id="folder_count" className="absolute none flex items-center justify-center text-gray-300 bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-10 h-[25px] min-w-[150px]">{folderCount.length!==0?(<p>View folders : {folderCount.length}</p>):(<p>No folder</p>)}</span>
                            <div className="flex gap-1 items-center ">
                                <MdFolder/>
                                <span>Folder: {folderCount.length}</span>
                            </div>
                        </button>
                        <button onClick={()=>{
                            props.data.onlyFiles()
                        }} onMouseEnter={()=>showToast("file_count")} onMouseLeave={()=>showToast("file_count")} className="relative inline-block px-[15px] py-[2px] hover:bg-blue-400">
                            <span id="file_count" className="absolute none flex items-center justify-center text-gray-300 bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-10 h-[25px] min-w-[150px]">{fileCount.length!==0?(<p>View files : {fileCount.length}</p>):(<p>No file</p>)}</span>
                            <div className="flex gap-1 items-center ">
                                <MdFileOpen/>
                                <span>File: {fileCount.length}</span>
                            </div>
                        </button>
                        <Link to="/" onMouseEnter={()=>showToast("shared_folder")} onMouseLeave={()=>showToast("shared_folder")} className="relative inline-block px-[10px] py-[2px] hover:bg-blue-400">
                            <span id="shared_folder" className="absolute none flex items-center justify-center text-gray-300 bg-[#252525] z-10 -mt-8 border-[1px] border-[var(--theme-gray)] -ml-6 h-[25px] min-w-[150px]">View shared folders</span>
                            <div className="flex gap-1 items-center ">
                                <MdFolderShared/>
                                <span>Shared Folder: 8</span>
                            </div>
                        </Link>
                    </div>
                </div>
            </div>
        </footer>
    )
}
