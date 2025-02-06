import {Container, TextInput} from "@mantine/core";
import {MagnifyingGlass} from "@phosphor-icons/react";
import {useSearchParams} from "react-router";

function Search() {
    let [searchParams] = useSearchParams();
    return (
        <div>
            <Container>
                    <TextInput placeholder="What data are you looking for?"
                               leftSection={<MagnifyingGlass size={16} />
                                }
                    />

                    Search results will go here for <i>{searchParams.get("q")}</i>
            </Container>
        </div>
    )
}

export default Search
