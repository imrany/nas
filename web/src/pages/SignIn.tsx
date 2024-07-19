import bg2 from "../assets/background/bg2.png";
import { useContext } from "react";
import { GlobalContext } from "../context";

type Props={
    data:{
        backgroundImage:string,
        changeBackground:any
    }
}

export default function SignIn(props:Props){
    let { API_URL }=useContext(GlobalContext)

    async function handleSubmit(e:any){
        try{
            e.preventDefault()
            console.log(e.target.username.value)
        }catch(error:any){
            console.log(error)
        }
    }
    return(
        <div style={!props.data.backgroundImage.includes("primary-01")&&props.data.backgroundImage!=="default"?{background: `linear-gradient(0deg, rgba(0, 0, 0, 0.7), rgba(0, 0, 0, 0.7)),url('${props.data.backgroundImage}') top no-repeat`, backgroundSize:"cover", backgroundAttachment:"fixed"}:props.data.backgroundImage==="default"?{background: `linear-gradient(0deg, rgba(0, 0, 0, 0.2), rgba(0, 0, 0, 0.2)),url('${bg2}') top no-repeat`, backgroundSize:"cover", backgroundAttachment:"fixed"}:{background: `var(--${props.data.backgroundImage})`}} className="min-h-[100vh] text-white">
            <div className="flex items-center justify-center h-screen">
                <form onSubmit={handleSubmit} className="flex flex-col blur-effect items-center shadow-md justify-center rounded-md py-[30px] px-[40px]">
                    <div className="flex flex-col mb-2 gap-1">
                        <label htmlFor="username">Username</label>
                        <input type="text" name="username" required className="px-2 blur-effect py-1 w-full rounded-sm text-black focus:border-none focus:outline-none border-none focus:ring-none" id="username"/>
                    </div>
                    <div className="flex flex-col mb-4 gap-1">
                        <label htmlFor="password">Password</label>
                        <input type="password" name="password" required className="px-2 blur-effect py-1 w-full rounded-sm text-black focus:border-none focus:outline-none border-none focus:ring-none" id="password" minLength={8} maxLength={24}/>
                    </div>
                    <button className="py-1 px-[16px] border-none w-full text-[#1D1D1D] rounded-md bg-[var(--yellow-primary-01)] active:bg-[var(--yellow-primary-02)]">Sign in</button>
                </form>
            </div>
        </div>
    )
}
