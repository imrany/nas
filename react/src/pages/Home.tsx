import { MdArrowBack, MdClose, MdFolder } from "react-icons/md";
import Footer from "../components/Footer";
import SideNav from "../components/SideNav";
import TopNav from "../components/TopNav";
import { useEffect, useState } from "react";
import { ErrorBody, Folder } from "../types/definitions"

export default function Home(){
    let [name,setName]=useState("")
    let [folders,setFolders]=useState<Folder>({
        contents:[
            {
                name:"",
                root:"",
                path:"",
                metadata:{
                    is_file:false,
                    file_extension:""
                }
            }
        ]
    })
    let [error,setError]=useState<ErrorBody>({
        message:""
    })
    async function open(url:string){
        try {
            const response=await fetch(url,{
                method:"POST",
                headers:{
                    "content-type":"application/json"
                },
                body:JSON.stringify({
                    root:localStorage.getItem("path")
                })
            })
    		document.title=localStorage.getItem("path")?`${localStorage.getItem("path")}`:"Anvel • Home"
            let path:any=localStorage.getItem("path");
            let parts = path.split("/");
            setName(parts[parts.length - 1]);
            const parseRes:any=await response.json()
            if(response.ok){
                setFolders(parseRes)
            }else{
                setError(parseRes)
            }
        } catch (error:any) {
            console.error(error.message)
        }
    }

    useEffect(()=>{
        open("http://localhost:8000/api/directory_content")
	},[])
    return(
        <div className="min-h-[100vh] bg-[#1d1d1d]">
            <TopNav data={{name}}/>
            <div className="flex">
                <SideNav data={{folders,error,open}}/>

                <div className="mt-[48px] flex-grow mb-[22px] text-[#999999]">
                    {/*  folder view */}
                    <div id="folder_view">
                        {/* folder view nav */}
                        <div id="folder_view_nav" className="fixed overflow-hidden left-[200px] right-0 top-[46px] h-[57px]">
                            <div className="flex w-full bg-[#151515]">
                                {localStorage.getItem("path")==="/"?"":(
                                    <div onClick={()=>{
                                        let path:any=localStorage.getItem("path")!==null?localStorage.getItem("path"):""
                                        let newPath=path.slice(0,path?.lastIndexOf("/"))===""?path:path.slice(0,path?.lastIndexOf("/"))
                                        localStorage.setItem("path",newPath)
                                        open("http://localhost:8000/api/directory_content")
                                    }} title="Previous" className="bg-[#151515] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] w-[50px] h-[35px] flex items-center">
                                        <MdArrowBack className="w-[18px] h-[18px] mr-[5px]"/>
                                    </div>
                                )}
                                <div className="bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                                    <MdFolder className="w-[18px] h-[18px] mr-[5px]"/>
                                    <p className="text-[#E5E5E5] text-[13px] capitalize root_path_indicator">{name}</p>
                                    <MdClose className="w-[22px] h-[22px] ml-[3px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white" onClick={()=>{
                                        localStorage.removeItem("path");
                                        window.location.reload();
                                    }}/>
                                </div>
                            </div>

                            <div id="root_path_indicator_nav" className="w-full flex items-center h-[22px] text-[13px] bg-[#1d1d1d] text-[#999999] px-[16px] root_path_indicator">
                            </div>
                        </div>
                        {/* folder view body  */}
                        <div className="w-full flex flex-wrap mt-[35px]" id="folder_view_body">
                            <div id="test" className="ml-[200px] grid max-sm:grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 w-full gap-4 px-[25px] py-[13px]">
                            </div>
                        </div>
                    </div>

                    {/* share tab */}
                    <div id="share_tab"></div>
                </div>
            </div>
            <Footer/>
        </div>
    )
}