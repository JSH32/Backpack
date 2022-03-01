import * as React from "react"

import { 
    Button, 
    useColorModeValue, 
    Flex, 
    Icon, 
    Text 
} from "@chakra-ui/react"

import { 
    ChevronLeftIcon, 
    ChevronRightIcon 
} from "@chakra-ui/icons"

export const Pagination: React.FC<{
    pages: number,
    currentPage: number,
    range: number,
    onPageSelect(page: number): void
}> = ({ pages, currentPage, range, onPageSelect }) => {
    const prePages = []
    for (let i = currentPage - range; i !== currentPage; i++)
        if (i >= 1) prePages.push(i)

    const postPages = []
    for (let i = currentPage + 1; i - 1 !== currentPage + range; i++)
        if (i <= pages) postPages.push(i)

    return <Flex gap={2}>
        { currentPage !== 1 ? 
            <Button onClick={() => onPageSelect(currentPage - 1)}><Icon w={5} h={5} as={ChevronLeftIcon}/></Button> 
          : <Button disabled><Icon w={5} h={5} as={ChevronLeftIcon}/></Button>}

        { currentPage - range > 1 ? <Text color="gray.500">...</Text> : <></> }

        { prePages.map((pg, index) => <Button key={`pgIndex${index}'`} onClick={() => onPageSelect(pg)}>{pg}</Button>) }
        <Button bg={useColorModeValue("gray.300", "gray.600")}>{currentPage}</Button>
        { postPages.map((pg, index) => <Button key={`pgIndex${index}`} onClick={() => onPageSelect(pg)}>{pg}</Button>) }

        { currentPage + range < pages ? <Text color="gray.500">...</Text> : <></> }

        { currentPage + 1 > pages ? 
            <Button disabled><Icon w={5} h={5} as={ChevronRightIcon}/></Button> 
          : <Button onClick={() => onPageSelect(currentPage + 1)}><Icon w={5} h={5} as={ChevronRightIcon}/></Button>}
    </Flex>
}
