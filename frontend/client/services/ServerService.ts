/* istanbul ignore file */
/* tslint:disable */
import type { AppInfo } from '../models/AppInfo';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class ServerService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Get public server configuration
     *
     * @returns AppInfo
     * @throws ApiError
     */
    public info(): CancelablePromise<AppInfo> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/info',
        });
    }

}
