import { MdArrowBack, MdClose, MdContentCopy, MdFolder, MdOpenInNew, MdShare } from "react-icons/md";
import Footer from "../components/Footer";
import SideNav from "../components/SideNav";
import TopNav from "../components/TopNav";
import { useEffect, useState } from "react";
import { ErrorBody, Folder, Content } from "../types/definitions"
import FileImage from "../assets/icons/file.png";
import FolderImage from "../assets/icons/folder.png";
import { openFile } from "../components/actions";
import { useNavigate } from "react-router-dom";

export default function Home(){
    const navigate=useNavigate()
    let [name,setName]=useState("")
    let [isLoading,setIsLoading]=useState(true)
    let [showCloseBtn,setShowCloseBtn]=useState(false)
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
   
    let [contents,setContents]=useState<Content[]>([
        {
            name:"",
            root:"",
            path:"",
            metadata:{
                is_file:false,
                file_extension:""
            }
        }
    ])
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
	    	    // setContents(parseRes.contents)
                // setFolders(parseRes)
                let withOutDotConfig:Folder={
                    contents:[]
                }
                parseRes.contents.forEach((content:Content) => {
                   if(content.name.slice(0,1)!=="."){
                    withOutDotConfig.contents.push(content)
                   }
                });
                setFolders(withOutDotConfig)
	    	    setContents(withOutDotConfig.contents)
            }else{
                setError(parseRes)
                navigate(`/error?error=${parseRes.message}`)
            }
            setIsLoading(false)
        } catch (error:any) {
            console.error(error.message)
            setIsLoading(false)
            navigate(`/error?error=${error.message}`)
        }
    }

    function onlyFolders(){
	    let arr:Folder={
            contents:[]
        }
        contents.map((content)=>{
            if(!content.metadata.is_file){
                arr.contents.push(content)
            }
            setFolders(arr)
        })
    }

    function onlyFiles(){
        let arr:Folder={
            contents:[]
        }
        contents.map((content)=>{
            if(content.metadata.is_file){
                arr.contents.push(content)
            }
            setFolders(arr)
        })
    }

    useEffect(()=>{
        open("http://localhost:8000/api/directory_content")
	},[])
    return(
        <>
            {isLoading?(
                <div className="bg-white text-[var(--theme-dark)] flex flex-col h-screen w-screen items-center justify-center">
                    <p className="text-lg">Loading...</p>
                </div>
            ):(
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
                                                let newPath:any;
                                                if(path.slice(0,path?.lastIndexOf("/"))===""||path.slice(0,path?.lastIndexOf("/"))===":"){
                                                    newPath="root"
                                                }else if(path==="shared"){
                                                    newPath=localStorage.getItem("previous")
                                                }else{
                                                    newPath=path.slice(0,path?.lastIndexOf("/"))
                                                }
                                                localStorage.setItem("path",newPath)
                                                open("http://localhost:8000/api/directory_content")
                                            }} title="Previous" className="bg-[#151515] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] w-[50px] h-[35px] flex items-center">
                                                <MdArrowBack className="w-[18px] h-[18px] mr-[5px]"/>
                                            </div>
                                        )}
                                        <div onMouseEnter={()=>setShowCloseBtn(true)} onMouseLeave={()=>setShowCloseBtn(false)} className="bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[128px] h-[35px] flex items-center">
                                            <MdFolder className="w-[18px] h-[18px] mr-[5px]"/>
                                            <p className="text-[#E5E5E5] mr-[3px] text-[13px] capitalize root_path_indicator">{name}</p>
                                            {showCloseBtn?(
                                                <MdClose className="p-[3px] w-[22px] h-[22px] bg-[#3c3c3c]/90 ml-auto rounded-sm text-white" onClick={()=>{
                                                    localStorage.removeItem("path");
                                                    window.location.reload();
                                                }}/>
                                            ):""}
                                        </div>
                                    </div>
                                </div>
                                {/* folder view body  */}
                                <div className="w-full flex flex-wrap mt-[35px]" id="folder_view_body">
                                    <div id="test" className="ml-[200px] grid max-sm:grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 w-full gap-4 px-[25px] py-[13px]">
                                        { folders&&folders.contents.map((content)=>{
                                            return(
                                                <div key={content.name} className="flex flex-col items-center text-center">
                                                    <button id={content.name} title={content.name}
                                                        onContextMenu={()=>{
                                                            let dropdown_list=document.getElementById(`context_list_${content.name}`);
                                                            dropdown_list?.classList.toggle("block");
                                                        }}
                                                        onDoubleClick={()=>{
                                                            if(!content.metadata.is_file){
                                                                localStorage.setItem("path",content.path)
                                                                open("http://localhost:8000/api/directory_content")
                                                            }else{
                                                                openFile("http://localhost:8000/api/open",content.path)
                                                            }
                                                        }}  className='flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:bg-[#3c3c3c]/90 focus:text-white dropdown_btn'>
                                                        {content.metadata.is_file?(<img src={FileImage} alt='file' className='w-[70px] h-[70px]'/>):(<img src={FolderImage} alt='folder' className='w-[70px] h-[70px]'/>)}
                                                        <div>
                                                            <p className='text-center'>{content.name.length<30?content.name:(<>{content.name.slice(0,30)}...</>)}</p>
                                                        </div>
                                                    </button>
                                                    <div id={`context_list_${content.name}`} className='dropdown-content  flex-wrap  w-[200px] mt-[50px] -ml-[5px] max-lg:-ml-[27px]'>
                                                        <div>
                                                            <div onClick={()=>{
                                                                if(content.metadata.is_file){
                                                                    openFile("http://localhost:8000/api/open",content.path)
                                                                }else{
                                                                    localStorage.setItem("path",content.path)
                                                                    open("http://localhost:8000/api/directory_content")
                                                                }
                                                            }} className='px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35 {name_str}_open_item'>
                                                                <MdOpenInNew className="w-[25px] h-[25px] pr-[6px]"/>
                                                                <p>Open</p>
                                                            </div>
                                                            <button onClick={()=>{
                                                                navigator.clipboard.writeText(content.path)
                                                            }} className='px-[12px] w-full py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35 {name_str}_open_item'>
                                                                <MdContentCopy className="w-[25px] h-[25px] pr-[6px]"/>
                                                                <p>Copy Path</p>
                                                            </button>
                                                            {content.metadata.is_file?(<div className='px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35'>
                                                                <MdShare className="w-[25px] h-[25px] pr-[6px]"/>
                                                                <p>Share</p>
                                                            </div>):""}
                                                        </div>
                                                    </div>
                                                </div>
                                            )
                                        })}
                                    </div>
                                </div>
                            </div>

                            {/* share tab */}
                            <div id="share_tab"></div>
                        </div>
                    </div>
                    <Footer data={{folders, onlyFolders, onlyFiles, open}}/>
                </div>
            )}
        </>
    )
}
