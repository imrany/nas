// @flow strict
import { MdEdit, MdFileOpen, MdFolder, MdMoreHoriz, MdRefresh, MdSearch } from "react-icons/md"
import { openDialog, openFile } from "./actions"
import { ErrorBody, Folder } from "../types/definitions"

type Props = {
    data:{
        folders:Folder,
        error:ErrorBody
        open:any
    }
};
function SideNav(props:Props) {
    return (
        <div id="sidebar" className="overflow-hidden h-[100vh] fixed pb-[12px] bottom-[18px] left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
            <div className="flex flex-col mb-3">
                <div className="h-[33px] flex items-center text-[#999999] uppercase pl-[12px] pr-[8px]">
                    <button className="focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">
                        <MdFileOpen className="w-[18px] h-[18px]"/>
                    </button>
                    <button className="focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">
                        <MdSearch className="w-[18px] h-[18px]"/>
                    </button>
                    <button onClick={()=>window.location.reload()} className="focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">
                        <MdRefresh className="w-[18px] h-[18px]"/>
                    </button>
                </div>
                {/* folders */}
                <div className="resize-y">
                    <div className="flex items-center text-[11px] uppercase px-[8px] h-[35px] hover:text-white text-[#e5e5e5]">
                        <p className="pl-[12px]">EXPLORER</p>
                        <MdMoreHoriz className="text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]"/>
                    </div>
                    <div id="folders" className="sidebar_folders overflow-y-auto pb-[33px] pt-1 h-screen">
                        <div className="flex flex-col">
                            {props.data.folders?props.data.folders.contents.map(content=>{
                                return(
                                    <div key={content.name}>
                                        {content.metadata.is_file?(
                                            <button key={content.name} onClick={()=>{
                                                if(!content.metadata.is_file){
                                                    localStorage.setItem("path",content.path)
                                                    props.data.open("http://localhost:8000/api/directory_content")
                                                }else{
                                                    openFile("http://localhost:8000/api/open",content.path)
                                                }
                                            }} className='flex flex-grow items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                                <MdFileOpen className="w-[20px] h-[20px] pr-[3px]"/>
                                                <p className='text-[#e5e5e5 text-[11px] uppercase'>{content.name.length<20?content.name:(<>{content.name.slice(0,16)}...</>)}</p>
                                            </button>
                                        ):(
                                            <button onClick={()=>{
                                                localStorage.setItem("path",content.path)
                                                props.data.open("http://localhost:8000/api/directory_content")
                                            }} key={content.name} id='folders_{name_str}' className='flex flex-grow items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                                <MdFolder className="w-[20px] h-[20px] pr-[3px]"/>
                                                <p className='text-[#e5e5e5 text-[11px] uppercase'>{content.name.length<20?content.name:(<>{content.name.slice(0,16)}...</>)}</p>
                                            </button>
                                        )}
                                    </div>
                                )
                            }):(
                                <div className="flex flex-col justify-start items-start py-2 px-3">
                                    <p>{props.data.error.message}</p>
                                    <button onClick={()=>openDialog("open_folder_dialog")} className="mt-2 underline flex gap-2 text-blue-500 items-center justify-center">
                                        <MdEdit className="w-[16px] h-[16px]"/>
                                        <span>Edit path</span>
                                    </button>
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* search */}
                <div id="search"></div>
                
            </div>
        </div>
    );
};

export default SideNav;