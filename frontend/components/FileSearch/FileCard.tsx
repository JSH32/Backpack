import { Box, Button, Flex, Heading, HStack, Text, Tooltip, useColorModeValue, Icon, Image } from "@chakra-ui/react"
import { dateToString, getExtension, isExtImage } from "helpers/util"
import * as React from "react"
import InfoIcon from "assets/icons/info.svg"
import DeleteIcon from "assets/icons/trash.svg"
import { FileData } from "@/client"

const FileCard: React.FC<{ 
    file: FileData, 
    onDetails: (file: FileData) => void, 
    onDelete: (file: FileData) => void
}> = ({ file, onDetails, onDelete }) => {
    const ext = getExtension(file.name)

    return <Box 
        bg={useColorModeValue("white", "gray.700")}
        rounded="lg"
        boxShadow="lg"
        position="relative">
        { isExtImage(ext) ? <Image
            w="200px"
            h="200px"
            objectFit="cover"
            src={file.thumbnailUrl}
            alt={`Picture of ${file.url}`}
            roundedTop="lg"
        /> : <Flex 
            minW="200px" 
            minH="200px"
            roundedTop="lg" 
            bg={useColorModeValue("gray.200", "gray.600")} 
            align="center"
            textAlign="center"
            justify="center">
            <Heading noOfLines={1} w={180}>{ext !== "" ? ext : "FILE"}</Heading>
        </Flex> }
        <Box p="4">
            <Flex
                justifyContent="space-between" 
                alignContent="center">
                <Box
                    fontSize="1xl"
                    fontWeight="semibold"
                    as="h4"
                    maxW="120px"
                    lineHeight="tight"
                    noOfLines={1}>
                    {file.name}
                </Box>
                <HStack>
                    <Tooltip 
                        label="File details">
                        <Button 
                            onClick={() => onDetails(file)}
                            variant="ghost"
                            size="s">
                            <Icon as={InfoIcon} />
                        </Button>
                    </Tooltip>
                    <Tooltip
                        label="Delete file">
                        <Button 
                            onClick={() => onDelete(file)}
                            color="red.500"
                            variant="ghost" 
                            size="s">
                            <Icon as={DeleteIcon} />
                        </Button>
                    </Tooltip>
                </HStack>
            </Flex>
            <Text color="gray.500">{dateToString(new Date(file.uploaded))}</Text>
        </Box>
    </Box>
}

export default FileCard