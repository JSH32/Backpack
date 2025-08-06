import {
  Box,
  Button,
  Flex,
  Heading,
  HStack,
  Text,
  Tooltip,
  useColorModeValue,
  Icon,
  Image
} from "@chakra-ui/react"
import { dateToString, getExtension } from "helpers/util"
import * as React from "react"
import InfoIcon from "assets/icons/info.svg"
import DeleteIcon from "assets/icons/trash.svg"
import { UploadData } from "@/client"

const FileCard: React.FC<{
  file: UploadData,
  onDetails: (file: UploadData) => void,
  onDelete: (file: UploadData) => void,
  isSelectable?: boolean,
  isSelected?: boolean,
  onToggleSelection?: () => void
}> = ({ file, onDetails, onDelete, isSelectable = false, isSelected = false, onToggleSelection }) => {
  const ext = getExtension(file.name)

  const cardBg = useColorModeValue("white", "gray.700")
  const selectedBg = useColorModeValue("primary.50", "primary.900")
  const selectedBorder = useColorModeValue("primary.300", "primary.600")
  const defaultBorder = useColorModeValue("transparent", "transparent")

  const handleCardClick = React.useCallback(() => {
    if (isSelectable && onToggleSelection) {
      onToggleSelection()
    }
  }, [isSelectable, onToggleSelection])

  const handleDetailsClick = React.useCallback((e: React.MouseEvent) => {
    e.stopPropagation()
    onDetails(file)
  }, [file, onDetails])

  const handleDeleteClick = React.useCallback((e: React.MouseEvent) => {
    e.stopPropagation()
    onDelete(file)
  }, [file, onDelete])

  return (
    <Box
      bg={isSelected ? selectedBg : cardBg}
      rounded="lg"
      boxShadow="lg"
      position="relative"
      border="2px"
      borderColor={isSelected ? selectedBorder : defaultBorder}
      cursor={isSelectable ? "pointer" : "default"}
      onClick={handleCardClick}
      transition="all 0.2s ease"
      _hover={isSelectable ? {
        transform: "translateY(-1px)",
        boxShadow: "xl",
        borderColor: isSelected ? selectedBorder : "primary.200"
      } : {}}
    >
      {/* Selection overlay */}
      {isSelectable && (
        <Box
          position="absolute"
          top={0}
          left={0}
          right={0}
          bottom={0}
          bg={isSelected ? "primary.500" : "transparent"}
          opacity={isSelected ? 0.1 : 0}
          rounded="lg"
          transition="opacity 0.2s ease"
          pointerEvents="none"
          zIndex={1}
        />
      )}

      {file.thumbnailUrl ? (
        <Image
          w="200px"
          h="200px"
          objectFit="cover"
          src={file.thumbnailUrl}
          alt={`Picture of ${file.url}`}
          roundedTop="lg"
        />
      ) : (
        <Flex
          minW="200px"
          minH="200px"
          roundedTop="lg"
          bg={useColorModeValue("gray.200", "gray.600")}
          align="center"
          textAlign="center"
          justify="center"
        >
          <Heading noOfLines={1} w={180}>
            {ext !== "" ? ext : "FILE"}
          </Heading>
        </Flex>
      )}

      <Box p="4" position="relative" zIndex={2}>
        <Flex justifyContent="space-between" alignContent="center">
          <Box
            fontSize="1xl"
            fontWeight="semibold"
            as="h4"
            maxW="120px"
            lineHeight="tight"
            noOfLines={1}
          >
            {file.name}
          </Box>

          <HStack>
            <Tooltip label="File details">
              <Button
                onClick={handleDetailsClick}
                variant="ghost"
                size="s"
                _hover={{
                  bg: useColorModeValue("gray.100", "gray.600")
                }}
              >
                <Icon as={InfoIcon} />
              </Button>
            </Tooltip>
            <Tooltip label="Delete file">
              <Button
                onClick={handleDeleteClick}
                color="red.500"
                variant="ghost"
                size="s"
                _hover={{
                  bg: useColorModeValue("red.50", "red.900"),
                  color: "red.600"
                }}
              >
                <Icon as={DeleteIcon} />
              </Button>
            </Tooltip>
          </HStack>
        </Flex>

        <Text color="gray.500">
          {dateToString(new Date(file.uploaded))}
        </Text>
      </Box>
    </Box>
  )
}

export default FileCard