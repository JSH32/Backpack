/* istanbul ignore file */
/* tslint:disable */

/**
 * Enabled authorization methods.
 */
export type AuthMethods = {
    /**
     * Cached discord tag.
     */
    discord?: string;
    /**
     * Cached github username.
     */
    github?: string;
    /**
     * Google username (email before the @).
     */
    google?: string;
    /**
     * Is password authentication enabled.
     */
    password: boolean;
};

