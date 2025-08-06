/* istanbul ignore file */
/* tslint:disable */

import type { UserRole } from './UserRole';

export type UserData = {
    /**
     * This will not be present if accessed by another user.
     */
    email?: string | null;
    id: string;
    /**
     * Has the user already verified with a registration key?
     * This will be true always if service is in `invite_only` mode.
     * This will not be present if accessed by another user.
     */
    registered?: boolean | null;
    role: UserRole;
    username: string;
    /**
     * This will not be present if accessed by another user.
     */
    verified?: boolean | null;
};

