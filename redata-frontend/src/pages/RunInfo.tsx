import {useParams} from "react-router";

function RunInfo() {
    let { runId } = useParams();

    return (
        <div>
            <p>Run Data will go here for View {runId} </p>

        </div>
    )
}

export default RunInfo
