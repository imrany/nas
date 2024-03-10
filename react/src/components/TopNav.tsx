// @flow strict
import { MdContentCopy, MdEdit, MdOutlineExpandMore, MdWifi } from "react-icons/md";
import { openDialog } from "./Feedback";
import { Link } from "react-router-dom";

type Props={
    data:{
        name:string
    }
}
function TopNav(props:Props) {
    window.onclick = function(event:any) {
        if (!event.target.matches('.dropbtn')) {
            var dropdowns = document.getElementsByClassName("dropdown-content");
            var i;
            for (i = 0; i < dropdowns.length; i++) {
                var openDropdown = dropdowns[i];
                if (openDropdown.classList.contains('block')) {
                    openDropdown.classList.remove('block');
                }
            }
        }
    }

    function showDropdownMenu(){
        let dropdown_list=document.getElementById("dropdown_list");
        dropdown_list?.classList.toggle("block");
    }
    
    return (
        <nav className="fixed bg-[#151515] px-[12px] top-0 left-0 right-0 z-10">
            <div className="font-semibold text-[13px] flex justify-between min-h-[48px] items-center text-white">
                <div className="dropdown">
                    <button onClick={showDropdownMenu} className="flex dropbtn justify-center items-center py-[4px] px-[12px] cursor-pointer h-[24px]">
                        <p className="dropbtn">Anvel</p>
                        <MdOutlineExpandMore className="w-[25px] h-[25px] dropbtn p-[3px]"/>
                    </button>
                    <div id="dropdown_list"  className="dropdown-content">
                        <Link to="/docs" className="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                            <MdContentCopy className="w-[25px] h-[25px] pr-[6px]"/>
                            <p>Documentation</p>
                        </Link>
                        <div className="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                            <MdEdit className="w-[25px] h-[25px] pr-[6px]"/>
                            <p>Customize</p>
                        </div>
                        
                        <div onClick={()=>openDialog("network_dialog")} className="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                            <MdWifi className="w-[25px] h-[25px] pr-[6px]"/>
                            <p>Network Info</p>
                        </div>
                    </div>
                </div>
                <div className="text-[#C2C2C2] flex font-medium min-w-[10%] items-center">
                    <p className="rounded-md bg-[#252525] py-[2px] px-2 mr-2">Directory</p>
                    <p className="root_path_indicator capitalize">{props.data.name}</p>
                </div>
                <div className="text-[#C2C2C2] flex min-w-[10vw] justify-around">
                    <button
                        className="hover:text-white ml-[10px] active:text-white focus:text-white"
                        onClick={()=>openDialog("network_dialog")}
                    >
                     Network info
                     </button>
                </div>
            </div>
        </nav>
    );
};

export default TopNav;