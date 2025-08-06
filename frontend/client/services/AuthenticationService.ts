/* istanbul ignore file */
/* tslint:disable */
import type { AuthMethods } from '../models/AuthMethods';
import type { BasicAuthForm } from '../models/BasicAuthForm';
import type { LoginRedirectUrl } from '../models/LoginRedirectUrl';
import type { OAuthProvider } from '../models/OAuthProvider';
import type { TokenResponse } from '../models/TokenResponse';
import type { UnlinkAuthMethod } from '../models/UnlinkAuthMethod';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class AuthenticationService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
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
     * If token is provided, this will link to the existing account.
     * @param provider
     * @param includeToken
     * @param redirect
     * @returns LoginRedirectUrl
     * @throws ApiError
     */
    public oauthLogin(
        provider: OAuthProvider,
        includeToken: boolean,
        redirect?: string | null,
    ): CancelablePromise<LoginRedirectUrl> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/oauth',
            query: {
                'provider': provider,
                'redirect': redirect,
                'include_token': includeToken,
            },
        });
    }

    /**
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

    /**
     * This redirects to the redirect provided in the oauth initialization route.
     * @param provider Provider to callback to.
     * @param code
     * @param state
     * @returns void
     * @throws ApiError
     */
    public oauthCallback(
        provider: string,
        code: string,
        state: string,
    ): CancelablePromise<void> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/auth/{provider}/callback',
            path: {
                'provider': provider,
            },
            query: {
                'code': code,
                'state': state,
            },
        });
    }

}
