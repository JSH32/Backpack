import React from "react"
import type { NextPage } from "next"

import {
    Box,
    Stack,
    Button,
    FormControl,
    FormLabel,
    Input,
    useToast,
    InputGroup,
    InputRightElement
} from "@chakra-ui/react"

import { useForm } from "react-hook-form"

import {
    PasswordTab,
    SettingsLayout
} from "layouts/SettingsLayout"

import { 
    ViewIcon, 
    ViewOffIcon 
} from "@chakra-ui/icons"
import api from "helpers/api"

const ViewButton: React.FC<{
    active: boolean,
    onToggle: (active: boolean) => void
}> = ({ active, onToggle }) => {
    return <Button
        variant="ghost"
        onClick={() => onToggle(!active)}>
        {active ? <ViewIcon /> : <ViewOffIcon />}
    </Button>
}

const Password: NextPage = () => {
    const { register, handleSubmit, reset } = useForm()
    const [loading, setLoading] = React.useState(false)
    const toast = useToast()

    const [viewStatus, setViewStatus] = React.useState<any>({
        current: false,
        new: false,
        confirmNew: false
    })

    const setViewButtonValue = React.useCallback((field: string, status: boolean) => {
        setViewStatus({ ...viewStatus, [field]: status })
    }, [viewStatus])

    const onSubmit = React.useCallback((form: any) => {
        if (form.newPassword !== form.confirmNewPassword) {
            toast({
                title: "Passwords do not match",
                status: "error",
                duration: 5000,
                isClosable: true
            })

            return
        }

        setLoading(true)
        api.user.settings({ 
            newPassword: form.newPassword, 
            currentPassword: form.currentPassword 
        })
            .then(() => {
                toast({
                    title: "Success",
                    description: "Password changed successfully",
                    status: "success",
                    duration: 5000,
                    isClosable: true
                })

                reset()
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
            .finally(() => setLoading(false))
    }, [loading])

    return <SettingsLayout tab={PasswordTab}>
        <form onSubmit={handleSubmit(onSubmit)}>
            <Stack spacing={4}>
                <FormControl isRequired>
                    <FormLabel>Current Password</FormLabel>
                    <InputGroup>
                        <Input
                            {...register("currentPassword", { required: "Current Password is required" })}
                            id="currentPassword"
                            type={viewStatus.current ? "text" : "password"} />
                        <InputRightElement h="full">
                            <ViewButton
                                active={viewStatus.current}
                                onToggle={active => setViewButtonValue("current", active)} />
                        </InputRightElement>
                    </InputGroup>
                </FormControl>
                <FormControl isRequired>
                    <FormLabel>New Password</FormLabel>
                    <InputGroup>
                        <Input
                            {...register("newPassword", { required: "New Password is required" })}
                            id="newPassword"
                            type={viewStatus.new ? "text" : "password"} />
                        <InputRightElement h="full">
                            <ViewButton
                                active={viewStatus.new}
                                onToggle={active => setViewButtonValue("new", active)} />
                        </InputRightElement>
                    </InputGroup>
                </FormControl>
                <FormControl isRequired>
                    <FormLabel>Confirm New Password</FormLabel>
                    <InputGroup>
                        <Input
                            {...register("confirmNewPassword", { required: "New Password confirmation is required" })}
                            id="confirmNewPassword"
                            type={viewStatus.confirmNew ? "text" : "password"} />
                        <InputRightElement h="full">
                            <ViewButton
                                active={viewStatus.confirmNew}
                                onToggle={active => setViewButtonValue("confirmNew", active)} />
                        </InputRightElement>
                    </InputGroup>
                </FormControl>
                <Box textAlign="right">
                    <Button alignSelf="right" colorScheme="primary" type="submit" isLoading={loading}>Change Password</Button>
                </Box>
            </Stack>
        </form>
    </SettingsLayout>
}

export default Password
