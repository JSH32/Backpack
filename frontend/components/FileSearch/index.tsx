/* eslint-disable react/no-children-prop */
import * as React from "react"
import { SearchIcon } from "@chakra-ui/icons"
import { useForm } from "react-hook-form"
import { Pagination } from "../Pagination"
import FileCard from "./FileCard"
import {
  Box,
  Flex,
  Heading,
  Input,
  InputGroup,
  InputLeftElement,
  Icon,
  Spinner,
  Text,
  Button,
  HStack,
  Checkbox,
  useDisclosure,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  Select,
  VStack,
  Badge,
  useToast
} from "@chakra-ui/react"
import { UploadData, Page_UploadData, AlbumData, Page_AlbumData } from "@/client"
import FolderPlusIcon from "assets/icons/folder.svg"

export const FileSearch: React.FC<{
  onSearch: (page: number, query?: string) => Promise<Page_UploadData>
  onDelete: (fileId: string) => Promise<void>
  onFileDetails: (fileId: string) => void
  onGetAlbums?: () => Promise<AlbumData[]>
  onAddFilesToAlbum?: (albumId: string, fileIds: string[]) => Promise<void>
}> = ({ onSearch, onDelete, onFileDetails, onGetAlbums, onAddFilesToAlbum }) => {
  const [searchResult, setSearchResult] = React.useState<Page_UploadData | null>(null)
  const [queryString, setQueryString] = React.useState<string>("")
  const [currentPage, setCurrentPage] = React.useState(1)
  const [initialLoading, setInitialLoading] = React.useState(true)

  // Selection state
  const [selectedFiles, setSelectedFiles] = React.useState<Set<string>>(new Set())
  const [selectionMode, setSelectionMode] = React.useState(false)

  // Album modal state
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [albums, setAlbums] = React.useState<AlbumData[]>([])
  const [selectedAlbum, setSelectedAlbum] = React.useState<string>("")
  const [albumsLoading, setAlbumsLoading] = React.useState(false)

  const toast = useToast()

  React.useEffect(() => {
    onSearch(1, "")
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
      .finally(() => setInitialLoading(false))
  }, [])

  const searchCallback = React.useCallback((form: any) => {
    setQueryString(form.query)
    onSearch(1, form.query)
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
  }, [onSearch])

  React.useEffect(() => {
    onSearch(currentPage, queryString)
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
  }, [currentPage, onSearch, queryString])

  const deleteFile = React.useCallback((fileId: string) => {
    onDelete(fileId)
      .then(() => onSearch(1, queryString))
      .then(setSearchResult)
      .catch(() => setSearchResult(null))
  }, [onDelete, onSearch, queryString])

  // Selection handlers
  const toggleFileSelection = React.useCallback((fileId: string) => {
    setSelectedFiles(prev => {
      const newSet = new Set(prev)
      if (newSet.has(fileId)) {
        newSet.delete(fileId)
      } else {
        newSet.add(fileId)
      }
      return newSet
    })
  }, [])

  const selectAllFiles = React.useCallback(() => {
    if (!searchResult) return
    setSelectedFiles(new Set(searchResult.items.map(file => file.id)))
  }, [searchResult])

  const clearSelection = React.useCallback(() => {
    setSelectedFiles(new Set())
    setSelectionMode(false)
  }, [])

  const toggleSelectionMode = React.useCallback(() => {
    setSelectionMode(prev => !prev)
    if (selectionMode) {
      setSelectedFiles(new Set())
    }
  }, [selectionMode])

  // Album modal handlers
  const openAlbumModal = React.useCallback(async () => {
    if (!onGetAlbums) return

    setAlbumsLoading(true)
    try {
      const albumList = await onGetAlbums()
      setAlbums(albumList)
      onOpen()
    } catch (error) {
      toast({
        title: "Error",
        description: "Failed to load albums",
        status: "error",
        duration: 3000,
        isClosable: true
      })
    } finally {
      setAlbumsLoading(false)
    }
  }, [onGetAlbums, onOpen, toast])

  const handleAddToAlbum = React.useCallback(() => {
    if (!onAddFilesToAlbum || !selectedAlbum || selectedFiles.size === 0) return

    onAddFilesToAlbum(selectedAlbum, Array.from(selectedFiles))
      .then(() => {
        // Clear selection and close modal
        setSelectedFiles(new Set())
        setSelectionMode(false)
        setSelectedAlbum("")
        onClose()
      })
  }, [onAddFilesToAlbum, selectedAlbum, selectedFiles, toast, onClose])

  const { register, handleSubmit } = useForm()

  const hasSelectedFiles = selectedFiles.size > 0

  return (
    <Box>
      {/* Search and Controls */}
      <VStack spacing={4} align="stretch">
        <form onSubmit={handleSubmit(searchCallback)}>
          <InputGroup>
            <InputLeftElement
              color="gray.500"
              children={<Icon as={SearchIcon} />}
            />
            <Input
              variant="filled"
              {...register("query")}
              placeholder="Search for files"
            />
          </InputGroup>
        </form>

        {/* Selection Controls */}
        <HStack spacing={3} justify="space-between" flexWrap="wrap">
          <HStack spacing={3}>
            <Button
              size="sm"
              variant={selectionMode ? "solid" : "outline"}
              colorScheme={selectionMode ? "primary" : "gray"}
              onClick={toggleSelectionMode}
            >
              {selectionMode ? "Cancel Selection" : "Select Files"}
            </Button>

            {selectionMode && (
              <>
                <Button size="sm" variant="ghost" onClick={selectAllFiles}>
                  Select All
                </Button>
                <Button size="sm" variant="ghost" onClick={clearSelection}>
                  Clear
                </Button>
              </>
            )}
          </HStack>

          {hasSelectedFiles && (
            <HStack spacing={3}>
              <Badge colorScheme="primary" fontSize="sm" px={3} py={1} rounded="full">
                {selectedFiles.size} selected
              </Badge>
              <Button
                size="sm"
                colorScheme="primary"
                leftIcon={<Icon as={FolderPlusIcon} w={4} h={4} />}
                onClick={openAlbumModal}
                isLoading={albumsLoading}
              >
                Add to Album
              </Button>
            </HStack>
          )}
        </HStack>
      </VStack>

      {/* File Results */}
      {initialLoading ? (
        <Flex justify="center" align="center" mt={6}>
          <Spinner size="lg" />
        </Flex>
      ) : searchResult === null ? (
        <Box color="gray.500" textAlign="center">
          <Heading size="xl" mt={6} mb={2}>
            :(
          </Heading>
          <Heading as="h2" size="lg">
            No files found
          </Heading>
          <Text>There were no files matched your query</Text>
        </Box>
      ) : (
        <Flex justifyContent="center" gap="20px" wrap="wrap" mt={6}>
          {searchResult.items.map((file: UploadData) => (
            <Box key={file.id} position="relative">
              {selectionMode && (
                <Checkbox
                  position="absolute"
                  top={2}
                  left={2}
                  zIndex={2}
                  isChecked={selectedFiles.has(file.id)}
                  onChange={() => toggleFileSelection(file.id)}
                  size="lg"
                  colorScheme="primary"
                />
              )}
              <FileCard
                file={file}
                onDetails={(file: UploadData) => onFileDetails(file.id)}
                onDelete={(file: UploadData) => deleteFile(file.id)}
                isSelectable={selectionMode}
                isSelected={selectedFiles.has(file.id)}
                onToggleSelection={() => toggleFileSelection(file.id)}
              />
            </Box>
          ))}
        </Flex>
      )}

      {/* Pagination */}
      {(searchResult?.pages || 0) > 1 && (
        <Flex justifyContent="center" mt={5}>
          <Pagination
            pages={searchResult?.pages || 1}
            currentPage={currentPage}
            range={3}
            onPageSelect={setCurrentPage}
          />
        </Flex>
      )}

      {/* Add to Album Modal */}
      <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Add Files to Album</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <VStack spacing={4} align="stretch">
              <Text fontSize="sm" color="gray.600">
                Adding {selectedFiles.size} file{selectedFiles.size === 1 ? '' : 's'} to an album
              </Text>

              <Select
                placeholder="Select an album"
                value={selectedAlbum}
                onChange={(e) => setSelectedAlbum(e.target.value)}
              >
                {albums.map((album) => (
                  <option key={album.id} value={album.id}>
                    {album.name} {album.public ? "(Public)" : "(Private)"}
                  </option>
                ))}
              </Select>

              {albums.length === 0 && (
                <Text fontSize="sm" color="gray.500" textAlign="center">
                  No albums found. Create an album first to add files to it.
                </Text>
              )}
            </VStack>
          </ModalBody>

          <ModalFooter>
            <Button variant="ghost" mr={3} onClick={onClose}>
              Cancel
            </Button>
            <Button
              colorScheme="primary"
              onClick={handleAddToAlbum}
              isDisabled={!selectedAlbum || albums.length === 0}
            >
              Add to Album
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </Box>
  )
}