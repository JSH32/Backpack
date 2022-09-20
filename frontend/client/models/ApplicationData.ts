/* istanbul ignore file */
/* tslint:disable */

export type ApplicationData = {
    /**
     * Date of application creation
     */
    created: string;
    id: string;
    /**
     * Last time the application was used for a request
     */
    lastAccessed: string;
    name: string;
    /**
     * Only sent when the token is originally created
     */
    token?: string;
    /**
     * User ID who owns the application
     */
    userId: string;
};

