import * as React from "react"
import { Page } from "layouts/Page"
import type { NextPage } from "next"
import { Heading, VStack, Text } from "@chakra-ui/react"

const Page404: NextPage = () => {
    return (
        <Page title="404">
            <VStack
                h="100vh"
                justifyContent="center"
                textAlign="center">
                <Heading
                    bgGradient="linear(to-r, primary.400, primary.300)"
                    bgClip="text"
                    fontSize="6xl">
                    404
                </Heading>
                <Text fontSize="2xl">Page not found</Text>
            </VStack>
        </Page>
    )
}

export default Page404