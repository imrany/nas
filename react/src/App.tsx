import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import ErrorPage from './pages/ErrorPage';
import Docs from './pages/Docs';
import Home from "./pages/Home";
import { OpenFolderDialog } from "./components/dialogs";
import LandingPage from "./pages/LandingPage";
import Layout from "./pages/Layout";
import bg1 from "./assets/background/bg_1.png";
import { useEffect, useState } from "react";
import { UserPreference } from "./types/definitions";

function App() {
  let userPreference:UserPreference={
    backgroundImage:""
  }
  let [backgroundImage,setBackgroundImage]=useState("")
  let path=localStorage.getItem("path")
  window.oncontextmenu=(e:any)=>{
    e.preventDefault()
  }

  useEffect(()=>{
    setBackgroundImage(bg1)
  },[backgroundImage])
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/welcome" element={path===null?<LandingPage data={{backgroundImage}}/>:<Navigate to="/"/>} />
        <Route path="/" element={path!==null?<Layout/>:<Navigate to="/welcome"/>}>
          <Route index element={<Home data={{backgroundImage}}/>} />
          <Route path="docs" element={<Docs />} />
        </Route>
        <Route path="*" element={<ErrorPage />} />
      </Routes>
      <OpenFolderDialog/>
    </BrowserRouter>
  )
}

export default App
