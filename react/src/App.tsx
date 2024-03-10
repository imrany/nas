import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { ToastContainer } from 'react-toastify';
import { Toaster } from "react-hot-toast";
import ErrorPage from './pages/ErrorPage';
import Docs from './pages/Docs';
import Home from "./pages/Home";
import { OfflineDialog } from "./components/dialogs";
import LandingPage from "./pages/LandingPage";

function App() {
  return (
    <BrowserRouter>
      <ToastContainer 
        autoClose={5000}
        hideProgressBar={false}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="light"
      />
      <Toaster/>
      <Routes>
        <Route path="/" element={<LandingPage/>} />
        <Route path="/home" element={<Home/>} />
        <Route path="/docs" element={<Docs />} />
        <Route path="*" element={<ErrorPage />} />
      </Routes>
      <OfflineDialog/>
    </BrowserRouter>
  )
}

export default App
