import * as React from "react"

import { convertBytes } from "helpers/util"
import { Icon } from "@chakra-ui/icons"
import { FileSearch } from "components/FileSearch"
import { Page } from "layouts/Page"
import Router from "next/router"
import UploadIcon from "assets/icons/upload.svg"
import { Authenticated } from "components/Authenticated"
import { 
    Box, 
    Divider, 
    Flex, 
    Heading, 
    Stack, 
    Stat, 
    StatLabel, 
    StatNumber, 
    ToastId, 
    useToast, 
    Button
} from "@chakra-ui/react"
import api from "helpers/api"

const UploadFiles: React.FC = () => {
    const [usage, setUsage] = React.useState<string>()
    const [searchReload, setSearchReload] = React.useState(0)
    const [currentUploading, setCurrentUploading] = React.useState(0)

    const toast = useToast()
    const toastIdRef = React.useRef<ToastId>()
    
    React.useEffect(() => {
        api.file.stats()
            .then(stats => setUsage(convertBytes(stats.usage)))
            .catch(() => setUsage("0 Bytes"))
    }, [searchReload])

    const shadowUploader = React.useRef(null)
    const uploadButtonCallback = React.useCallback(() => {
        (shadowUploader.current as any)?.click()
    }, [shadowUploader])
    
    React.useEffect(() => {
        if (currentUploading > 0) {
            if (!toastIdRef.current) {
                toastIdRef.current = toast({
                    title: "Uploading",
                    description: `Currently uploading ${currentUploading} files`,
                    status: "info",
                    duration: null,
                    isClosable: false
                })
            } else {
                toast.update(toastIdRef.current, {
                    title: "Uploading",
                    description: `Currently uploading ${currentUploading} files`,
                    status: "info",
                    duration: null,
                    isClosable: false
                })
            }
        } else {
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            toast.close(toastIdRef.current!)
            toastIdRef.current = undefined
        }
    }, [currentUploading])

    const uploadCallback = React.useCallback((event: any) => {
        const uploadPromises = []

        for (const file of event.target.files) {
            setCurrentUploading(count => count + 1)
            uploadPromises.push(api.file.upload({ uploadFile: file })
                .finally(() => {
                    setCurrentUploading(count => count - 1)
                }))
        }

        Promise.allSettled(uploadPromises)
            .then(() => {
                setSearchReload(searchReload + 1)
            })
    }, [searchReload])

    const deleteCallback = React.useCallback((id: string) => {
        return api.file.deleteFile(id)
            .then(api.file.stats)
            .then(stats => setUsage(convertBytes(stats.usage)))
            .catch(() => setUsage("0 Bytes"))
    }, [])

    return <Authenticated>
         <Page title="Uploads">
            <Flex mt="7em" minH="100vh" justify="center" mb={5}>
                <Box w={{ base: "90vw", md: "70vw" }} maxW="1200px">
                    <Stack spacing={4}>
                        <Heading>Uploads</Heading>
                        <input type="file" ref={shadowUploader} onChange={uploadCallback} style={{display: "none"}} multiple/>
                        <Button
                            onClick={uploadButtonCallback}
                            bg="primary.500"
                            _hover={{
                                bg: "primary.600"
                            }}
                            color="white"
                            position="fixed"
                            bottom="3em"
                            right="3em"
                            borderRadius="50px"
                            w="60px"
                            h="60px"
                            zIndex={4}>
                                <Icon w={5} h={5} as={UploadIcon}/>
                            </Button>
                        <Divider/>
                        <Box>
                            <Stat>
                                <StatLabel>Usage</StatLabel>
                                <StatNumber>{usage}</StatNumber>
                            </Stat>
                        </Box>
                        <Divider/>
                        <FileSearch
                            key={searchReload}
                            onSearch={(page, query) => api.file.list(page, query)}
                            onDelete={deleteCallback}
                            onFileDetails={fileId => Router.push(`/user/uploads/${fileId}`)}/>
                    </Stack>
                </Box>
            </Flex>
        </Page>
    </Authenticated>
}

export default UploadFiles
