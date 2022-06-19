/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ApplicationCreate } from '../models/ApplicationCreate';
import type { ApplicationData } from '../models/ApplicationData';
import type { MessageResponse } from '../models/MessageResponse';
import type { TokenResponse } from '../models/TokenResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class ApplicationService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Get all applications
     * Get all applications
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @returns ApplicationData
     * @throws ApiError
     */
    public list(): CancelablePromise<Array<ApplicationData>> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/application',
        });
    }

    /**
     * Create an application
     * Create an application
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param requestBody
     * @returns ApplicationData
     * @throws ApiError
     */
    public create(
        requestBody: ApplicationCreate,
    ): CancelablePromise<ApplicationData> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/application',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Token limit reached or invalid name`,
            },
        });
    }

    /**
     * Get token info
     * Get token info
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param applicationId
     * @returns ApplicationData
     * @throws ApiError
     */
    public info(
        applicationId: string,
    ): CancelablePromise<ApplicationData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/application/{application_id}',
            path: {
                'application_id': applicationId,
            },
        });
    }

    /**
     * Delete an application
     * Delete an application
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param applicationId
     * @returns MessageResponse Application was deleted
     * @throws ApiError
     */
    public delete(
        applicationId: string,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/application/{application_id}',
            path: {
                'application_id': applicationId,
            },
            errors: {
                401: `Unauthorized or token does not exist`,
            },
        });
    }

    /**
     * Get token by application ID
     * Get token by application ID
     * - Minimum required role: `user`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param applicationId Application ID to get token for
     * @returns TokenResponse
     * @throws ApiError
     */
    public token(
        applicationId: string,
    ): CancelablePromise<TokenResponse> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/application/{application_id}/token',
            path: {
                'application_id': applicationId,
            },
            errors: {
                404: `Application not found`,
            },
        });
    }

}
