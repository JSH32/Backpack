/* istanbul ignore file */
/* tslint:disable */
import type { BasicAuthForm } from '../models/BasicAuthForm';
import type { TokenResponse } from '../models/TokenResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class AuthenticationService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Login with email and password
     *
     * @param requestBody
     * @returns TokenResponse
     * @throws ApiError
     */
    public basic(
        requestBody: BasicAuthForm,
    ): CancelablePromise<TokenResponse> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/auth/basic',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Invalid credentials`,
            },
        });
    }

}
