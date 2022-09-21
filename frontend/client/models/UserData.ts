/* istanbul ignore file */
/* tslint:disable */

import type { UserRole } from './UserRole';

export type UserData = {
    email: string;
    id: string;
    /**
     * Has the user already verified with a registration key?
     */
    registered: boolean;
    role: UserRole;
    username: string;
    verified: boolean;
};

