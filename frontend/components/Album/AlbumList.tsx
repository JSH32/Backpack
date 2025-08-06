import * as React from "react"
import { Pagination } from "../Pagination"
import AlbumCard from "./AlbumCard"

import {
  Box,
  Flex,
  Heading,
  Spinner,
  Text
} from "@chakra-ui/react"
import { AlbumData, Page_AlbumData } from "@/client"
import { AlbumFormData } from "./AlbumModal"

export const AlbumList: React.FC<{
  onLoad: (page: number) => Promise<Page_AlbumData>
  onDelete: (albumId: string) => Promise<void>
  onAlbumOpen: (albumId: string) => void
  updateAlbum: (albumId: string, updates: AlbumFormData) => Promise<void>
}> = ({ onLoad, onDelete, onAlbumOpen, updateAlbum }) => {
  const [albumData, setAlbumData] = React.useState<Page_AlbumData | null>(null)
  const [currentPage, setCurrentPage] = React.useState(1)
  const [initialLoading, setInitialLoading] = React.useState(true)

  React.useEffect(() => {
    onLoad(1)
      .then(setAlbumData)
      .catch(() => setAlbumData(null))
      .finally(() => setInitialLoading(false))
  }, [])

  React.useEffect(() => {
    onLoad(currentPage)
      .then(setAlbumData)
      .catch(() => setAlbumData(null))
  }, [currentPage])

  const deleteAlbum = React.useCallback((albumId: string) => {
    onDelete(albumId)
      .then(() => onLoad(1))
      .then(setAlbumData)
      .catch(() => setAlbumData(null))
  }, [onDelete, onLoad, albumData])

  const onUpdate = React.useCallback((album: AlbumData, form: AlbumFormData) => {
    updateAlbum(album.id, form)
      .then(() => onLoad(1))
      .then(setAlbumData)
      .catch(() => setAlbumData(null))
  }, [onDelete, onLoad, albumData])

  return (
    <Box>
      {initialLoading ? (
        <Flex justify="center" align="center" mt={6}>
          <Spinner size="lg" />
        </Flex>
      ) : albumData === null ? (
        <Box color="gray.500" textAlign="center">
          <Heading size="xl" mt={6} mb={2}>
            :(
          </Heading>
          <Heading as="h2" size="lg">
            No albums found
          </Heading>
          <Text>There were no albums found</Text>
        </Box>
      ) : albumData.items.length === 0 ? (
        <Box color="gray.500" textAlign="center" py={12}>
          <Heading as="h3" size="md" mb={2}>
            No albums yet
          </Heading>
          <Text fontSize="sm">
            Create your first album to get started
          </Text>
        </Box>
      ) : (
        <Flex justifyContent="center" gap="20px" wrap="wrap" mt={6}>
          {albumData.items.map((album: AlbumData) => (
            <AlbumCard
              key={album.id}
              album={album}
              onOpen={(album: AlbumData) => onAlbumOpen(album.id)}
              onDelete={(album: AlbumData) => deleteAlbum(album.id)}
              onUpdate={onUpdate}
            />
          ))}
        </Flex>
      )}
      {(albumData?.pages || 0) > 1 ? (
        <Flex justifyContent="center" mt={5}>
          <Pagination
            pages={albumData?.pages || 1}
            currentPage={currentPage}
            range={3}
            onPageSelect={setCurrentPage}
          />
        </Flex>
      ) : (
        <></>
      )}
    </Box>
  )
}