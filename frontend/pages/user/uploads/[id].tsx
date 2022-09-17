import * as React from "react"
import { Resource } from "components/Resource"
import Router, { useRouter } from "next/router"
import { Result } from "components/Result"
import { convertBytes, dateToString, getExtension, isExtImage } from "helpers/util"
import TrashIcon from "assets/icons/trash.svg"
import { Page } from "layouts/Page"

import {
    Box,
    chakra,
    Image,
    Flex,
    Spinner,
    Text,
    Divider,
    Table,
    TableCaption,
    Tr,
    Td,
    Tbody,
    Link,
    Button,
    Icon,
    useToast
} from "@chakra-ui/react"
import { FileData } from "@/client"
import api from "helpers/api"

const FileInfo: React.FC = () => {
    const router = useRouter()
    const { id } = router.query
    const [fileInfo, setFileInfo] = React.useState<FileData | null>(null)
    const [isError, setIsError] = React.useState(false)
    const toast = useToast()

    React.useEffect(() => {
        api.file.info(id as string)
            .then(setFileInfo)
            .catch(() => setIsError(true))
    }, [])

    const deleteCallback = React.useCallback(() => {
        api.file.deleteFile(fileInfo?.id as string)
            .then(() => {
                toast({
                    title: "File deleted",
                    description: `Deleted ${fileInfo?.name}`,
                    status: "success",
                    duration: 5000,
                    isClosable: true
                })
                Router.push("/user/uploads")
            })
            .catch(error => {
                toast({
                    title: "Error",
                    description: error.body.message,
                    status: "error",
                    duration: 5000,
                    isClosable: true
                })
            })
    }, [fileInfo])

    return <Box className="fileinfo">
        {isError ? <Page title="Invalid File">
            <Result type="error" title="Invalid resource">
                <Text><chakra.span fontWeight="bold">{id}</chakra.span> was an invalid resource</Text>
            </Result>
        </Page> : <>
            {fileInfo ? <Resource
                backPath="/user/uploads"
                title={fileInfo.name}
                right={<Button onClick={deleteCallback} w={5} colorScheme="red">
                    <Icon as={TrashIcon} />
                </Button>}
                id={fileInfo.id}>
                <Box mt={5}>
                    {isExtImage(getExtension(fileInfo.name)) ?
                        <Image mb="10px" maxH="300px" src={fileInfo.url} alt={fileInfo.name} /> : <></>}
                    <Divider />
                    <Table wordBreak="break-all" sx={{ "font-variant-numeric": "unset;" }}>
                        <Tbody>
                            <Tr>
                                <Td>ID</Td>
                                <Td>{fileInfo.id}</Td>
                            </Tr>
                            <Tr>
                                <Td>Name</Td>
                                <Td>{fileInfo.name}</Td>
                            </Tr>
                            <Tr>
                                <Td>Original Name</Td>
                                <Td>{fileInfo.originalName}</Td>
                            </Tr>
                            <Tr>
                                <Td>Size</Td>
                                <Td>{convertBytes(fileInfo.size)}</Td>
                            </Tr>
                            <Tr>
                                <Td>Date Uploaded</Td>
                                <Td>{dateToString(new Date(fileInfo.uploaded))}</Td>
                            </Tr>
                            <Tr>
                                <Td>URL</Td>
                                <Td><Link color="primary.300" target="_blank" href={fileInfo.url}>{fileInfo.url}</Link></Td>
                            </Tr>
                            <Tr>
                                <Td>Hash</Td>
                                <Td>{fileInfo.hash}</Td>
                            </Tr>
                        </Tbody>
                        <TableCaption>File information</TableCaption>
                    </Table>
                </Box>
            </Resource> :
                <Page>
                    <Flex h="100vh" justify="center" align="center">
                        <Spinner size="xl" />
                    </Flex>
                </Page>}
        </>}
    </Box>
}

export default FileInfo