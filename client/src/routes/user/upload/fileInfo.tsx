import * as React from "react"
import { Resource } from "components/resource"
import { useHistory, useParams } from "react-router-dom"
import { deleteFile, FileData, getFile } from "api"
import { Result } from "components/result"
import { convertBytes, dateToString, getExtension, isExtImage } from "bpkutil"
import "./fileInfo.scss"
import TrashIcon from "assets/icons/trash.svg"
import { Page } from "components/page"

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
    Link,
    Button,
    Icon,
    useToast
} from "@chakra-ui/react"

export const FileInfo: React.FC = () => {
    const { id } = useParams<any>()
    const [fileInfo, setFileInfo] = React.useState<FileData | null>(null)
    const [isError, setIsError] = React.useState(false)
    const history = useHistory()
    const toast = useToast()

    React.useEffect(() => {
        getFile(id)
            .then(setFileInfo)
            .catch(() => setIsError(true))
    }, [])

    const deleteCallback = React.useCallback(() => {
        deleteFile(fileInfo.id)
            .then(() => {
                toast({
                    title: "File deleted",
                    description: `Deleted ${fileInfo.name}`,
                    status: "success",
                    duration: 5000,
                    isClosable: true
                })
                history.push("/user/uploads")
            })
            .catch(error => {
                toast({
                    title: "Error",
                    description: error.response.data.message,
                    status: "error",
                    duration: 5000,
                    isClosable: true
                })
            })
    }, [fileInfo])

    return <>
        {isError ? <Page>
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
                        <Image mb="10px" maxH="500px" src={fileInfo.url} alt={fileInfo.name} /> : <></>}
                    <Divider />
                    <Table wordBreak="break-all">
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
    </>
}