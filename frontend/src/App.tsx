import { createResource, type Component, onCleanup, Switch, Match } from "solid-js";
import wretch from "wretch";
import { System } from "./types";

const App: Component = () => {
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
                            <p class="text-lg">CPU usage {sysdata().cpu.toFixed(2)}%</p>
                            <div class="h-6 bg-gray-400 w-full rounded">
                                <div
                                    class="bg-green-500 h-full transition-width duration-500 rounded"
                                    style={{ width: `${sysdata().cpu}%` }}
                                />
                            </div>
                            <p class="text-lg">
                                RAM usage:{" "}
                                {`${sysdata().ram.used.num} ${sysdata().ram.used.suffix} / ${
                                    sysdata().ram.total.num
                                } ${sysdata().ram.total.suffix}`}
                            </p>
                            <div class="h-6 bg-gray-400 w-full rounded">
                                <div
                                    class="bg-red-500 h-full transition-width duration-500 rounded"
                                    style={{ width: `${sysdata().ram.percent}%` }}
                                />
                            </div>
                            <p class="text-lg">
                                Swap usage:{" "}
                                {`${sysdata().swap.used.num} ${sysdata().swap.used.suffix} / ${
                                    sysdata().swap.total.num
                                } ${sysdata().swap.total.suffix}`}
                            </p>
                            <div class="h-6 bg-gray-400 w-full rounded">
                                <div
                                    class="bg-blue-500 h-full transition-width duration-500 rounded"
                                    style={{ width: `${sysdata().swap.percent}%` }}
                                />
                            </div>
                        </>
                    )}
                </Match>
            </Switch>
        </div>
    );
};

export default App;
