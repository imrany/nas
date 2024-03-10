import { Link } from "react-router-dom";

export default function Footer(){
    return(
        <footer className="fixed bottom-0 h-[22px] py-[2px] bg-[#151515] px-[7px] left-0 right-0">
            <div className="flex text-[12px] text-[#808080]">
                <p>anvel</p>
                <div className="flex ml-auto text-[12px]">
                    <Link to="/" className="mx-[3px] px-[5px]">Folder: 2</Link>
                    <Link to="/" className="mx-[3px] px-[5px]">File: 49</Link>
                    <Link to="/" className="mx-[3px] px-[5px]">Shared Folder: 8</Link>
                </div>
            </div>
        </footer>
    )
}