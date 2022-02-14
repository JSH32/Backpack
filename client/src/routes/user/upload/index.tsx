import * as React from "react"

import { getUsage, searchFile, uploadFile, deleteFile } from "api"
import { convertBytes } from "bpkutil"
import { Icon } from "@chakra-ui/icons"
import { FileSearch } from "components/filesearch"
import { Page } from "components/page"
import { useHistory } from "react-router-dom"
import UploadIcon from "assets/icons/upload.svg"

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

export const UploadFiles: React.FC = () => {
    const [usage, setUsage] = React.useState<string>()
    const [searchReload, setSearchReload] = React.useState(0)
    const [currentUploading, setCurrentUploading] = React.useState(0)

    const toast = useToast()
    const toastIdRef = React.useRef<ToastId>()
    
    const history = useHistory()
    
    React.useEffect(() => {
        getUsage()
            .then(bytes => setUsage(convertBytes(bytes)))
    }, [searchReload])

    const shadowUploader = React.useRef(null)
    const uploadButtonCallback = React.useCallback(() => {
        shadowUploader.current?.click()
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
            toast.close(toastIdRef.current)
            toastIdRef.current = undefined
        }
    }, [currentUploading])

    const uploadCallback = React.useCallback((event: any) => {
        const uploadPromises = []

        for (const file of event.target.files) {
            setCurrentUploading(count => count + 1)
            uploadPromises.push(uploadFile(file)
                .finally(() => {
                    setCurrentUploading(count => count - 1)
                }))
        }

        Promise.allSettled(uploadPromises)
            .then(() => {
                setSearchReload(searchReload + 1)
            })
    }, [searchReload])

    return <Page>
        <Flex mt="7em" minH="100vh" justify="center">
            <Box w={["90%", "70%"]}>
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
                        onSearch={searchFile} 
                        onDelete={deleteFile} 
                        onFileDetails={fileId => history.push(`/user/uploads/${fileId}`)}/>
                </Stack>
            </Box>
        </Flex>
    </Page>
}