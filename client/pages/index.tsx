import type { NextPage } from "next"

import * as React from "react"
import UploadIcon from "/assets/icons/upload.svg"
import { Heading, Icon, Text, VStack } from "@chakra-ui/react"
import { Page } from "layouts/Page"
import { observer } from "mobx-react-lite"
import { useAppInfo } from "helpers/info"

const Home: NextPage = observer(() => {
    // File counter
    const [count] = React.useState(420)

    const appInfo = useAppInfo()

    return <Page title={appInfo?.appName}>
        <VStack
            h="100vh"
            justifyContent="center"
            textAlign="center">
                <Heading
                    bgGradient="linear(to-r, primary.400, primary.300)"
                    bgClip="text"
                    fontSize="6xl">
                    {appInfo?.appName}</Heading>
            <Text fontSize="xl">{appInfo?.appDescription}</Text>
            <Text fontSize="2xl">{count} <Icon as={UploadIcon}/></Text>
        </VStack>
    </Page>
})

export default Home
