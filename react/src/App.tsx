import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import ErrorPage from './pages/ErrorPage';
import Docs from './pages/Docs';
import Home from "./pages/Home";
import { OfflineDialog, OpenFolderDialog } from "./components/dialogs";
import LandingPage from "./pages/LandingPage";
import Layout from "./pages/Layout";

function App() {
  let path=localStorage.getItem("path")
  window.oncontextmenu=(e:any)=>{
    e.preventDefault()
  }
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/welcome" element={path===null?<LandingPage/>:<Navigate to="/"/>} />
        <Route path="/" element={path!==null?<Layout/>:<Navigate to="/welcome"/>}>
          <Route index element={<Home/>} />
          <Route path="docs" element={<Docs />} />
        </Route>
        <Route path="*" element={<ErrorPage />} />
      </Routes>
      <OfflineDialog/>
      <OpenFolderDialog/>
    </BrowserRouter>
  )
}

export default App
