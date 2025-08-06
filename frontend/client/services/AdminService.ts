/* istanbul ignore file */
/* tslint:disable */
import type { MessageResponse } from '../models/MessageResponse';
import type { Page_RegistrationKeyData } from '../models/Page_RegistrationKeyData';
import type { RegistrationKeyData } from '../models/RegistrationKeyData';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class AdminService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * - Minimum required role: `admin`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     * @param uses Maximum amount of key uses.
     * @param expiration Expiration in milliseconds from creation date.
     * @returns RegistrationKeyData
     * @throws ApiError
     */
    public create(
        uses?: number | null,
        expiration?: number | null,
    ): CancelablePromise<RegistrationKeyData> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/admin/registrationKey',
            query: {
                'uses': uses,
                'expiration': expiration,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     * @param pageNumber Page to get
     * @returns Page_RegistrationKeyData
     * @throws ApiError
     */
    public list(
        pageNumber: number,
    ): CancelablePromise<Page_RegistrationKeyData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/admin/registrationKey/list/{page_number}',
            path: {
                'page_number': pageNumber,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     * @param registrationId Registration key to get
     * @returns RegistrationKeyData
     * @throws ApiError
     */
    public getOne(
        registrationId: number,
    ): CancelablePromise<RegistrationKeyData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/admin/registrationKey/{registration_id}',
            path: {
                'registration_id': registrationId,
            },
            errors: {
                404: `Registration key was not found`,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     * @param registrationId Registration key to delete
     * @returns MessageResponse Registration key was deleted
     * @throws ApiError
     */
    public delete(
        registrationId: number,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/admin/registrationKey/{registration_id}',
            path: {
                'registration_id': registrationId,
            },
            errors: {
                404: `Registration key was not found`,
            },
        });
    }

}
