import { createSignal, lazy } from "solid-js";
import { makePersisted } from "@solid-primitives/storage";
import { Route, Routes } from "@solidjs/router";
import Header from "./main-components/Header";
import SidebarMenu from "./main-components/SidebarMenu";

const Home = lazy(() => import("./pages/Home"));

function App() {
    // eslint-disable-next-line solid/reactivity
    const [darkMode, setDarkMode] = makePersisted(createSignal(false));
    const [showMenu, setShowMenu] = createSignal(true);

    return (
        <div classList={{ dark: darkMode() }} class="flex">
            <SidebarMenu show={showMenu()} />
            <div class="w-full">
                <Header
                    toggleDarkMode={() => setDarkMode((darkMode) => !darkMode)}
                    toggleMenu={() => setShowMenu((menu) => !menu)}
                    darkMode={darkMode()}
                />
                <div class="h-screen flex gap-3 bg-gray-50">
                    <Routes>
                        <Route path="/test" component={<Home darkMode={darkMode()} />} />
                    </Routes>
                </div>
            </div>
        </div>
    );
}

export default App;
