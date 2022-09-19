import { UserData } from "@/client"
import { action, makeAutoObservable, observable } from "mobx"
import api from "helpers/api"
import { enableStaticRendering } from "mobx-react-lite"
import React from "react"

enableStaticRendering(typeof window === "undefined")

export class Store {
    @observable userData: UserData | undefined = undefined

    constructor() {
        makeAutoObservable(this)
        api.user.info()
            .then(this.setUserInfo)
            .catch(() => this.setUserInfo(undefined))
    }

    @action setUserInfo = (value?: UserData) => {
        this.userData = value
    }
}

export const StoreContext = React.createContext<Store | null>(null)
export const useStore = (): Store | null => React.useContext(StoreContext)
