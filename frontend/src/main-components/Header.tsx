import logo from "../../assets/dietpi-logo.png";

function Header(props: { toggleDarkMode: () => void; toggleMenu: () => void; darkMode: boolean }) {
    return (
        <header class="h-12 flex items-center justify-between bg-dplime-primary px-2">
            <button class="i-fa-bars text-3xl" onClick={() => props.toggleMenu()} />
            <img src={logo} alt="DietPi logo" />
            <button
                class="text-3xl"
                title={props.darkMode ? "Dark mode" : "Light mode"}
                classList={{ "i-fa-sun": !props.darkMode, "i-fa-moon": props.darkMode }}
                onClick={() => props.toggleDarkMode()}
            />
        </header>
    );
}

export default Header;
