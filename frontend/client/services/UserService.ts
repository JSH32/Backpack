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
     * Verify using a verification code.
     *
     * This will verify whatever user the code was created for.
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

    /**
     * Get private user information. This is not the same thing as a user profile.
     * - Allow unverified users: `true`
     * - Application token allowed: `true`
     *
     * @param userId
     * @returns UserData
     * @throws ApiError
     */
    public info(
        userId: string,
    ): CancelablePromise<UserData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user/{user_id}',
            path: {
                'user_id': userId,
            },
        });
    }

    /**
     * Delete a user and all files owned by the user
     * - Allow unverified users: `true`
     * - Application token allowed: `false`
     *
     * @param userId
     * @param requestBody Verify your password
     * @returns MessageResponse User was deleted
     * @throws ApiError
     */
    public delete(
        userId: string,
        requestBody: UserDeleteForm,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/user/{user_id}',
            path: {
                'user_id': userId,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Incorrect password`,
            },
        });
    }

    /**
     * Register account using a registration key.
     * This is only required on services with `invite_only` enabled.
     * Admins can register a user without a key.
     *
     * @param userId
     * @param key This doesn't have to be provided if an admin is calling this route.
     * @returns UserData
     * @throws ApiError
     */
    public registerKey(
        userId: string,
        key?: string,
    ): CancelablePromise<UserData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user/{user_id}/register',
            path: {
                'user_id': userId,
            },
            query: {
                'key': key,
            },
        });
    }

    /**
     * Change user settings
     * - Allow unverified users: `true`
     * - Application token allowed: `false`
     *
     * @param userId
     * @param requestBody
     * @returns UserData
     * @throws ApiError
     */
    public settings(
        userId: string,
        requestBody: UpdateUserSettings,
    ): CancelablePromise<UserData> {
        return this.httpRequest.request({
            method: 'PUT',
            url: '/api/user/{user_id}/settings',
            path: {
                'user_id': userId,
            },
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Resend a verification code to the email
     * - Allow unverified users: `true`
     * - Application token allowed: `false`
     *
     * This will be disabled if `smtp` is disabled in server settings
     *
     * @param userId
     * @returns MessageResponse
     * @throws ApiError
     */
    public resendVerify(
        userId: string,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'PATCH',
            url: '/api/user/{user_id}/verify/resend',
            path: {
                'user_id': userId,
            },
            errors: {
                409: `Already verified`,
                410: `SMTP is disabled`,
            },
        });
    }

}
