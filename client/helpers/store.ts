import { getUserData, UserData } from "helpers/api"
import { action, makeAutoObservable, observable } from "mobx"

class Store {
    @observable userData: UserData | undefined = undefined

    constructor() {
        makeAutoObservable(this)
        getUserData().then(this.setUserInfo)
    }

    @action setUserInfo = (value?: UserData) => {
        this.userData = value
    }
}

export default new Store()