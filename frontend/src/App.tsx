import { createResource, onCleanup, Switch, Match } from "solid-js";
import wretch from "wretch";
import { System } from "./types";

function DisplayBar(props: { message: string; percent: number; color: string; name: string }) {
    return (
        <>
            <p class="text-lg">
                {props.name} usage: {props.message}
            </p>
            <div class="h-6 bg-gray-400 w-full rounded">
                <div
                    class={`h-full transition-width duration-500 rounded ${props.color}`}
                    style={{ width: `${props.percent}%` }}
                />
            </div>
        </>
    );
}

function App() {
    const [sysdata, { refetch: refetchSystem }] = createResource<System>(() =>
        wretch("http://localhost:5252/api/system").get().json(),
    );

    const handle = setInterval(refetchSystem, 2000);

    onCleanup(() => clearInterval(handle));

    return (
        <div class="bg-gray-50 h-screen">
            <Switch>
                <Match when={sysdata.error !== undefined}>Couldn't fetch system data</Match>
                <Match when={sysdata()}>
                    {(sysdata) => (
                        <>
                            <DisplayBar
                                message={`${sysdata().cpu}%`}
                                percent={sysdata().cpu}
                                name="CPU"
                                color="bg-green-500"
                            />
                            <DisplayBar
                                message={`${sysdata().ram.used.num} ${
                                    sysdata().ram.used.suffix
                                } / ${sysdata().ram.total.num} ${sysdata().ram.total.suffix}`}
                                percent={sysdata().ram.percent}
                                name="RAM"
                                color="bg-red-500"
                            />
                            <DisplayBar
                                message={`${sysdata().swap.used.num} ${
                                    sysdata().swap.used.suffix
                                } / ${sysdata().swap.total.num} ${sysdata().swap.total.suffix}`}
                                percent={sysdata().swap.percent}
                                name="Swap"
                                color="bg-blue-500"
                            />
                        </>
                    )}
                </Match>
            </Switch>
        </div>
    );
}

export default App;
