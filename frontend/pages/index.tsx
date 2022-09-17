import type { NextPage } from "next"

import * as React from "react"
import UploadIcon from "/assets/icons/upload.svg"
import { Heading, HStack, Icon, Text, VStack } from "@chakra-ui/react"
import { Page } from "layouts/Page"
import { observer } from "mobx-react-lite"
import { useAppInfo } from "helpers/info"

const Home: NextPage = observer(() => {
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
            <HStack>
                <Text fontSize="2xl">{appInfo?.uploadedFiles}</Text>
                <Icon as={UploadIcon} fontSize="xl"/>
            </HStack>
        </VStack>
    </Page>
})

export default Home
