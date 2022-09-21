/* istanbul ignore file */
/* tslint:disable */

/**
 * Enabled authorization methods.
 */
export type AuthMethods = {
    discord: boolean;
    github: boolean;
    google: boolean;
    /**
     * Password authentication.
     */
    password: boolean;
};

