import { Box, Flex } from "@chakra-ui/react"
import * as React from "react"
import Header from "components/Header"
import { Meta } from "components/Meta"

export const Page: React.FC<{
    title?: string
    children?: JSX.Element | JSX.Element[] | React.ReactNode
}> = ({ title, children }) => {
    return <>
        <Meta
            title={title}
            description={title}/>
        <Header/>
        <Box minH="100vh" h="100%">
            <Flex justifyContent="center" w="100%">
                <Box w="100%">{children}</Box>
            </Flex>
        </Box>
    </>
}