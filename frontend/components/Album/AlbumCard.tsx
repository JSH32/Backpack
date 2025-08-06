import {
  Box,
  Button,
  Flex,
  Heading,
  Text,
  Tooltip,
  useColorModeValue,
  Icon,
  Badge,
  VStack,
  useDisclosure,
  HStack,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  IconButton
} from "@chakra-ui/react"
import { dateToString } from "helpers/util"
import * as React from "react"
import DeleteIcon from "assets/icons/trash.svg"
import EditIcon from "assets/icons/edit.svg"
import FolderIcon from "assets/icons/folder.svg"
import MoreIcon from "assets/icons/more-vertical.svg"
import { AlbumData } from "@/client"
import AlbumModal, { AlbumFormData } from "./AlbumModal"

const AlbumCard: React.FC<{
  album: AlbumData,
  onOpen: (album: AlbumData) => void,
  onDelete: (album: AlbumData) => void,
  onUpdate?: (album: AlbumData, updates: AlbumFormData) => void
}> = ({ album, onOpen, onDelete, onUpdate }) => {
  const { isOpen, onOpen: openModal, onClose: closeModal } = useDisclosure()

  const handleEdit = (e: React.MouseEvent) => {
    e.stopPropagation()
    openModal()
  }

  const handleDelete = (e: React.MouseEvent) => {
    e.stopPropagation()
    onDelete(album)
  }

  const handleUpdateSubmit = async (formData: AlbumFormData) => {
    if (!onUpdate) return

    try {
      await onUpdate(album, formData)
      closeModal()
    } catch (error) {
      throw error
    }
  }


  const cardBg = useColorModeValue("white", "gray.700")
  const borderColor = useColorModeValue("gray.200", "gray.600")
  const textColor = useColorModeValue("gray.800", "white")
  const subtextColor = useColorModeValue("gray.600", "gray.300")
  const mutedTextColor = useColorModeValue("gray.500", "gray.400")

  return (
    <>
      <Box
        bg={cardBg}
        rounded="lg"
        boxShadow="lg"
        position="relative"
        cursor="pointer"
        onClick={() => onOpen(album)}
        w={{ sm: "100%", md: "300px" }}
        minH="200px"
        p={5}
      >
        {/* Action Menu */}
        <Menu>
          <MenuButton
            as={IconButton}
            icon={<Icon as={MoreIcon} w={4} h={4} />}
            variant="ghost"
            size="sm"
            position="absolute"
            top={4}
            right={4}
            onClick={(e) => e.stopPropagation()}
          />
          <MenuList>
            {onUpdate && (
              <MenuItem icon={<Icon as={EditIcon} w={4} h={4} />} onClick={handleEdit}>
                Edit Album
              </MenuItem>
            )}
            <MenuItem
              icon={<Icon as={DeleteIcon} w={4} h={4} />}
              onClick={handleDelete}
              color="red.500"
            >
              Delete Album
            </MenuItem>
          </MenuList>
        </Menu>

        <VStack spacing={4} h="full" justify="space-between" align="stretch">
          {/* Header with icon and title */}
          <VStack spacing={3} align="center">
            <Box p={3}>
              <Icon
                as={FolderIcon}
                w={10}
                h={10}
                color="primary.300"
              />
            </Box>

            <VStack spacing={1} align="center">
              <Heading
                fontSize="lg"
                fontWeight="semibold"
                textAlign="center"
                noOfLines={2}
                color={textColor}
                lineHeight="1.3"
              >
                {album.name}
              </Heading>

              {album.description && (
                <Text
                  fontSize="sm"
                  color={subtextColor}
                  textAlign="center"
                  noOfLines={2}
                >
                  {album.description}
                </Text>
              )}
            </VStack>
          </VStack>

          {/* Footer */}
          <VStack spacing={2}>
            <HStack justify="space-between" w="full">
              <Text fontSize="xs" color={mutedTextColor}>
                {dateToString(new Date(album.created))}
              </Text>
              <Badge
                colorScheme={album.public ? "green" : "gray"}
                variant="subtle"
                fontSize="xs"
                px={2}
                py={1}
                rounded="md"
              >
                {album.public ? "Public" : "Private"}
              </Badge>
            </HStack>
          </VStack>
        </VStack>
      </Box>

      {onUpdate && (
        <AlbumModal
          isOpen={isOpen}
          onClose={closeModal}
          album={album}
          onSubmit={handleUpdateSubmit}
        />
      )}
    </>
  )
}

export default AlbumCard