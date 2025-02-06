import '@mantine/core/styles.css';
import './App.css'

import {MantineProvider} from '@mantine/core';
import Search from "./pages/Search.tsx";
import Home from "./pages/Home.tsx";
import {BrowserRouter, Route, Routes} from "react-router";
import RunInfo from "./pages/RunInfo.tsx";
import Explorer from "./pages/Explorer.tsx";

function App() {

  return (
      <MantineProvider>
          <BrowserRouter>
              <Routes>
                  <Route index element={<Home />} />
                  <Route path={"search"} element={<Search/>} />
                  <Route path={"run/:runId"} element={<RunInfo />} />
                  <Route path={"explorer/:viewId?"} element={<Explorer />} />
              </Routes>
          </BrowserRouter>
      </MantineProvider>
  )
}

export default App
