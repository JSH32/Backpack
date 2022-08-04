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
     * Git tag or commit hash.
     */
    inviteOnly: boolean;
    /**
     * App name
     */
    appName: string;
    /**
     * Is SMTP (email verification) enabled on the server?
     */
    smtp: boolean;
    /**
     * Git tag (version) or commit hash
     */
    gitVersion: string;
    /**
     * Theme color of the Backpack instance
     */
    color: string;
};

