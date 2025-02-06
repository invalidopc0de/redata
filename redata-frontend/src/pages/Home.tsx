import {Container, TextInput} from "@mantine/core";
import {MagnifyingGlass} from "@phosphor-icons/react";

function Home() {

    return (
        <>
            <Container>
                <TextInput placeholder="What data are you looking for?"
                           leftSection={<MagnifyingGlass size={16} />
                           }
                />
            </Container>
        </>
    )
}

export default Home
