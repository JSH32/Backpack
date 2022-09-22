/* istanbul ignore file */
/* tslint:disable */

import type { OAuthProvider } from './OAuthProvider';

/**
 * Unlink an OAuth method.
 */
export type UnlinkAuthMethod = {
    method: OAuthProvider;
    /**
     * Password required if present.
     */
    password?: string;
};

