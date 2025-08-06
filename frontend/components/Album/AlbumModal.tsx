import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  Button,
  Input,
  FormControl,
  FormLabel,
  Checkbox
} from "@chakra-ui/react"
import * as React from "react"
import { useForm } from "react-hook-form"

export interface AlbumFormData {
  name: string
  description: string
  public: boolean
}

interface AlbumModalProps {
  isOpen: boolean
  onClose: () => void
  album?: any // For edit mode
  onSubmit: (data: AlbumFormData) => Promise<void>
}

const AlbumModal: React.FC<AlbumModalProps> = ({
  isOpen,
  onClose,
  album,
  onSubmit
}) => {
  const albumForm = useForm()

  React.useEffect(() => {
    if (album) {
      albumForm.reset({
        name: album.name,
        description: album.description || '',
        public: album.public
      })
    } else {
      albumForm.reset({
        name: '',
        description: '',
        public: false
      })
    }
  }, [album, albumForm])

  const handleSubmit = React.useCallback(async (data: any) => {
    await onSubmit(data)
    onClose()
    albumForm.reset()
  }, [albumForm, isOpen, onClose])

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <form onSubmit={albumForm.handleSubmit(handleSubmit)}>
          <ModalHeader>{!album ? 'Create Album' : 'Edit Album'}</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <FormControl>
              <FormLabel>Name</FormLabel>
              <Input
                placeholder="Album Name"
                {...albumForm.register("name", { required: true })}
              />
            </FormControl>
            <FormControl mt={3}>
              <FormLabel>Description</FormLabel>
              <Input
                placeholder="Album Description"
                {...albumForm.register("description")}
              />
            </FormControl>
            <FormControl mt={3}>
              <Checkbox {...albumForm.register("public")}>
                Public
              </Checkbox>
            </FormControl>
          </ModalBody>
          <ModalFooter>
            <Button variant='ghost' mr={3} onClick={onClose}>
              Cancel
            </Button>
            <Button colorScheme="primary" type="submit">
              {!album ? 'Create' : 'Save'}
            </Button>
          </ModalFooter>
        </form>
      </ModalContent>
    </Modal>
  )
}

export default AlbumModal