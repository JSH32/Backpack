import { action, makeAutoObservable, observable } from "mobx"
import { AppInfo, getAppInfo } from "api"

class Store {
    @observable appInfo: AppInfo = undefined

    constructor() {
        makeAutoObservable(this)
        getAppInfo().then(v => this.appInfo = v)
    }

    @action setAppInfo = (value: AppInfo) => {
        this.appInfo = value
    }
}

export default new Store()