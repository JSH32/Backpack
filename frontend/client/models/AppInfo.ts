/* istanbul ignore file */
/* tslint:disable */

/**
 * Public server configuration
 */
export type AppInfo = {
    /**
     * App description
     */
    appDescription: string;
    /**
     * App name
     */
    appName: string;
    /**
     * Theme color of the Backpack instance
     */
    color: string;
    /**
     * Git tag (version) or commit hash
     */
    gitVersion: string;
    /**
     * Git tag or commit hash.
     */
    inviteOnly: boolean;
    /**
     * Is SMTP (email verification) enabled on the server?
     */
    smtp: boolean;
};

