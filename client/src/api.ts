/**
 * Web client for backpack API, only to be used with frontend
 */

import axios, { AxiosResponse } from "axios"

const BASE_URL = import.meta.env.SNOWPACK_PUBLIC_API_URL

enum UserRole {
    User = "user",
    Admin = "admin"
}

export interface UserData {
    id: string,
    username: string,
    email: string,
    verified: boolean,
    role: UserRole
}

/**
 * Will log out of the service, httponly cookie will be deleted
 */
export const logout = async (): Promise<void> => {
    return await axios.post(`${BASE_URL}/auth/logout`)
}

/**
 * Log in with password authentication, key will be stored as httponly
 * 
 * @param email email
 * @param password password
 * @returns user data
 */
export const passwordLogin = async (auth: string, password: string): Promise<UserData> => {
    return (await axios.post<UserData>(`${BASE_URL}/auth/basic`, {
        auth: auth,
        password: password
    })).data
}

/**
 * Get data about the current user
 * 
 * @returns user data
 */
export const getUserData = async (): Promise<UserData> => {
    return (await axios.get<UserData>(`${BASE_URL}/user/info`)).data
}

/**
 * Create and log in to a new account
 * 
 * @param username username
 * @param email email
 * @param password password
 * @returns user data
 */
export const userCreate = async (username: string, email: string, password: string): Promise<UserData> => {
    return (await axios.post<UserData>(`${BASE_URL}/user/create`, {
        username: username,
        email: email,
        password: password
    })).data
}

export const verify = async (code: string): Promise<AxiosResponse<any>> => {
    return await axios.post(`${BASE_URL}/user/verify/${code}`)
}

export const resendCode = async (email: string): Promise<AxiosResponse<any>> => {
    return await axios.post(`${BASE_URL}/user/verify/resend`, {
        email: email
    })
}