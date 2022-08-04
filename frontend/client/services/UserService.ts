/* istanbul ignore file */
/* tslint:disable */
import type { MessageResponse } from '../models/MessageResponse';
import type { UpdateUserSettings } from '../models/UpdateUserSettings';
import type { UserCreateForm } from '../models/UserCreateForm';
import type { UserData } from '../models/UserData';
import type { UserDeleteForm } from '../models/UserDeleteForm';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class UserService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Get current user information
     * - Minimum required role: `user`
     * - Allow unverified users: `true`
     * - Application token allowed: `true`
     *
     * @returns UserData
     * @throws ApiError
     */
    public info(): CancelablePromise<UserData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user',
        });
    }

    /**
     * Create a new user
     *
     * @param requestBody
     * @returns UserData
     * @throws ApiError
     */
    public create(
        requestBody: UserCreateForm,
    ): CancelablePromise<UserData> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/user',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Delete a user and all files owned by the user
     * - Minimum required role: `user`
     * - Allow unverified users: `true`
     * - Application token allowed: `false`
     *
     * @param requestBody Verify your password
     * @returns MessageResponse User was deleted
     * @throws ApiError
     */
    public delete(
        requestBody: UserDeleteForm,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/user',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Incorrect password`,
            },
        });
    }

    /**
     * Change user settings
     * - Minimum required role: `user`
     * - Allow unverified users: `true`
     * - Application token allowed: `false`
     *
     * @param requestBody
     * @returns UserData
     * @throws ApiError
     */
    public settings(
        requestBody: UpdateUserSettings,
    ): CancelablePromise<UserData> {
        return this.httpRequest.request({
            method: 'PUT',
            url: '/api/user/settings',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Resend a verification code to the email
     * - Minimum required role: `user`
     * - Allow unverified users: `true`
     * - Application token allowed: `false`
     *
     * This will be disabled if `smtp` is disabled in server settings
     *
     * @returns MessageResponse
     * @throws ApiError
     */
    public resendVerify(): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'PATCH',
            url: '/api/user/verify/resend',
            errors: {
                409: `Already verified`,
                410: `SMTP is disabled`,
            },
        });
    }

    /**
     * Verify using a verification code
     *
     * This will be disabled if `smtp` is disabled in server settings
     *
     * @param code Verification code to verify
     * @returns MessageResponse
     * @throws ApiError
     */
    public verify(
        code: string,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'PATCH',
            url: '/api/user/verify/{code}',
            path: {
                'code': code,
            },
            errors: {
                400: `Invalid verification code`,
                410: `SMTP is disabled`,
            },
        });
    }

}
