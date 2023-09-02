import { Show, createResource, createEffect, onCleanup } from "solid-js";
import wretch from "wretch";
import { makeTimer } from "@solid-primitives/timer";
import prettyBytes from "pretty-bytes";
import uPlot from "uplot";
import "uplot/dist/uPlot.min.css";
import { SystemData } from "../types";
import Card from "../shared-components/Card";

function DisplayBar(props: { message: string; percent: number; color: string; name: string }) {
    return (
        <>
            <p class="text-lg">
                {props.name} usage: {props.message}
            </p>
            <div class="h-6 w-full rounded bg-gray-400">
                <div
                    class={`h-full transition-width duration-500 rounded ${props.color}`}
                    style={{ width: `${props.percent}%` }}
                />
            </div>
        </>
    );
}

function StatsCard(props: { sysdata: SystemData }) {
    const prettyRamUsed = () => prettyBytes(props.sysdata.ram.used, { binary: true });
    const prettyRamTotal = () => prettyBytes(props.sysdata.ram.total, { binary: true });

    const prettySwapUsed = () => prettyBytes(props.sysdata.swap.used, { binary: true });
    const prettySwapTotal = () => prettyBytes(props.sysdata.swap.total, { binary: true });

    return (
        <Card>
            <DisplayBar
                message={`${props.sysdata.cpu}%`}
                percent={props.sysdata.cpu}
                name="CPU"
                color="bg-green-500"
            />
            <DisplayBar
                message={`${prettyRamUsed()} / ${prettyRamTotal()}`}
                percent={props.sysdata.ram.percent}
                name="RAM"
                color="bg-red-500"
            />
            <DisplayBar
                message={`${prettySwapUsed()} / ${prettySwapTotal()}`}
                percent={props.sysdata.swap.percent}
                name="Swap"
                color="bg-blue-500"
            />
        </Card>
    );
}

function GraphCard(props: { sysdata: SystemData }) {
    const createGraph = (el: HTMLDivElement) => {
        const series = [
            {
                label: "CPU",
                stroke: "#10b981",
                width: 3,
                scale: "%",
            },
            {
                label: "RAM",
                stroke: "#ef4444",
                width: 3,
                scale: "bytes",
            },
            {
                label: "Swap",
                stroke: "#3b82f6",
                width: 3,
                scale: "bytes",
            },
        ];

        const opts: uPlot.Options = {
            width: 500,
            height: 700,
            series: [{}, ...series],
            axes: [{}],
            scales: {
                bytes: {
                    distr: 3,
                },
            },
        };

        const data: number[][] = [[], [], [], []];

        // eslint-disable-next-line new-cap
        const graph = new uPlot(opts, data as uPlot.AlignedData, el);

        createEffect(() => {
            data[0].push(Date.now() / 1000);
            data[1].push(props.sysdata.cpu);
            data[2].push(props.sysdata.ram.used);
            data[3].push(props.sysdata.swap.used);
            graph.setData(data as uPlot.AlignedData);
        });

        onCleanup(() => {
            graph.destroy();
        });
    };

    return (
        <Card>
            <div ref={createGraph} />
        </Card>
    );
}

function Home(props: { darkMode: boolean }) {
    const [sysdata, { refetch: refetchSystem }] = createResource<SystemData>(() =>
        wretch("http://localhost:5252/api/system").get().json(),
    );

    makeTimer(() => void refetchSystem(), 2000, setInterval);

    createEffect(() => console.log(props.darkMode));

    return (
        <Show when={sysdata()}>
            {(sysdata) => (
                <>
                    <GraphCard sysdata={sysdata()} />
                    <StatsCard sysdata={sysdata()} />
                </>
            )}
        </Show>
    );
}

export default Home;
