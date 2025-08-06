/* istanbul ignore file */
/* tslint:disable */
import type { ApplicationCreate } from '../models/ApplicationCreate';
import type { ApplicationData } from '../models/ApplicationData';
import type { MessageResponse } from '../models/MessageResponse';
import type { Page_ApplicationData } from '../models/Page_ApplicationData';
import type { TokenResponse } from '../models/TokenResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class ApplicationService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
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
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
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
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
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
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
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

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     * @param pageNumber Page to get applications by (starts at 1)
     * @param userId
     * @returns Page_ApplicationData
     * @throws ApiError
     */
    public list(
        pageNumber: number,
        userId: string,
    ): CancelablePromise<Page_ApplicationData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user/{user_id}/application/{page_number}',
            path: {
                'page_number': pageNumber,
                'user_id': userId,
            },
        });
    }

}
