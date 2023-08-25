import { JSX, createEffect, onCleanup } from "solid-js";
import uPlot from "uplot";
import "uplot/dist/uPlot.min.css";
import prettyBytes from "pretty-bytes";
import { SystemData } from "./types";

function Card(props: { children: JSX.Element }) {
    return (
        <div class="bg-white rounded shadow-md border border-gray-800 flex-1">{props.children}</div>
    );
}

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

export { GraphCard, StatsCard };
