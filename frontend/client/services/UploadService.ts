/* istanbul ignore file */
/* tslint:disable */
import type { BatchDeleteRequest } from '../models/BatchDeleteRequest';
import type { BatchDeleteResponse } from '../models/BatchDeleteResponse';
import type { MessageResponse } from '../models/MessageResponse';
import type { Page_UploadData } from '../models/Page_UploadData';
import type { UploadData } from '../models/UploadData';
import type { UploadFile } from '../models/UploadFile';
import type { UploadStats } from '../models/UploadStats';

import type { CancelablePromise } from '../core/CancelablePromise';
import type { BaseHttpRequest } from '../core/BaseHttpRequest';

export class UploadService {

    constructor(public readonly httpRequest: BaseHttpRequest) {}

    /**
     * You can only upload a file for yourself regardless of admin status.
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param formData
     * @returns UploadData
     * @throws ApiError
     */
    public upload(
        formData: UploadFile,
    ): CancelablePromise<UploadData> {
        return this.httpRequest.request({
            method: 'POST',
            url: '/api/upload',
            formData: formData,
            mediaType: 'multipart/form-data',
            errors: {
                409: `File already uploaded`,
                413: `File too large`,
            },
        });
    }

    /**
     * This will ignore any invalid IDs.
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param requestBody IDs to delete.
     * @returns BatchDeleteResponse Information about the batch operation result.
     * @throws ApiError
     */
    public deleteFiles(
        requestBody: BatchDeleteRequest,
    ): CancelablePromise<BatchDeleteResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/upload/batch',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param uploadId Upload ID
     * @returns UploadData
     * @throws ApiError
     */
    public info(
        uploadId: number,
    ): CancelablePromise<UploadData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/upload/{upload_id}',
            path: {
                'upload_id': uploadId,
            },
            errors: {
                403: `Access denied`,
                404: `File not found`,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param uploadId File ID
     * @returns MessageResponse File deleted
     * @throws ApiError
     */
    public deleteFile(
        uploadId: string,
    ): CancelablePromise<MessageResponse> {
        return this.httpRequest.request({
            method: 'DELETE',
            url: '/api/upload/{upload_id}',
            path: {
                'upload_id': uploadId,
            },
            errors: {
                403: `Access denied`,
                404: `File not found`,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param uploadId Upload ID to set status
     * @param _public Public or private.
     * If public can be read by anyone.
     * If private can be read by uploader and admins.
     * @returns UploadData
     * @throws ApiError
     */
    public setPublic(
        uploadId: string,
        _public: boolean,
    ): CancelablePromise<UploadData> {
        return this.httpRequest.request({
            method: 'PATCH',
            url: '/api/upload/{upload_id}',
            path: {
                'upload_id': uploadId,
            },
            query: {
                'public': _public,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param pageNumber Page to get files by (starts at 1)
     * @param userId
     * @param query Query by name of file.
     * @param albumId For non admins, this must be a public album
     * or a private album owned by you.
     * @param _public If accessing another user as a non admin, this must be `true`
     * @returns Page_UploadData
     * @throws ApiError
     */
    public list(
        pageNumber: number,
        userId: string,
        query?: string | null,
        albumId?: string | null,
        _public?: boolean | null,
    ): CancelablePromise<Page_UploadData> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user/{user_id}/upload/list/{page_number}',
            path: {
                'page_number': pageNumber,
                'user_id': userId,
            },
            query: {
                'query': query,
                'album_id': albumId,
                'public': _public,
            },
            errors: {
                400: `Invalid page number`,
                404: `Page not found`,
            },
        });
    }

    /**
     * - Allow unverified users: `false`
     * - Application token allowed: `true`
     * @param userId
     * @returns UploadStats
     * @throws ApiError
     */
    public stats(
        userId: string,
    ): CancelablePromise<UploadStats> {
        return this.httpRequest.request({
            method: 'GET',
            url: '/api/user/{user_id}/upload/stats',
            path: {
                'user_id': userId,
            },
        });
    }

}
