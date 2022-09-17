/* istanbul ignore file */
/* tslint:disable */
import type { MessageResponse } from '../models/MessageResponse';
import type { RegistrationKeyData } from '../models/RegistrationKeyData';
import type { RegistrationKeyPage } from '../models/RegistrationKeyPage';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class AdminService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Create a registration key
     * - Minimum required role: `admin`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param uses Maximum amount of key uses.
     * @param expiration Expiration in milliseconds from creation date.
     * @returns RegistrationKeyData
     * @throws ApiError
     */
    public create(
        uses?: number,
        expiration?: number,
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
     * Get registration keys
     * - Minimum required role: `admin`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param pageNumber Page to get
     * @returns RegistrationKeyPage
     * @throws ApiError
     */
    public list(
        pageNumber: number,
    ): CancelablePromise<RegistrationKeyPage> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/admin/registrationKey/list/{page_number}',
            path: {
                'page_number': pageNumber,
            },
        });
    }

    /**
     * Get a single registration key
     * - Minimum required role: `admin`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param registrationId Registration key to get
     * @returns RegistrationKeyData
     * @throws ApiError
     */
    public getOne(
        registrationId: string,
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
     * Delete a registration key
     * - Minimum required role: `admin`
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param registrationId Registration key to delete
     * @returns MessageResponse Registration key was deleted
     * @throws ApiError
     */
    public delete(
        registrationId: string,
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
