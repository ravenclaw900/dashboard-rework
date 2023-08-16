import { createResource, type Component, onCleanup, Switch, Match } from "solid-js";
import wretch from "wretch";
import { Cpu } from "./types/cpu";

const App: Component = () => {
    const [cpu, { refetch: refetchCpu }] = createResource<Cpu>(() =>
        wretch("http://localhost:5000/api/cpu-usage").get().json(),
    );

    const handle = setInterval(refetchCpu, 1000);

    onCleanup(() => clearInterval(handle));

    return (
        <div class="bg-gray-50 h-screen">
            <Switch>
                <Match when={cpu.error !== undefined}>Couldn't fetch CPU data</Match>
                <Match when={cpu()}>
                    {(cpu) => (
                        <>
                            <p class="text-lg">CPU usage {cpu().usage.toFixed(2)}%</p>
                            <div class="h-6 bg-gray-400 w-full rounded">
                                <div
                                    class="bg-green-500 h-full transition-width duration-500 rounded"
                                    style={{ width: `${cpu().usage}%` }}
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
