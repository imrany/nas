import { Link } from "react-router-dom";
import { Folder } from "../types/definitions";

type Props={
    data:{
        folders:Folder
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
    return(
        <footer className="fixed bottom-0 h-[25px] bg-blue-500 px-[7px] left-0 right-0">
            <div className="flex">
                <p className="px-[5px] py-[2px] text-[13px]">anvel</p>
                <div className="flex ml-auto text-[13px]">
                    <Link to="/" className="mx-[3px] px-[5px] py-[2px] hover:bg-blue-400">Folder: {folderCount.length}</Link>
                    <Link to="/" className="mx-[3px] px-[5px] py-[2px] hover:bg-blue-400">File: {fileCount.length}</Link>
                    <Link to="/" className="mx-[3px] px-[5px] py-[2px] hover:bg-blue-400">Shared Folder: 8</Link>
                </div>
            </div>
        </footer>
    )
}