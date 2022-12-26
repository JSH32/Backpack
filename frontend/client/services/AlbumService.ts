/* istanbul ignore file */
/* tslint:disable */
import type { AlbumCreate } from '../models/AlbumCreate';
import type { AlbumData } from '../models/AlbumData';
import type { AlbumUpdate } from '../models/AlbumUpdate';
import type { ApplicationPage } from '../models/ApplicationPage';
import type { MessageResponse } from '../models/MessageResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class AlbumService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * Create an album
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param requestBody
     * @returns AlbumData
     * @throws ApiError
     */
    public create(
        requestBody: AlbumCreate,
    ): CancelablePromise<AlbumData> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/album',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Get album info.
     *
     * This wont work if you don't have access to the album and the album is privated.
     *
     * **For private albums:**
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param albumId
     * @returns AlbumData
     * @throws ApiError
     */
    public info(
        albumId: string,
    ): CancelablePromise<AlbumData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/album/{album_id}',
            path: {
                'album_id': albumId,
            },
        });
    }

    /**
     * Delete an album.
     *
     * - Allow unverified users: `false`
     * - Application token allowed: `false`
     *
     * @param albumId
     * @returns MessageResponse
     * @throws ApiError
     */
    public delete(
        albumId: string,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/album/{album_id}',
            path: {
                'album_id': albumId,
            },
        });
    }

    /**
     * Update album settings
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     *
     * @param albumId
     * @param requestBody
     * @returns AlbumData
     * @throws ApiError
     */
    public update(
        albumId: string,
        requestBody: AlbumUpdate,
    ): CancelablePromise<AlbumData> {
        return this.httpRequest.request({
            method: 'PATCH',
            url: '/api/album/{album_id}',
            path: {
                'album_id': albumId,
            },
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * Get all albums owned by a user.
     * - Allow unverified users: `true`
     * - Application token allowed: `true`
     *
     * @param pageNumber Page to get albums by (starts at 1)
     * @param userId
     * @returns ApplicationPage
     * @throws ApiError
     */
    public list(
        pageNumber: string,
        userId: string,
    ): CancelablePromise<ApplicationPage> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user/{user_id}/album/{page_number}',
            path: {
                'page_number': pageNumber,
                'user_id': userId,
            },
        });
    }

}
