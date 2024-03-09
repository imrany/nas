import { FaLinkedin, FaTwitter } from "react-icons/fa";
import { Link } from "react-router-dom";

export default function Footer(){
    const date=new Date();
    let year=date.getFullYear()
    return(
        <footer className="fixed bottom-0 h-[22px] py-[2px] bg-[#151515] px-[7px] left-0 right-0">
            <div className="flex text-[12px] text-[#808080]">
                <p>anvel</p>
                <div className="flex ml-auto text-[12px]">
                    <a href="/" className="mx-[3px] px-[5px]">Folder: 2</a>
                    <a href="/" className="mx-[3px] px-[5px]">File: 49</a>
                    <a href="/" className="mx-[3px] px-[5px]">UTF-8</a>
                </div>
            </div>
        </footer>
    )
}