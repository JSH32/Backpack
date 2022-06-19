/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * Public server configuration
 */
export type AppInfo = {
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
    /**
     * Git tag or commit hash.
     */
    inviteOnly: boolean;
    /**
     * App name
     */
    appName: string;
    /**
     * App description
     */
    appDescription: string;
};

