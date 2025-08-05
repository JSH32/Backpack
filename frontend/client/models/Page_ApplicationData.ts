/* istanbul ignore file */
/* tslint:disable */

export type Page_ApplicationData = {
    items: Array<{
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
        token?: string | null;
        /**
         * User ID who owns the application
         */
        userId: string;
    }>;
    page: number;
    pages: number;
};

