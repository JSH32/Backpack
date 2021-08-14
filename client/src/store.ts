import { getUserData, UserData } from "api"
import { action, makeAutoObservable, observable } from "mobx"

class Store {
    @observable userData: UserData = undefined

    constructor() {
        makeAutoObservable(this)
        getUserData().then(v => this.userData = v)
    }

    @action setAppInfo = (value: UserData) => {
        this.userData = value
    }
}

export default new Store()