/* istanbul ignore file */
/* tslint:disable */

export type ApplicationData = {
    /**
     * User ID who owns the application
     */
    userId: string;
    id: string;
    /**
     * Last time the application was used for a request
     */
    lastAccessed: string;
    /**
     * Only sent when the token is originally created
     */
    token?: string;
    name: string;
};

