import { Box, Flex } from "@chakra-ui/react"
import * as React from "react"
import Header from "components/Header"

export const Page: React.FC<{
    children?: JSX.Element | JSX.Element[]
}> = ({ children }) => {
    return <>
        <Header/>
        <Box minH="100vh" h="100%">
            <Flex justifyContent="center" w="100%">
                <Box
                    maxW={2000} 
                    w="100vw">{children}</Box>
            </Flex>
        </Box>
    </>
}