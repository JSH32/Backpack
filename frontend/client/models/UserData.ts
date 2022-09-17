/* istanbul ignore file */
/* tslint:disable */

import type { UserRole } from './UserRole';

export type UserData = {
    email: string;
    id: string;
    role: UserRole;
    username: string;
    verified: boolean;
};

