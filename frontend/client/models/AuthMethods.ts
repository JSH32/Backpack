/* istanbul ignore file */
/* tslint:disable */

/**
 * Enabled authorization methods.
 */
export type AuthMethods = {
    /**
     * Cached discord tag.
     */
    discord?: string | null;
    /**
     * Cached github username.
     */
    github?: string | null;
    /**
     * Google username (email before the @).
     */
    google?: string | null;
    /**
     * Is password authentication enabled.
     */
    password: boolean;
};

