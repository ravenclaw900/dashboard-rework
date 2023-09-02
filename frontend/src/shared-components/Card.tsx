import { JSX } from "solid-js";

function Card(props: { children: JSX.Element }) {
    return (
        <div class="flex-1 border border-gray-800 rounded bg-white shadow-md">{props.children}</div>
    );
}

export default Card;
