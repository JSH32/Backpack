/* istanbul ignore file */
/* tslint:disable */
import type { AuthRequest } from '../models/AuthRequest';
import type { BasicAuthForm } from '../models/BasicAuthForm';
import type { TokenResponse } from '../models/TokenResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class AuthenticationService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Login with email and password.
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

    /**
     * Github OAuth redirect URL.
     * This redirects to frontend with token if a valid user was found with the parameters.
     *
     * @param requestBody
     * @returns TokenResponse
     * @throws ApiError
     */
    public githubAuth(
        requestBody: AuthRequest,
    ): CancelablePromise<TokenResponse> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/github/auth',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Initiate Github OAuth authentication.
     * This redirects to github to authenticate the user.
     *
     * @returns TokenResponse
     * @throws ApiError
     */
    public githubLogin(): CancelablePromise<TokenResponse> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/github/login',
        });
    }

    /**
     * Google OAuth redirect URL.
     * This redirects to frontend with token if a valid user was found with the parameters.
     *
     * @param requestBody
     * @returns TokenResponse
     * @throws ApiError
     */
    public googleAuth(
        requestBody: AuthRequest,
    ): CancelablePromise<TokenResponse> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/google/auth',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Initiate Google OAuth authentication.
     * This redirects to google to authenticate the user.
     *
     * @returns TokenResponse
     * @throws ApiError
     */
    public googleLogin(): CancelablePromise<TokenResponse> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/google/login',
        });
    }

}
