/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type ApplicationData = {
    /**
     * Last time the application was used for a request
     */
    lastAccessed: string;
    id: string;
    /**
     * Only sent when the token is originally created
     */
    token?: string;
    name: string;
    /**
     * User ID who owns the application
     */
    userId: string;
};

