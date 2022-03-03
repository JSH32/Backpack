import { ArrowBackIcon } from "@chakra-ui/icons"
import * as React from "react"
import Router from "next/router"

import { 
    Box, 
    Flex,
    useColorModeValue,
    Text, 
    Stack, 
    Icon, 
    Button, 
    HStack, 
    Badge 
} from "@chakra-ui/react"

export const Resource: React.FC<{
    backPath: string,
    title: string,
    id: string,
    right?: JSX.Element | JSX.Element[],
    children?: JSX.Element | JSX.Element[]
}> = ({ backPath, title, id, right, children }) => {
    return <>
        <Box pos="fixed" w="100%">
            <Flex 
                justifyContent="center" 
                w="100%" 
                borderBottom={1}
                borderStyle="solid"
                borderColor={useColorModeValue("gray.200", "gray.900")}
                bg={useColorModeValue("gray.50", "gray.800")}>
                <Flex
                    py={{ base: 2 }}
                    px={{ base: 4 }}
                    minH="60px"
                    maxW={1000}
                    w="100vw"
                    justifyContent="space-between"
                    align="center">

                    <HStack spacing={5}>
                        <Button _hover={{ bg: useColorModeValue("gray.300", "gray.700") }} onClick={() => Router.push(backPath)} w={10} h={10}><Icon as={ArrowBackIcon}/></Button>
                        <Stack spacing={0.01}>
                            <Text fontWeight="bold">{title}</Text>
                            <HStack>
                                <Text>ID: </Text>
                                <Badge colorScheme="primary">{id}</Badge>
                            </HStack>
                        </Stack>
                    </HStack>
                    {right ? right : <></>}
                </Flex>
            </Flex>
        </Box>
        <Box bg="white.500" minH="100vh" h="100%">
            <Flex
                py={{ base: 2 }}
                px={{ base: 4 }}
                justifyContent="center" 
                w= "100%">
                    <Box 
                        mt="60px"
                        maxW={1000} 
                        w="100vw">
                        {children}
                    </Box>
            </Flex>
        </Box>
    </>
}