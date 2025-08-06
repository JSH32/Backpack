import * as React from "react"
import { Icon } from "@chakra-ui/icons"
import { Page } from "layouts/Page"
import Router from "next/router"
import PlusIcon from "assets/icons/plus.svg"
import { Authenticated } from "components/Authenticated"
import {
  Box,
  Divider,
  Flex,
  Heading,
  Text,
  Stack,
  useToast,
  Button,
  useDisclosure,
} from "@chakra-ui/react"
import api from "helpers/api"
import { useForm } from "react-hook-form"
import { AlbumList } from "components/Album/AlbumList"
import AlbumModal from "components/Album/AlbumModal"
import { AlbumFormData } from "components/Album/AlbumModal"

const Albums: React.FC = () => {
  const toast = useToast()
  const { isOpen, onOpen, onClose } = useDisclosure()
  const albumForm = useForm()
  const [listReload, setListReload] = React.useState(0)

  const createAlbumCallback = React.useCallback(async (data: any) => {
    api.album.create(data).then(() => {
      toast({
        title: "Album created",
        description: "Your album has been created successfully",
        status: "success",
        duration: 3000,
        isClosable: true
      })
      setListReload(listReload + 1)
    }).catch(error => {
      toast({
        title: "Error",
        description: error.body.message,
        status: "error",
        duration: 5000,
        isClosable: true
      })
    })
  }, [toast, onClose, albumForm])

  const updateCallback = React.useCallback(async (albumId: string, data: AlbumFormData) => {
    return api.album.update(albumId, data)
      .then(() => {
        toast({
          title: "Album updated",
          description: "Album has been updated successfully",
          status: "success",
          duration: 3000,
          isClosable: true
        })
      }).catch(error => {
        toast({
          title: "Error",
          description: error.body.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      })
  }, [])

  const deleteCallback = React.useCallback(async (id: string) => {
    api.album.delete(id)
      .then(() => {
        toast({
          title: "Album deleted",
          description: "Album has been deleted successfully",
          status: "success",
          duration: 3000,
          isClosable: true
        })
      }).catch(error => {
        toast({
          title: "Error",
          description: error.body.message,
          status: "error",
          duration: 5000,
          isClosable: true
        })
      })
  }, [])

  return (
    <Authenticated>
      <Page title="Albums">
        <Flex mt="7em" minH="100vh" justify="center" mb={5}>
          <Box w={{ base: "90vw", md: "70vw" }} maxW="1200px">
            <Stack spacing={4}>
              <Heading>Albums</Heading>
              <Text as="h2" fontSize="sm" color="gray.400">
                Create albums to share with others
              </Text>

              <Button
                onClick={onOpen}
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
                zIndex={4}
              >
                <Icon w={5} h={5} as={PlusIcon} />
              </Button>

              <AlbumModal onSubmit={createAlbumCallback} onClose={onClose} isOpen={isOpen} />

              <Divider />

              <AlbumList
                key={listReload}
                onLoad={(page) => api.album.list(page, "@me")}
                onDelete={deleteCallback}
                updateAlbum={updateCallback}
                onAlbumOpen={albumId => Router.push(`/album/${albumId}`)}
              />
            </Stack>
          </Box>
        </Flex>
      </Page>
    </Authenticated>
  )
}

export default Albums