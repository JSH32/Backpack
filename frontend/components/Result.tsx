import * as React from "react"

import {
    InfoIcon,
    WarningTwoIcon,
    CloseIcon
} from "@chakra-ui/icons"

import { 
    Box, 
    Flex,
    Heading 
} from "@chakra-ui/react"

export const Result: React.FC<{
    type: "info" | "warning" | "error" | JSX.Element,
    title: string,
    children?: JSX.Element | JSX.Element[]
}> = ({ type, title, children }) => {
    return <Flex minH="100vh"
        align="center"
        justify="center">
        <Box py={10} px={6} textAlign="center">
            {type === "info" && <InfoIcon boxSize="50px" color="blue.500"/>}
            {type === "warning" && <WarningTwoIcon boxSize="50px" color="orange.300"/>}
            {type === "error" && <Box display="inline-block">
                <Flex
                    flexDirection="column"
                    justifyContent="center"
                    alignItems="center"
                    bg="red.500"
                    rounded="50px"
                    w="55px"
                    h="55px"
                    textAlign="center">
                    <CloseIcon boxSize="20px" color="white" />
                </Flex>
            </Box>}
            {typeof type !== "string" && <>{type}</>}
            <Heading as="h2" size="xl" mt={6} mb={2}>{title}</Heading>
            {children}
        </Box>
    </Flex>
}