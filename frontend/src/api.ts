import axios from "axios"

const BASE_URL = "/api/v1"

enum UserRole {
    User = "user",
    Admin = "admin"
}

interface UserData {
    username: string,
    email: string,
    verified: boolean,
    role: UserRole
}

export const passwordLogin = async (email: string, password: string): Promise<UserData> => {
    try {
        const res = await axios.post<UserData>(`${BASE_URL}/auth/basic`, {
            email: email,
            password: password
        })
        return res.data
    } catch (error) {
        throw new Error(error.response.data.message)
    }
}

export const getUserData = async (): Promise<UserData> => {
    try {
        const res = await axios.get<UserData>(`${BASE_URL}/user/info`)
        return res.data
    } catch (error) {
        throw new Error(error.response.data.message)
    }
}