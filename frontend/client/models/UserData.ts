/* istanbul ignore file */
/* tslint:disable */

import type { UserRole } from './UserRole';

export type UserData = {
    /**
     * This will not be present if accessed by another user.
     */
    email?: string;
    id: string;
    /**
     * Has the user already verified with a registration key?
     */
    registered?: boolean;
    role: UserRole;
    username: string;
    /**
     * This will not be present if accessed by another user.
     */
    verified?: boolean;
};

