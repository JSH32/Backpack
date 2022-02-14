import * as React from "react"
import UploadIcon from "assets/icons/upload.svg"
import { Heading, Icon, Text, VStack } from "@chakra-ui/react"
import { Page } from "components/page"

export const Home: React.FC = () => {
    // File counter
    const [count] = React.useState(420)

    return <Page>
        <VStack 
            h="100vh"
            justifyContent="center"
            textAlign="center">
                <Heading
                    bgGradient="linear(to-l, #a061ff, #7d4aff)"
                    bgClip="text"
                    fontSize="6xl">
                    {import.meta.env.SNOWPACK_PUBLIC_APP_NAME}</Heading>
            <Text fontSize="xl">{import.meta.env.SNOWPACK_PUBLIC_APP_DESCRIPTION}</Text>
            <Text fontSize="2xl">{count} <Icon as={UploadIcon}/></Text>
        </VStack>
    </Page>
}