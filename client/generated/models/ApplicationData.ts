/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type ApplicationData = {
    name: string;
    id: string;
    readonly userId: string;
    /**
     * Last time the application was used for a request
     */
    readonly lastAccessed: number;
    /**
     * Only sent when the token is originally created
     */
    readonly token?: string;
};

