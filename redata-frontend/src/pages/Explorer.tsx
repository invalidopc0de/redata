import {useParams, useSearchParams} from "react-router";

function Explorer() {
    let { viewId } = useParams();
    let [searchParams] = useSearchParams();

    return (
        <div>
            <p>Data will go here for View {viewId} </p>

            <p>Initial data will be loaded from Run {searchParams.get("initialRunId")}</p>

        </div>
    )
}

export default Explorer
