import { UserData } from "@/client"
import { action, makeAutoObservable, observable } from "mobx"
import api from "helpers/api"

class Store {
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

export default new Store()