/* istanbul ignore file */
/* tslint:disable */

import type { OAuthProviders } from './OAuthProviders';

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
     * Are registration keys enabled?
     */
    inviteOnly: boolean;
    oauthProviders: OAuthProviders;
    /**
     * Is SMTP (email verification) enabled on the server?
     */
    smtp: boolean;
    /**
     * Amount of files uploaded.
     */
    uploadedFiles: number;
};

