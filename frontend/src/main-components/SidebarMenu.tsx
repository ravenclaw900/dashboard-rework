import { A } from "@solidjs/router";

function SidebarLink(props: { name: string; icon: string; href: string }) {
    return (
        <A
            class="h-12 flex cursor-pointer items-center text-2xl text-gray-400 hover:bg-gray-700"
            href={props.href}
        >
            <span class={`${props.icon} m-2`} />
            {props.name}
        </A>
    );
}

function SidebarMenu(props: { show: boolean }) {
    return (
        <div
            class="w-1/6 bg-gray-900 transition-width duration-1500"
            classList={{ "!w-0": !props.show }}
        >
            <div class="h-12 flex items-center justify-center whitespace-nowrap bg-dplime-dark text-3xl">
                DietPi Dashboard
            </div>
            <SidebarLink name="Statistics" icon="i-fa-database" href="/test" />
            <SidebarLink name="Processes" icon="i-fa-microchip" href="/" />
        </div>
    );
}

export default SidebarMenu;
