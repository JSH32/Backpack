/* istanbul ignore file */
/* tslint:disable */

export type RegistrationKeyData = {
    id: string;
    /**
     * Registration key.
     */
    code: string;
    /**
     * Admin which issued this registration key.
     */
    issUser: string;
    /**
     * Amount of uses left.
     */
    usesLeft: number;
    /**
     * Key invalidation date.
     */
    expiryDate?: string;
};

