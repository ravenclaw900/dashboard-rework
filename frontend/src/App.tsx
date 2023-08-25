import { createResource, Show } from "solid-js";
import wretch from "wretch";
import { makeTimer } from "@solid-primitives/timer";
import { SystemData } from "./types";
import { GraphCard, StatsCard } from "./Cards";

function App() {
    const [sysdata, { refetch: refetchSystem }] = createResource<SystemData>(() =>
        wretch("http://localhost:5252/api/system").get().json(),
    );

    makeTimer(() => void refetchSystem(), 2000, setInterval);

    return (
        <div class="bg-gray-50 h-screen flex gap-3">
            <Show when={sysdata()}>
                {(sysdata) => (
                    <>
                        <GraphCard sysdata={sysdata()} />
                        <StatsCard sysdata={sysdata()} />
                    </>
                )}
            </Show>
        </div>
    );
}

export default App;
