// @flow strict
import { useEffect, useState } from "react"
import { MdDrafts, MdFileOpen, MdFilePresent, MdFolder, MdMoreHoriz, MdRefresh, MdSearch } from "react-icons/md"

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

    // async function handleFetchData(url:string){
    //     try{
    //         const response=await fetch(url,{
    //             method:"GET"
    //         })
    //         const parseRes=await response.json()
    //         console.log(parseRes)
    //     }catch(error:any){
    //         console.error(error.message)
    //     }
    // }

    useEffect(()=>{
        open("http://localhost:8000/api/directory_content",localStorage.getItem("path"))
    },[])
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
                    <div id="folders" className="sidebar_folders overflow-y-auto pb-[12px] pt-1 h-[45vh]">
                        {folders?(
                            <>
                                <a href='#' id='folders_{name_str}' className='flex items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                    <MdFolder className="w-[20px] h-[20px] pr-[3px]"/>
                                    <p className='text-[#e5e5e5 text-[11px] uppercase'>Downloads</p>
                                </a>

                                <a href='#' id='folders_{name_str}' className='flex items-center mx-[1px] px-3 py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300'>
                                    <MdFileOpen className="w-[20px] h-[20px] pr-[3px]"/>
                                    <p className='text-[#e5e5e5 text-[11px] uppercase'>music.mp3</p>
                                </a>
                            </>
                        ):"No folder"}
                    </div>
                </div>

                {/* shared folder */}
                <div className="resize-y">
                    <div className="flex items-center cursor-pointer hover:text-white text-[#e5e5e5] text-[11px] uppercase px-[8px] h-[35px]">
                        <p className="pl-[12px]">SHARED FOLDER</p>
                        <MdMoreHoriz className="text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]"/>
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