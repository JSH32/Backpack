/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type ApplicationData = {
    name: string;
    /**
     * Last time the application was used for a request
     */
    lastAccessed: string;
    /**
     * Only sent when the token is originally created
     */
    token?: string;
    id: string;
    /**
     * User ID who owns the application
     */
    userId: string;
};

