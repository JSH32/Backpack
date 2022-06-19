/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * Public server configuration
 */
export type AppInfo = {
    /**
     * Git tag or commit hash.
     */
    inviteOnly: boolean;
    /**
     * Git tag (version) or commit hash
     */
    gitVersion: string;
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
     * Is SMTP (email verification) enabled on the server?
     */
    smtp: boolean;
};

