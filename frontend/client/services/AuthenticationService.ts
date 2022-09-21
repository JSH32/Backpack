/* istanbul ignore file */
/* tslint:disable */
import type { AuthMethods } from '../models/AuthMethods';
import type { BasicAuthForm } from '../models/BasicAuthForm';
import type { OAuthRequest } from '../models/OAuthRequest';
import type { TokenResponse } from '../models/TokenResponse';
import type { UnlinkAuthMethod } from '../models/UnlinkAuthMethod';

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
     * Callback for Discord OAuth provider.
     * This redirects to frontend with token if a valid user was found with the parameters.
     *
     * @param requestBody
     * @returns void
     * @throws ApiError
     */
    public discordCallback(
        requestBody: OAuthRequest,
    ): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/discord/callback',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Discord OAuth2 authentication.
     * Redirects to Discord to authenticate the user.
     *
     * @returns void
     * @throws ApiError
     */
    public discordLogin(): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/discord/login',
        });
    }

    /**
     * Callback for Github OAuth provider.
     * This redirects to frontend with token if a valid user was found with the parameters.
     *
     * @param requestBody
     * @returns void
     * @throws ApiError
     */
    public githubCallback(
        requestBody: OAuthRequest,
    ): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/github/callback',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Github OAuth2 authentication.
     * Redirects to Github to authenticate the user.
     *
     * @returns void
     * @throws ApiError
     */
    public githubLogin(): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/github/login',
        });
    }

    /**
     * Callback for Google OAuth provider.
     * This redirects to frontend with token if a valid user was found with the parameters.
     *
     * @param requestBody
     * @returns void
     * @throws ApiError
     */
    public googleCallback(
        requestBody: OAuthRequest,
    ): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/google/callback',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Google OAuth2 authentication.
     * Redirects to Google to authenticate the user.
     *
     * @returns void
     * @throws ApiError
     */
    public googleLogin(): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/google/login',
        });
    }

    /**
     * Get all enabled auth methods for this user.
     *
     * @returns AuthMethods
     * @throws ApiError
     */
    public enabledMethods(): CancelablePromise<AuthMethods> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/methods',
        });
    }

    /**
     * Unlink an OAuth method from a user.
     *
     * @param requestBody
     * @returns AuthMethods
     * @throws ApiError
     */
    public unlinkMethod(
        requestBody: UnlinkAuthMethod,
    ): CancelablePromise<AuthMethods> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/auth/unlink',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Need at least one auth provider.`,
            },
        });
    }

}
